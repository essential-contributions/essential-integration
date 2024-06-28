use essential_types::contract::Contract;
use std::{path::PathBuf, process::Stdio};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    process::{Child, Command},
};

pub async fn setup_server() -> anyhow::Result<(String, Child)> {
    let mut child = Command::new("essential-rest-server")
        .env(
            "RUST_LOG",
            "[run_loop]=trace,[check_predicate]=trace,[constraint]=trace,[recover_secp256k1]=trace",
        )
        .arg("--db")
        .arg("memory")
        .arg("0.0.0.0:0")
        .arg("--loop-freq")
        .arg("1")
        // this function returns `Child` so that it is not dropped prematurely despite this setting.
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

    let server_address = format!("http://localhost:{}", port);
    Ok((server_address, child))
}

pub async fn compile_pint_project(path: PathBuf) -> anyhow::Result<Contract> {
    assert!(path.exists(), "Path does not exist: {:?}", path);

    let output = Command::new("pint")
        .arg("build")
        .arg("--manifest-path")
        .arg(path.display().to_string())
        .output()
        .await?;

    assert!(
        output.status.success(),
        "pint failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let path = path.parent().unwrap();
    let file = tokio::fs::File::open(path.join("out/debug/pint.json")).await?;
    let mut bytes = Vec::new();
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut bytes).await?;

    let contract = serde_json::from_slice(&bytes)?;
    Ok(contract)
}
