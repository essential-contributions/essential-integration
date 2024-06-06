use essential_rest_client::EssentialClient;
use essential_types::{intent::Intent, ContentAddress, IntentAddress};
use std::{collections::HashMap, path::PathBuf, process::Stdio};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    process::Command,
};

pub async fn compile(app_name: String) -> anyhow::Result<()> {
    let pint_dir_path = concat!(env!("CARGO_MANIFEST_DIR"), "/pint");
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

pub async fn setup_server() -> anyhow::Result<u16> {
    let mut child = Command::new("essential-rest-server")
        .env(
            "RUST_LOG",
            "[run_loop]=trace,[check_intent]=trace,[constraint]=trace",
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
    Ok(port)
}

pub async fn deploy_intents(
    server_address: String,
    signer_name: String,
    intents: Vec<Intent>,
    intent_addresses: HashMap<String, ContentAddress>,
) -> anyhow::Result<HashMap<String, IntentAddress>> {
    let client = EssentialClient::new(server_address.to_string())?;
    essential_wallet::new_key_pair(signer_name.clone(), essential_wallet::Scheme::Secp256k1).ok();
    let intents = essential_wallet::sign_intent_set(intents, &signer_name)?;
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
    Ok(addresses)
}

#[ignore]
#[tokio::test]
async fn setup() {
    let app_name = "token".to_string();
    let intent_names = [
        "mint".to_string(),
        "burn".to_string(),
        "transfer".to_string(),
    ];
    let deployer_name = "deployer".to_string();

    compile(app_name.clone()).await.unwrap();
    let (intents, intent_addresses) = read_output(app_name, &intent_names).await.unwrap();
    let server_port = setup_server().await.unwrap();
    let server_address = format!("http://localhost:{}", server_port);
    let _intent_addresses =
        deploy_intents(server_address, deployer_name, intents, intent_addresses)
            .await
            .unwrap();
}
