use std::process::Stdio;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
};

pub async fn setup_server() -> anyhow::Result<(String, Child)> {
    let mut child = Command::new("essential-rest-server")
        .env(
            "RUST_LOG",
            "[run_loop]=trace,[check_predicates]=trace,[check_predicate_constraints]=trace,[check]=trace,[constraint]=trace,[recover_secp256k1]=trace",
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
