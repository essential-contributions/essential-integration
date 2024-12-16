use clap::builder::styling::Style;
use essential_node_types::register_contract_solution;
use essential_rest_client::builder_client::EssentialBuilderClient;
use essential_types::{contract::Contract, solution::SolutionSet, ContentAddress};
use std::path::PathBuf;

pub async fn submit_solution(
    solutions_opt: Option<PathBuf>,
    builder_address: String,
    contract_opt: Option<&Contract>,
) -> anyhow::Result<ContentAddress> {
    // Used by both `pint-submit` and `pint-deploy`.
    // For the former, we receive a path to a solution,
    // whereas for the latter we receive a reference to a contract.
    let solution_set: SolutionSet = match (solutions_opt, contract_opt) {
        // handle `pint-submit`
        (Some(s), None) => serde_json::from_str::<SolutionSet>(&from_file(s).await?)?,
        // handle `pint-deploy`
        (None, Some(contract)) => {
            let registry_predicate = essential_node_types::BigBang::default().contract_registry;
            let solution = register_contract_solution(registry_predicate, contract)?;
            SolutionSet {
                solutions: vec![solution],
            }
        }
        (None, None) | (Some(_), Some(_)) => {
            anyhow::bail!("Either a solution or a contract must be provided, but not both.");
        }
    };

    let builder_client = EssentialBuilderClient::new(builder_address)?;
    let solution_ca = essential_hash::content_addr(&solution_set);
    print_submitting(&solution_ca);
    let output = builder_client.submit_solution(&solution_set).await?;
    if solution_ca != output {
        anyhow::bail!("The content address of the submitted solution differs from expected. May be a serialization error.");
    }
    print_submitted();
    Ok(output)
}

async fn from_file(path: PathBuf) -> anyhow::Result<String> {
    let content = tokio::fs::read_to_string(path).await?;
    Ok(content)
}

/// Print the "Submitting ..." output.
pub fn print_submitting(ca: &ContentAddress) {
    let bold = Style::new().bold();
    println!(
        "  {}Submitting{} solution {}",
        bold.render(),
        bold.render_reset(),
        ca,
    );
}

/// Print the "Submitted" output.
pub fn print_submitted() {
    let bold = Style::new().bold();
    println!(
        "   {}Submitted{} successfully",
        bold.render(),
        bold.render_reset(),
    );
}
