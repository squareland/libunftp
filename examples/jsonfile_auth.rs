use libunftp::auth::jsonfile_auth;
use log::*;

use std::io::Error;

use std::sync::Arc;

pub fn main() -> Result<(), Error> {
    pretty_env_logger::init();

    let authenticator = jsonfile_auth::JsonFileAuthenticator::new(String::from("credentials.json"))?;

    let addr = "127.0.0.1:2121";
    let server = libunftp::Server::with_root(std::env::temp_dir()).authenticator(Arc::new(authenticator));

    info!("Starting ftp server on {}", addr);
    let mut runtime = tokio02::runtime::Builder::new().build().unwrap();
    runtime.block_on(server.listener(addr));

    Ok(())
}