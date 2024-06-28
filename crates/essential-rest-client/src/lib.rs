#![deny(missing_docs)]

//! # Essential REST Client
//!
//! This library provides a client for interacting with the Essential REST Server.
//!
//! ## Deploy Contract
//! Contracts must be signed before they can be deployed.
//! `essential-wallet` can be used to sign contracts.
//! Deploying the same contract multiple times is a no-op.
//!
//! ## Check Solution
//! This allows checking a lotion without it actually being included in a block.
//! The solution with use the pre-state that is currently on the server.
//! Any contracts that the solution solves must be deployed already.
//!
//! This is useful when you want to check a solution before submitting it
//! and want to use the state that is currently on the server.
//!
//! ## Check Solution With Contracts
//! This allows checking a solution with the set of contracts it is solving.
//! All contracts that the solution solves must be included in the set of contracts.
//! The solution will use the state that is currently on the server.
//!
//! This is useful when you are building contracts that aren't ready to be deployed
//! and you want to test them with a solution.
//!
//! ## Submit Solution
//! This allows submitting a solution to be included in an upcoming block.
//! Once a solution is submitted it is added to the pool.
//! The block builder runs on a regular loop interval and will include the solution in a block
//! in FIFO order if it satisfies the constraints.
//!
//! The block builder is likely to become more sophisticated in the future.
//!
//! Note that currently if you submit a solution that conflicts with another solution then
//! which ever solution is submitted first will be included in the block and the other solution
//! will fail. Failed solutions are not retried and will eventually be pruned.
//!
//! A solution can conflict with another solution when one solution is built on top of pre state
//! that the other solution changes. For example if a counter can only increment by 1 and is
//! currently set to 5 then you submit a solution setting it to 6 but another solution is submitted
//! before yours that sets the counter to 6 then your solution will fail to satisfy the constraints.
//! In fact in this example your solution will never satisfy again unless you update the state mutation
//! to the current count + 1. But to do this you have to resubmit your solution.
//!
//! Submitting the same solution twice (even by different user) is idempotent.
//!
//! ## Solution Outcome
//! This allows querying the outcome of a solution.
//! A solution is either successfully included in a block or it fails with a reason.
//!
//! One thing to keep in mind is solutions aren't necessarily unique.
//! It's possible for the same solution to be submitted multiple times.
//! For example if the counter example also allowed decrementing by 1 then
//! a solution could increment the count from 4 to 5 and another solution could decrement the count from 5 to 4.
//! Then a solution that increments the count from 4 to 5 could be submitted again.
//! These two solutions would have the exact same content address.
//! This results in the same solution hash returning multiple outcomes.
//!
//! This might make it difficult to know if it was the solution that you submitted that
//! was successful or failed. But actually it doesn't really matter because there is no
//! real ownership over a solution. Remember if two of the same solution are submitted
//! at the same time then it is as if only one was submitted.
//!
//! If you are interested in "has my solution worked" then it probably makes more
//! sense to query the state of the contract that you were trying to change.
//!
//! Keep in mind this is all very application specific.
//!
//! ## Get Predicate
//! This allows retrieving a deployed predicate.
//! It might be useful to do this if you want to debug a solution.
//!
//! ## Get Contract
//! This allows retrieving a deployed contract.
//! Very similar to `Get Predicate` but gets you the entire contract.
//!
//! ## List Contracts
//! This allows listing all deployed contracts.
//! The results are paged so you can only get a maximum number of contracts per query.
//! The contracts can also be filtered by the time range that they were deployed.
//!
//! ## List Solutions Pool
//! This allows listing all solutions currently in the pool.
//! The results are also paged.
//! Depending on the backlog of solutions an individual solution might not be in the pool for long.
//!
//! ## List Winning Blocks
//! This allows listing all blocks that have been successfully created.
//! The results are also paged.
//! The blocks can also be filtered by time.
//! Blocks are only created if there are solutions in the pool.
//! Blocks are created on a regular interval.
//!
//! ## Query State
//! This allows querying the state of a contract.
//! It is the main way the front end application will interact with state.
//! It only really makes sense to query state where you know what the abi of the contract is.
//! The state that's returned is a list of words of variable size.
//! The keys are also variable sized lists of words.
//! To make use of this api you need to know what type of contract you are querying.
//!
//! ## Query State Reads
//! This allows querying the state of a contract using state read programs.
//! This is a more advanced way of querying state.
//! It allows you to query the state of a contract using the state read programs from a predicate.
//! Custom state read programs can be also be written.
//! Pint can be used to create custom state reads.
//!
//! This api is also very useful if you are trying to solve a predicate but need to know what the pre-state
//! that the solution will read is.
//! For example if you want to run a debugger you will need this pre-state.
//!
//! The api can return which keys were read and which values were returned.
//! It can also return that values that were read into state slots on the pre-state read
//! and post-state read.
//!
//! Note that it doesn't return the keys and values that were read on the post-state read
//! because it is trivial to compute this locally using the state mutations in the solution.
use essential_server_types::{
    CheckSolution, CheckSolutionOutput, QueryStateReads, QueryStateReadsOutput, SolutionOutcome,
};
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
    /// Create a new client with the given address.
    pub fn new(addr: String) -> anyhow::Result<Self> {
        let client = ClientBuilder::new().http2_prior_knowledge().build()?;
        let url = reqwest::Url::parse(&addr)?;
        Ok(Self { client, url })
    }

    /// Deploy a signed contract to the server.
    pub async fn deploy_contract(
        &self,
        signed_contract: SignedContract,
    ) -> anyhow::Result<ContentAddress> {
        let url = self.url.join("/deploy-contract")?;
        let response =
            handle_error(self.client.post(url).json(&signed_contract).send().await?).await?;
        Ok(response.json::<ContentAddress>().await?)
    }

    /// Check a solution with the server.
    /// Contracts that this solves must be deployed.
    pub async fn check_solution(&self, solution: Solution) -> anyhow::Result<CheckSolutionOutput> {
        let url = self.url.join("/check-solution")?;
        let response = handle_error(self.client.post(url).json(&solution).send().await?).await?;
        Ok(response.json::<CheckSolutionOutput>().await?)
    }

    /// Check a solution with these contracts.
    /// This uses the state on the server.
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
        let response = handle_error(self.client.post(url).json(&input).send().await?).await?;
        Ok(response.json::<CheckSolutionOutput>().await?)
    }

    /// Submit a solution to be included in an upcoming block.
    pub async fn submit_solution(&self, solution: Solution) -> anyhow::Result<ContentAddress> {
        let url = self.url.join("/submit-solution")?;
        let response = handle_error(self.client.post(url).json(&solution).send().await?).await?;
        Ok(response.json::<essential_types::ContentAddress>().await?)
    }

    /// Get the outcome of a solution.
    ///
    /// Note that a solution can have multiple outcomes because the
    /// same solution can be submitted multiple times.
    pub async fn solution_outcome(
        &self,
        solution_hash: &Hash,
    ) -> anyhow::Result<Vec<SolutionOutcome>> {
        let ca = ContentAddress(*solution_hash);
        let url = self.url.join(&format!("/solution-outcome/{ca}"))?;
        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Vec<SolutionOutcome>>().await?)
    }

    /// Get a deployed predicate.
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

    /// Get a deployed contract.
    pub async fn get_contract(
        &self,
        address: &ContentAddress,
    ) -> anyhow::Result<Option<SignedContract>> {
        let url = self.url.join(&format!("/get-contract/{address}"))?;
        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Option<SignedContract>>().await?)
    }

    /// List deployed contracts.
    pub async fn list_contracts(
        &self,
        time_range: Option<Range<Duration>>,
        page: Option<u64>,
    ) -> anyhow::Result<Vec<Contract>> {
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
        Ok(response.json::<Vec<Contract>>().await?)
    }

    /// List solutions currently in the pool.
    pub async fn list_solutions_pool(&self, page: Option<u64>) -> anyhow::Result<Vec<Solution>> {
        let mut url = self.url.join("list-solutions-pool")?;
        if let Some(page) = page {
            url.query_pairs_mut()
                .append_pair("page", page.to_string().as_str());
        }
        let response = handle_error(self.client.get(url).send().await?).await?;
        Ok(response.json::<Vec<Solution>>().await?)
    }

    /// List blocks that have been successfully created.
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

    /// Query the state of a contract.
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

    /// Query the state of a contract using state read programs.
    pub async fn query_state_reads(
        &self,
        query: QueryStateReads,
    ) -> anyhow::Result<QueryStateReadsOutput> {
        let url = self.url.join("/query-state-reads")?;
        let response = handle_error(self.client.post(url).json(&query).send().await?).await?;
        Ok(response.json::<QueryStateReadsOutput>().await?)
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
