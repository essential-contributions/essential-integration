use essential_server_types::{CheckSolution, CheckSolutionOutput, SolutionOutcome};
use essential_types::{
    contract::{Contract, SignedContract},
    convert::bytes_from_word,
    predicate::Predicate,
    solution::Solution,
    Block, ContentAddress, Hash, Key, PredicateAddress, Word,
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

    pub async fn deploy_contract(
        &self,
        predicates: SignedContract,
    ) -> anyhow::Result<ContentAddress> {
        let url = self.url.join("/deploy-contract")?;
        let response = self.client.post(url).json(&predicates).send().await?;
        if response.status().is_success() {
            Ok(response.json::<ContentAddress>().await?)
        } else {
            let text = response.text().await?;
            Err(anyhow::anyhow!("{}", text))
        }
        // Ok(response.json::<ContentAddress>().await?)
    }

    pub async fn check_solution(&self, solution: Solution) -> anyhow::Result<CheckSolutionOutput> {
        let url = self.url.join("/check-solution")?;
        let response = self.client.post(url).json(&solution).send().await?;
        Ok(response.json::<CheckSolutionOutput>().await?)
    }

    pub async fn check_solution_with_contracts(
        &self,
        solution: Solution,
        contracts: Vec<Contract>,
    ) -> anyhow::Result<CheckSolutionOutput> {
        let url = self.url.join("/check-solution-with-contracts")?;
        let input = CheckSolution {
            solution,
            contracts,
        };
        let response = self.client.post(url).json(&input).send().await?;
        Ok(response.json::<CheckSolutionOutput>().await?)
    }

    pub async fn submit_solution(&self, solution: Solution) -> anyhow::Result<ContentAddress> {
        let url = self.url.join("/submit-solution")?;
        let response = self.client.post(url).json(&solution).send().await?;
        // Ok(response.json::<essential_types::ContentAddress>().await?)
        if response.status().is_success() {
            Ok(response.json::<essential_types::ContentAddress>().await?)
        } else {
            let text = response.text().await?;
            Err(anyhow::anyhow!("{}", text))
        }
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

    pub async fn get_predicate(
        &self,
        address: &PredicateAddress,
    ) -> anyhow::Result<Option<Predicate>> {
        let url = self.url.join(&format!(
            "/get-predicate/{}/{}",
            address.contract, address.predicate,
        ))?;
        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Option<Predicate>>().await?)
    }

    pub async fn get_contract(
        &self,
        address: &ContentAddress,
    ) -> anyhow::Result<Option<SignedContract>> {
        let url = self.url.join(&format!("/get-contract/{address}"))?;
        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Option<SignedContract>>().await?)
    }

    pub async fn list_contracts(
        &self,
        time_range: Option<Range<Duration>>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<Vec<Predicate>>> {
        let mut url = self.url.join("/list-contracts")?;
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
        Ok(response.json::<Vec<Vec<Predicate>>>().await?)
    }

    pub async fn list_solutions_pool(&self, page: Option<u64>) -> anyhow::Result<Vec<Solution>> {
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
        page: Option<u64>,
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
            hex::encode_upper(
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
