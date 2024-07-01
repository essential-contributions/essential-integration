use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct ServerName {
    /// The address of the server to connect to.
    pub server: String,
    /// The name of the account to deploy the app with.
    pub account: String,
    /// The directory containing the pint files.
    pub pint_directory: PathBuf,
}
