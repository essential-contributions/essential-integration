use anyhow::{anyhow, bail};
use clap::{builder::styling::Style, Parser};
use essential_node_types::BigBang;
use essential_rest_client::{builder_client::EssentialBuilderClient, contract_from_path};
use essential_types::{contract::Contract, ContentAddress};
use pint_pkg::build::BuiltPkg;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "deploy", version, about, long_about = None)]
#[group(skip)]
/// Tool to deploy a contract to a Essential builder endpoint.
struct Args {
    // `pint deploy` builds too, so accepts all args that `pint build` does.
    #[command(flatten)]
    build_args: pint_cli::build::Args,
    /// The builder to which the contract deployment solution will be submitted.
    #[arg(long)]
    builder_address: String,
    /// Path to a specific `Contract` encoded as JSON.
    ///
    /// If a contract is specified in this manner, all arguments related to building a
    /// pint project are ignored.
    #[arg(long)]
    contract: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Err(err) = run(args).await {
        let bold = Style::new().bold();
        eprintln!("{}Error:{} {err:?}", bold.render(), bold.render_reset());
        std::process::exit(1);
    }
}

async fn run(args: Args) -> anyhow::Result<()> {
    let Args {
        build_args,
        builder_address,
        contract,
    } = args;

    // The expected configuration of the chain we're querying.
    // FIXME: Provide CLI arg for specifying a path to a yml like the node and builder.
    let big_bang = BigBang::default();

    let builder_client = EssentialBuilderClient::new(builder_address)?;

    // If a contract was specified directly, there's no need to do the build or inspect any of the
    // `build_args` - we can deploy this directly.
    if let Some(contract_path) = contract {
        let (contract, programs) = contract_from_path(&contract_path).await?;
        let name = format!(
            "{}",
            contract_path
                .canonicalize()
                .unwrap_or_else(|_| contract_path.to_path_buf())
                .display()
        );
        print_deploying(&name, &contract);
        let output = builder_client
            .deploy_contract(&big_bang, &contract, &programs)
            .await?;
        print_received(&output);
        return Ok(());
    }

    // Otherwise, we should find and build the project.
    let (plan, built_pkgs) = pint_cli::build::cmd(build_args)?;

    // FIXME: We assume the package containing the output artifact is the last one in the
    // compilation order. When supporting workspaces, we should deploy all (non-deployed) member
    // "output" nodes in order of dependency.
    let &n = plan
        .compilation_order()
        .last()
        .ok_or_else(|| anyhow!("No built packages to deploy"))?;
    let built = &built_pkgs[&n];
    let pinned = &plan.graph()[n];
    let manifest = &plan.manifests()[&pinned.id()];

    // Now that the project is built, find the contract output.
    // FIXME: Right now assume the `debug` profile just like the `build` command does.
    let out_dir = manifest.out_dir();
    let profile = "debug";
    let profile_dir = out_dir.join(profile);
    match built {
        BuiltPkg::Library(_) => {
            bail!("Expected a contract to deploy, but the pint package is a library")
        }
        BuiltPkg::Contract(_built) => {
            let contract_path = profile_dir.join(&pinned.name).with_extension("json");
            let (contract, programs) = contract_from_path(&contract_path).await?;
            print_deploying(&pinned.name, &contract);
            let output = builder_client
                .deploy_contract(&big_bang, &contract, &programs)
                .await?;
            print_received(&output);
        }
    }

    Ok(())
}

/// Print the "Deploying ..." output with nice, aligned formatting.
fn print_deploying(name: &str, contract: &Contract) {
    let bold = Style::new().bold();
    println!(
        "   {}Deploying{} {} {}",
        bold.render(),
        bold.render_reset(),
        name,
        essential_hash::content_addr(contract),
    );
}

/// Print the "Received ..." output.
fn print_received(ca: &ContentAddress) {
    let bold = Style::new().bold();
    println!(
        "    {}Received{} solution address {}",
        bold.render(),
        bold.render_reset(),
        ca
    );
}
