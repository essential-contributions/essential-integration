#![deny(missing_docs)]

//! Client libraries for interacting with the Essential builder and the Essential node.

/// Client library for sending requests to the Essential builder.
pub mod builder_client;

/// Client library for sending requests to the Essential node.
pub mod node_client;

/// Map `reqwest::Response` into `anyhow::Result`.
async fn handle_response(response: reqwest::Response) -> anyhow::Result<reqwest::Response> {
    let status = response.status();
    if status.is_success() {
        Ok(response)
    } else {
        let text = response.text().await?;
        Err(anyhow::anyhow!("{}: {}", status, text))
    }
}
