// #![deny(missing_docs)]

use essential_builder_types::SolutionFailure;
use essential_types::{solution::Solution, ContentAddress};
use reqwest::{Client, ClientBuilder, Response};

/// Client library for sending requests to the Essential builder.
pub struct EssentialBuilderClient {
    /// Async reqwest client to make requests with.
    client: Client,
    /// The url to make requests to.
    url: reqwest::Url,
}

impl EssentialBuilderClient {
    /// Create a new client with the given address.
    pub fn new(addr: String) -> anyhow::Result<Self> {
        let client = ClientBuilder::new().http2_prior_knowledge().build()?;
        let url = reqwest::Url::parse(&addr)?;
        Ok(Self { client, url })
    }

    /// Submit solution.
    ///
    /// Returns the content address of the submitted solution.
    pub async fn submit_solution(&self, solution: &Solution) -> anyhow::Result<ContentAddress> {
        let url = self.url.join("/submit-solution")?;
        let response = handle_error(self.client.post(url).json(solution).send().await?).await?;
        Ok(response.json::<ContentAddress>().await?)
    }

    /// For solution in the given content address, get the latest solution failures.
    ///
    /// The number of failures returned is limited by the `limit` parameter.
    /// The failures are ordered by block number and solution index in descending order.
    pub async fn latest_solution_failures(
        &self,
        solution_ca: &ContentAddress,
        limit: u32,
    ) -> anyhow::Result<Vec<SolutionFailure<'static>>> {
        let url = self
            .url
            .join(&format!("/latest_solution_failures/{solution_ca}/{limit}"))?;
        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Vec<SolutionFailure<'static>>>().await?)
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
