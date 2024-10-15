// #![deny(missing_docs)]

use essential_types::{
    contract::Contract, convert::bytes_from_word, predicate::Predicate, Block, ContentAddress, Key,
    Value, Word,
};
use reqwest::{Client, ClientBuilder, Response};
use std::ops::Range;

/// Client library for sending requests to the Essential node.
pub struct EssentialNodeClient {
    /// Async reqwest client to make requests with.
    client: Client,
    /// The url to make requests to.
    url: reqwest::Url,
}

impl EssentialNodeClient {
    /// Create a new client with the given address.
    pub fn new(addr: String) -> anyhow::Result<Self> {
        let client = ClientBuilder::new().http2_prior_knowledge().build()?;
        let url = reqwest::Url::parse(&addr)?;
        Ok(Self { client, url })
    }

    /// Get contract at content address.
    pub async fn get_contract(&self, contract_ca: &ContentAddress) -> anyhow::Result<Contract> {
        let url = self.url.join(&format!("/get-contract/{contract_ca}"))?;
        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Contract>().await?)
    }

    /// Get predicate at content address.
    pub async fn get_predicate(&self, predicate_ca: &ContentAddress) -> anyhow::Result<Predicate> {
        let url = self.url.join(&format!("/get-predicate/{predicate_ca}"))?;
        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Predicate>().await?)
    }

    /// List blocks in the given L2 block number range.
    pub async fn list_blocks(&self, range: Range<Word>) -> anyhow::Result<Vec<Block>> {
        let url = self.url.join(&format!(
            "/list-blocks?start={}&end={}",
            range.start, range.end
        ))?;
        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Vec<Block>>().await?)
    }

    /// List contracts in the given L2 block number range.
    pub async fn list_contracts(
        &self,
        range: Range<Word>,
    ) -> anyhow::Result<Vec<(Word, Vec<Contract>)>> {
        let url = self.url.join(&format!(
            "/list-contracts?start={}&end={}",
            range.start, range.end
        ))?;
        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Vec<(Word, Vec<Contract>)>>().await?)
    }

    /// Query state in the given contract address and key.
    pub async fn query_state(
        &self,
        contract_ca: ContentAddress,
        key: Key,
    ) -> anyhow::Result<Option<Value>> {
        let key_bytes: Vec<_> = key.iter().copied().flat_map(bytes_from_word).collect();
        let key = hex::encode(&key_bytes);
        let url = self
            .url
            .join(&format!("/query-state/{contract_ca}/{key}"))?;
        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Option<Value>>().await?)
    }
}

async fn handle_error(response: Response) -> anyhow::Result<Response> {
    let status = response.status();
    if status.is_success() {
        Ok(response)
    } else {
        let text = response.text().await?;
        Err(anyhow::anyhow!("{}: {}", status, text))
    }
}
