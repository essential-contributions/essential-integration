use clap::Parser;
use essential_rest_client::EssentialClient;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(default_value_t = String::from("http://0.0.0.0:0"))]
    address: String,
}

#[tokio::main]
async fn main() {
    let Cli { address } = Cli::parse();
    EssentialClient::new(address).await;
}
