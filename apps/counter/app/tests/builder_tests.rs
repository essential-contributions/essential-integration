use essential_app_utils::compile::compile_pint_project;

use regex::Regex;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command as TokioCommand};
use tokio::time::{sleep, Duration};

const PINT_DIRECTORY: &str = "../pint";

#[tokio::test]
async fn builder_integration() {
    let (_builder_process, node_address, builder_address) = start_essential_builder().await;

    let _ = compile_pint_project(concat!(env!("CARGO_MANIFEST_DIR"), "/../pint").into())
        .await
        .unwrap();

    sleep(Duration::from_secs(1)).await;

    deploy_contract(builder_address.clone()).await;

    let count = read_count(node_address.clone(), PINT_DIRECTORY).await;
    assert_eq!(count, 0);

    let returned_count = increment_count(
        builder_address.clone(),
        node_address.clone(),
        PINT_DIRECTORY,
    )
    .await;
    assert_eq!(returned_count, 1);

    let new_count = read_count(node_address.clone(), PINT_DIRECTORY).await;
    assert_eq!(new_count, returned_count);

    let count = new_count;
    let _ = increment_count(
        builder_address.clone(),
        node_address.clone(),
        PINT_DIRECTORY,
    )
    .await;

    let new_count = read_count(node_address.clone(), PINT_DIRECTORY).await;

    assert_eq!(new_count, count + 1);
}

async fn read_count(node_address: String, pint_directory: &str) -> u32 {
    let read_output = TokioCommand::new("cargo")
        .args(&[
            "run",
            "--",
            "read-count",
            &format!("http://{}", node_address.as_str()),
            pint_directory,
        ])
        .output()
        .await
        .expect("Failed to execute command");

    assert!(read_output.status.success(), "Command failed to run");

    let stdout_str = String::from_utf8(read_output.stdout).expect("Failed to parse stdout");

    let _stderr_str = String::from_utf8(read_output.stderr).expect("Failed to parse stderr");

    let count = stdout_str
        .split_whitespace()
        .last()
        .expect("Failed to parse count")
        .parse::<u32>()
        .expect("Failed to parse count");
    count
}

async fn increment_count(
    builder_address: String,
    node_address: String,
    pint_directory: &str,
) -> u32 {
    let increment_output = TokioCommand::new("cargo")
        .args(&[
            "run",
            "--",
            "increment-count",
            &format!("http://{}", builder_address.as_str()),
            &format!("http://{}", node_address.as_str()),
            pint_directory,
        ])
        .output()
        .await
        .expect("Failed to execute command");

    // Read stdout
    let stdout_str = String::from_utf8(increment_output.stdout).expect("Failed to parse stdout");

    let _stderr_str = String::from_utf8(increment_output.stderr).expect("Failed to parse stderr");

    // Regular expression to capture the new number
    let regx_new_count = Regex::new(r"Incremented count to: (\d+)").unwrap();
    let mut new_count = 0;

    // Check if stdout contains the desired line and capture the number
    if let Some(captures) = regx_new_count.captures(&stdout_str) {
        new_count = captures[1].parse::<u32>().expect("Failed to parse count");
    }
    new_count
}

async fn start_essential_builder() -> (Child, String, String) {
    let mut builder_process = TokioCommand::new("essential-builder")
        .arg("--block-interval-ms")
        .arg("100")
        // @todo can we remove ?
        .arg("--state-derivation")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .expect("Failed to start essential-builder");

    // Ensure the process is running
    let stdout = builder_process
        .stdout
        .take()
        .expect("Failed to capture stdout");
    let mut reader = BufReader::new(stdout).lines();

    // Regular expression to capture the address
    let regx_node = Regex::new(r"Starting node API server at (.+)").unwrap();
    let regx_builder = Regex::new(r"Starting builder API server at (.+)").unwrap();
    let mut node_address = String::new();
    let mut builder_address = String::new();

    // Wait for the specific line in the builder output
    while let Some(line) = reader.next_line().await.unwrap() {
        println!("Builder output: {}", line);
        if let Some(captures) = regx_node.captures(&line) {
            node_address = captures[1].to_string();
        }
        if let Some(captures) = regx_builder.captures(&line) {
            builder_address = captures[1].to_string();
        }
        if line.contains("Running the block builder") {
            break;
        }
    }

    (builder_process, node_address, builder_address)
}

async fn deploy_contract(builder_address: String) {
    let deploy_output = TokioCommand::new("essential-rest-client")
        .args(&[
            "deploy-contract",
            &format!("http://{}", builder_address.as_str()),
            concat!(env!("CARGO_MANIFEST_DIR"), "/../pint/out/debug/pint.json").into(),
        ])
        .output()
        .await
        .expect("Failed to execute command");

    assert!(deploy_output.status.success(), "Command failed to run");
}
