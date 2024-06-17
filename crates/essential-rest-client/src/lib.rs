use base64::Engine as _;
use essential_server_types::{CheckSolution, CheckSolutionOutput, SolutionOutcome};
use essential_types::{
    convert::bytes_from_word,
    intent::{self, Intent},
    solution::Solution,
    Block, ContentAddress, Hash, IntentAddress, Key, Word,
};
use reqwest::{Client, ClientBuilder, Response};
use std::{ops::Range, time::Duration};

/// Client library for sending requests to the Essential REST Server.
pub struct EssentialClient {
    /// Async reqwest client to make requests with.
    client: Client,
    /// The url to make requests to.
    url: reqwest::Url,
}

impl EssentialClient {
    pub fn new(addr: String) -> anyhow::Result<Self> {
        let client = ClientBuilder::new().http2_prior_knowledge().build()?;
        let url = reqwest::Url::parse(&addr)?;
        Ok(Self { client, url })
    }

    pub async fn deploy_intent_set(
        &self,
        intents: intent::SignedSet,
    ) -> anyhow::Result<ContentAddress> {
        let url = self.url.join("/deploy-intent-set")?;
        let response = self.client.post(url).json(&intents).send().await?;
        Ok(response.json::<ContentAddress>().await?)
    }

    pub async fn check_solution(&self, solution: Solution) -> anyhow::Result<CheckSolutionOutput> {
        let url = self.url.join("/check-solution")?;
        let response = self.client.post(url).json(&solution).send().await?;
        Ok(response.json::<CheckSolutionOutput>().await?)
    }

    pub async fn check_solution_with_data(
        &self,
        solution: Solution,
        intents: Vec<Intent>,
    ) -> anyhow::Result<CheckSolutionOutput> {
        let url = self.url.join("/check-solution-with-data")?;
        let input = CheckSolution { solution, intents };
        let response = self.client.post(url).json(&input).send().await?;
        Ok(response.json::<CheckSolutionOutput>().await?)
    }

    pub async fn submit_solution(&self, solution: Solution) -> anyhow::Result<ContentAddress> {
        let url = self.url.join("/submit-solution")?;
        let response = self.client.post(url).json(&solution).send().await?;
        Ok(response.json::<essential_types::ContentAddress>().await?)
    }

    pub async fn solution_outcome(
        &self,
        solution_hash: &Hash,
    ) -> anyhow::Result<Vec<SolutionOutcome>> {
        let ca = ContentAddress(*solution_hash);
        let url = self.url.join(&format!("/solution-outcome/{ca}"))?;
        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Vec<SolutionOutcome>>().await?)
    }

    pub async fn get_intent(&self, address: &IntentAddress) -> anyhow::Result<Option<Intent>> {
        let url = self
            .url
            .join(&format!("/get-intent/{}/{}", address.set, address.intent,))?;
        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Option<Intent>>().await?)
    }

    pub async fn get_intent_set(
        &self,
        address: &ContentAddress,
    ) -> anyhow::Result<Option<intent::SignedSet>> {
        let url = self.url.join(&format!("/get-intent-set/{address}"))?;
        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Option<intent::SignedSet>>().await?)
    }

    pub async fn list_intent_sets(
        &self,
        time_range: Option<Range<Duration>>,
        page: Option<usize>,
    ) -> anyhow::Result<Vec<Vec<Intent>>> {
        let mut url = self.url.join("/list-intent-sets")?;
        if let Some(time_range) = time_range {
            url.query_pairs_mut()
                .append_pair("start", time_range.start.as_secs().to_string().as_str())
                .append_pair("end", time_range.end.as_secs().to_string().as_str());
        }
        if let Some(page) = page {
            url.query_pairs_mut()
                .append_pair("page", page.to_string().as_str());
        }

        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Vec<Vec<Intent>>>().await?)
    }

    pub async fn list_solutions_pool(&self, page: Option<usize>) -> anyhow::Result<Vec<Solution>> {
        let mut url = self.url.join("list-solutions-pool")?;
        if let Some(page) = page {
            url.query_pairs_mut()
                .append_pair("page", page.to_string().as_str());
        }
        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Vec<Solution>>().await?)
    }

    pub async fn list_winning_blocks(
        &self,
        time_range: Option<Range<Duration>>,
        page: Option<usize>,
    ) -> anyhow::Result<Vec<Block>> {
        let mut url = self.url.join("/list-winning-blocks")?;
        if let Some(time_range) = time_range {
            url.query_pairs_mut()
                .append_pair("start", time_range.start.as_secs().to_string().as_str())
                .append_pair("end", time_range.end.as_secs().to_string().as_str());
        }
        if let Some(page) = page {
            url.query_pairs_mut()
                .append_pair("page", page.to_string().as_str());
        }
        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Vec<Block>>().await?)
    }

    pub async fn query_state(
        &self,
        address: &ContentAddress,
        key: &Key,
    ) -> anyhow::Result<Vec<Word>> {
        let url = self.url.join(&format!(
            "/query-state/{address}/{}",
            essential_types::serde::hash::BASE64.encode(
                key.iter()
                    .flat_map(|w: &i64| bytes_from_word(*w))
                    .collect::<Vec<u8>>()
            ),
        ))?;
        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Vec<Word>>().await?)
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
