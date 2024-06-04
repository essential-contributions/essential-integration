use base64::Engine as _;
use essential_server_types::{CheckSolutionOutput, SolutionOutcome};
use essential_types::{
    convert::bytes_from_word,
    intent::{self, Intent},
    solution::Solution,
    Block, ContentAddress, Hash, IntentAddress, Key, Word,
};
use reqwest::{Client, ClientBuilder};
use std::{ops::Range, time::Duration};

pub struct EssentialClient {
    client: Client,
    url: reqwest::Url,
}

impl EssentialClient {
    pub async fn new(addr: String) -> Self {
        let client = ClientBuilder::new()
            .http2_prior_knowledge()
            .build()
            .unwrap();
        let url = reqwest::Url::parse(&addr).unwrap();
        Self { client, url }
    }

    pub async fn deploy_intent_set(
        &self,
        intents: intent::SignedSet,
    ) -> anyhow::Result<ContentAddress> {
        let response = self
            .client
            .post(self.url.join("/deploy-intent-set").unwrap())
            .json(&intents)
            .send()
            .await
            .unwrap();
        Ok(response.json::<ContentAddress>().await?)
    }

    pub async fn check_solution(&self, solution: Solution) -> anyhow::Result<CheckSolutionOutput> {
        let response = self
            .client
            .post(self.url.join("/check-solution").unwrap())
            .json(&solution)
            .send()
            .await
            .unwrap();
        Ok(response.json::<CheckSolutionOutput>().await?)
    }

    pub async fn check_solution_with_data(
        &self,
        solution: Solution,
        intents: Vec<Intent>,
    ) -> anyhow::Result<CheckSolutionOutput> {
        #[derive(serde::Serialize)]
        struct CheckSolution {
            solution: Solution,
            intents: Vec<Intent>,
        }
        let input = CheckSolution { solution, intents };
        let response = self
            .client
            .post(self.url.join("/check-solution-with-data").unwrap())
            .json(&input)
            .send()
            .await
            .unwrap();
        Ok(response.json::<CheckSolutionOutput>().await?)
    }

    pub async fn submit_solution(&self, solution: Solution) -> anyhow::Result<ContentAddress> {
        let response = self
            .client
            .post(self.url.join("/submit-solution").unwrap())
            .json(&solution)
            .send()
            .await
            .unwrap();
        Ok(response.json::<essential_types::ContentAddress>().await?)
    }

    pub async fn solution_outcome(
        &self,
        solution_hash: &Hash,
    ) -> anyhow::Result<Vec<SolutionOutcome>> {
        let ca = ContentAddress(*solution_hash);
        let a = self.url.join(&format!("/solution-outcome/{ca}")).unwrap();
        let response = self.client.get(a).send().await.unwrap();
        Ok(response.json::<Vec<SolutionOutcome>>().await?)
    }

    pub async fn get_intent(&self, address: &IntentAddress) -> anyhow::Result<Option<Intent>> {
        let a = self
            .url
            .join(&format!("/get-intent/{}/{}", address.set, address.intent,))
            .unwrap();
        let response = self.client.get(a).send().await.unwrap();
        Ok(response.json::<Option<Intent>>().await?)
    }

    pub async fn get_intent_set(
        &self,
        address: &ContentAddress,
    ) -> anyhow::Result<Option<intent::SignedSet>> {
        let a = self
            .url
            .join(&format!("/get-intent-set/{address}"))
            .unwrap();
        let response = self.client.get(a).send().await.unwrap();
        Ok(response.json::<Option<intent::SignedSet>>().await?)
    }

    pub async fn list_intent_sets(
        &self,
        time_range: Option<Range<Duration>>,
        page: Option<usize>,
    ) -> anyhow::Result<Vec<Vec<Intent>>> {
        let mut a = self.url.join("/list-intent-sets").unwrap();
        if let Some(time_range) = time_range {
            a.query_pairs_mut()
                .append_pair("start", time_range.start.as_secs().to_string().as_str())
                .append_pair("end", time_range.end.as_secs().to_string().as_str());
        }
        if let Some(page) = page {
            a.query_pairs_mut()
                .append_pair("page", page.to_string().as_str());
        }

        let response = self.client.get(a).send().await.unwrap();
        Ok(response.json::<Vec<Vec<Intent>>>().await?)
    }

    pub async fn list_solutions_pool(&self, page: Option<usize>) -> anyhow::Result<Vec<Solution>> {
        let mut a = self.url.join("list-solutions-pool").unwrap();
        if let Some(page) = page {
            a.query_pairs_mut()
                .append_pair("page", page.to_string().as_str());
        }
        let response = self.client.get(a).send().await.unwrap();
        Ok(response.json::<Vec<Solution>>().await?)
    }

    pub async fn list_winning_blocks(
        &self,
        time_range: Option<Range<Duration>>,
        page: Option<usize>,
    ) -> anyhow::Result<Vec<Block>> {
        let mut a = self.url.join("/list-winning-blocks").unwrap();
        if let Some(time_range) = time_range {
            a.query_pairs_mut()
                .append_pair("start", time_range.start.as_secs().to_string().as_str())
                .append_pair("end", time_range.end.as_secs().to_string().as_str());
        }
        if let Some(page) = page {
            a.query_pairs_mut()
                .append_pair("page", page.to_string().as_str());
        }

        let response = self.client.get(a).send().await.unwrap();
        Ok(response.json::<Vec<Block>>().await?)
    }

    pub async fn query_state(
        &self,
        address: &ContentAddress,
        key: &Key,
    ) -> anyhow::Result<Vec<Word>> {
        let a = self
            .url
            .join(&format!(
                "/query-state/{address}/{}",
                essential_types::serde::hash::BASE64.encode(
                    key.iter()
                        .flat_map(|w: &i64| bytes_from_word(*w))
                        .collect::<Vec<u8>>()
                ),
            ))
            .unwrap();
        let response = self.client.get(a).send().await.unwrap();
        Ok(response.json::<Vec<Word>>().await?)
    }
}
