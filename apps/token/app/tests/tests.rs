// TODO: restructure

use essential_rest_client::EssentialClient;
use essential_types::{intent::Intent, ContentAddress, IntentAddress};
use std::{collections::HashMap, path::PathBuf, process::Stdio};
use token::token::{Addresses, Token};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    process::{Child, Command},
};

pub async fn compile(app_name: String) -> anyhow::Result<()> {
    let pint_dir_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../pint");
    let pint_path = PathBuf::from(pint_dir_path).join(format!("{}.pnt", app_name));
    anyhow::ensure!(pint_path.exists(), "failed to resolve pint dir path");
    let pint_target_path = PathBuf::from(pint_dir_path).join("target");
    if !pint_target_path.exists() {
        std::fs::create_dir(&pint_target_path)
            .map_err(|err| anyhow::anyhow!("failed to create pint target path: {}", err))?;
        std::fs::create_dir(pint_dir_path).ok();
    }
    let output_path = pint_target_path.join(format!("{}.json", app_name));
    let output = Command::new("pintc")
        .arg(pint_path.display().to_string())
        .arg("--output")
        .arg(output_path.display().to_string())
        .output()
        .await
        .map_err(|err| anyhow::anyhow!("failed to execute pint command: {}", err))?;
    anyhow::ensure!(
        output.status.success(),
        "pintc failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(())
}

pub async fn read_output(
    app_name: String,
    intent_names: &[String],
) -> anyhow::Result<(Vec<Intent>, HashMap<String, ContentAddress>)> {
    let pint_target_path =
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../pint")).join("target");
    let file = File::open(pint_target_path.join(format!("{}.json", app_name))).await?;
    let mut bytes = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut bytes).await?;
    let intents: Vec<Intent> =
        serde_json::from_slice(&bytes).expect("failed to deserialize intent set");
    let intent_addresses = intent_names
        .to_owned()
        .into_iter()
        .zip(intents.iter().map(essential_hash::content_addr))
        .collect::<HashMap<String, _>>();
    Ok((intents, intent_addresses))
}

pub async fn setup_server() -> anyhow::Result<(u16, Child)> {
    let mut child = Command::new("essential-rest-server")
        .env(
            "RUST_LOG",
            "[run_loop]=trace,[check_intent]=trace,[constraint]=trace,[recover_secp256k1]=trace",
        )
        .arg("--db")
        .arg("memory")
        .arg("0.0.0.0:0")
        .arg("--loop-freq")
        .arg("1")
        .kill_on_drop(true)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = child.stdout.take().unwrap();

    let buf = BufReader::new(stdout);
    let mut lines = buf.lines();

    let port;
    loop {
        if let Some(line) = lines.next_line().await.unwrap() {
            if line.contains("Listening") {
                port = line
                    .split(':')
                    .next_back()
                    .unwrap()
                    .trim()
                    .parse::<u16>()
                    .unwrap();
                break;
            }
        }
    }

    tokio::spawn(async move {
        loop {
            if let Some(line) = lines.next_line().await.unwrap() {
                println!("{}", line);
            }
        }
    });
    assert_ne!(port, 0);
    Ok((port, child))
}

pub async fn deploy_intents(
    server_address: String,
    wallet: &mut essential_wallet::Wallet,
    account_name: &str,
    intents: Vec<Intent>,
    intent_addresses: HashMap<String, ContentAddress>,
) -> anyhow::Result<Addresses> {
    let client = EssentialClient::new(server_address)?;
    let intents = wallet.sign_intent_set(intents, &account_name)?;
    let set_address = client.deploy_intent_set(intents).await?;
    let addresses = intent_addresses
        .into_iter()
        .map(|(n, i)| {
            (
                n,
                IntentAddress {
                    set: set_address.clone(),
                    intent: i,
                },
            )
        })
        .collect::<HashMap<_, _>>();
    Ok(Addresses {
        token: set_address,
        burn: addresses.get("burn").unwrap().clone(),
        init: addresses.get("init").unwrap().clone(),
        mint: addresses.get("mint").unwrap().clone(),
        transfer: addresses.get("transfer").unwrap().clone(),
    })
}

#[tokio::test]
async fn mint_and_transfer() {
    let app_name = "token".to_string();
    let intent_names = [
        "burn".to_string(),
        "init".to_string(),
        "mint".to_string(),
        "transfer".to_string(),
    ];
    let deployer_name = "deployer".to_string();
    let mut wallet = essential_wallet::Wallet::temp().unwrap();
    wallet
        .new_key_pair(&deployer_name, essential_wallet::Scheme::Secp256k1)
        .ok();
    compile(app_name.clone()).await.unwrap();
    let (intents, intent_addresses) = read_output(app_name, &intent_names).await.unwrap();
    let (port, _child) = setup_server().await.unwrap(); // receiving child so that it is not dropped until the test is over
    let server_address = format!("http://localhost:{}", port);
    let intent_addresses = deploy_intents(
        server_address.clone(),
        &mut wallet,
        &deployer_name,
        intents,
        intent_addresses,
    )
    .await
    .unwrap();

    let mut token = Token::new(server_address, intent_addresses, wallet).unwrap();

    // initialize token
    let _init_solution_address = token.init("art").await.unwrap();
    let mut name = token.name().await;
    while name.is_err() {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        name = token.name().await;
    }
    println!("token initialized");

    // alice mint 800 tokens
    let alice = "alice";
    token.create_account(&alice).unwrap();
    let first_mint_amount = 800;
    let _mint_solution_address = token.mint(&alice, first_mint_amount).await.unwrap();
    let mut balance = None;
    while balance == None {
        println!("{} balance {}", alice, 0);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        balance = token.balance(&alice).await.unwrap();
    }
    println!("{} balance {}", alice, balance.unwrap());
    assert_eq!(balance.unwrap(), first_mint_amount);

    // alice mint 200 tokens
    let second_mint_amount = 200;
    let mint_amount = first_mint_amount + second_mint_amount;
    let _mint_solution_address = token.mint(&alice, second_mint_amount).await.unwrap();
    let mut new_balance = balance;
    while new_balance == balance {
        println!("{} balance {}", alice, balance.unwrap());
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        new_balance = token.balance(&alice).await.unwrap();
    }
    println!("{} balance {}", alice, new_balance.unwrap());
    assert_eq!(new_balance.unwrap(), mint_amount);

    // alice transfer 500 tokens to bob
    let bob = "bob";
    token.create_account(&bob).unwrap();
    let transfer_amount = 500;
    let _transfer_solution_address = token.transfer(&alice, &bob, transfer_amount).await.unwrap();
    let mut alice_balance = balance;
    while alice_balance == balance {
        println!("{} balance {}", alice, new_balance.unwrap());
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        alice_balance = token.balance(&alice).await.unwrap();
    }
    println!("{} balance {}", alice, alice_balance.unwrap());
    let mut bob_balance = None;
    while bob_balance == None {
        println!("{} balance {}", bob, 0);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        bob_balance = token.balance(&bob).await.unwrap();
    }
    println!("{} balance {}", bob, bob_balance.unwrap());
    assert_eq!(alice_balance.unwrap(), mint_amount - transfer_amount);
    assert_eq!(bob_balance.unwrap(), transfer_amount);

    // alice burn 100 tokens
    let burn_amount = 100;
    let _burn_solution_address = token.burn(&alice, burn_amount).await.unwrap();
    let mut alice_new_balance = alice_balance;
    while alice_new_balance == alice_balance {
        println!("{} balance {}", alice, alice_balance.unwrap());
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        alice_new_balance = token.balance(&alice).await.unwrap();
    }
    println!("{} balance {}", alice, alice_new_balance.unwrap());
    assert_eq!(
        alice_new_balance.unwrap(),
        mint_amount - transfer_amount - burn_amount
    );
}
