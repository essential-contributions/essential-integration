use clap::builder::styling::Style;
use essential_node_types::register_contract_solution;
use essential_rest_client::builder_client::EssentialBuilderClient;
use essential_types::{contract::Contract, solution::SolutionSet, ContentAddress};

pub async fn submit_solution_set(
    solution_set: SolutionSet,
    builder_address: String,
) -> anyhow::Result<ContentAddress> {
    let builder_client = EssentialBuilderClient::new(builder_address)?;
    let solution_ca = essential_hash::content_addr(&solution_set);
    print_submitting(&solution_ca);
    let output = builder_client.submit_solution_set(&solution_set).await?;
    if solution_ca != output {
        anyhow::bail!("The content address of the submitted solution differs from expected. May be a serialization error.");
    }
    print_submitted();
    Ok(output)
}

pub async fn register_contract(
    builder_address: String,
    contract: &Contract,
) -> anyhow::Result<ContentAddress> {
    let registry_predicate = essential_node_types::BigBang::default().contract_registry;
    let solution = register_contract_solution(registry_predicate, contract)?;
    let solution_set = SolutionSet {
        solutions: vec![solution],
    };
    let output = submit_solution_set(solution_set, builder_address).await?;
    Ok(output)
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
