use crate::handle_response;
use essential_node_types::Block;
use essential_types::{convert::bytes_from_word, ContentAddress, Key, Value, Word};
use reqwest::{Client, ClientBuilder};
use std::ops::Range;

/// Client that binds to an Essential node address.
#[derive(Clone)]
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

    /// List blocks in the given L2 block number range.
    ///
    /// Blocks are only created if there are valid solutions.
    /// Blocks are created on a regular interval.
    pub async fn list_blocks(&self, range: Range<Word>) -> anyhow::Result<Vec<Block>> {
        let url = self.url.join(&format!(
            "/list-blocks?start={}&end={}",
            range.start, range.end
        ))?;
        let response = handle_response(self.client.get(url).send().await?).await?;
        Ok(response.json::<Vec<Block>>().await?)
    }

    /// Query state in the given contract address and key.
    ///
    /// This is the main way the front end application will interact with state.
    /// It only really makes sense to query state where you know what the ABI of the contract is.
    /// The state that's returned is a list of words of variable size.
    /// The keys are also variable sized lists of words.
    /// To make use of this API you need to know what type of contract you are querying.
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
        let response = handle_response(self.client.get(url).send().await?).await?;
        Ok(response.json::<Option<Value>>().await?)
    }
}
