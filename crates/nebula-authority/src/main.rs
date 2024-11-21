use std::{path::PathBuf, sync::Arc, time::Duration};

use application::Application;
use clap::Parser;
use domain::authority::Authority;
use nebula_token::auth::jwks_discovery::{fetch_jwks, CachedRemoteJwksDiscovery, JwksDiscovery, StaticJwksDiscovery};

use crate::logger::LoggerConfig;

mod application;
mod config;
mod domain;
mod logger;
mod server;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
    /// Sets a port to start a authority server
    #[arg(short, long, value_name = "PORT")]
    pub port: Option<u16>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init_logger(LoggerConfig::default());
    let args = Args::parse();
    let app_config = config::load_config(args.config, args.port)?;
    let authority = Authority::new(&app_config)?;
    let jwks_discovery: Arc<dyn JwksDiscovery + Send + Sync> = if let Some(refresh_interval) =
        app_config.jwks_refresh_interval
    {
        Arc::new(
            CachedRemoteJwksDiscovery::new(app_config.jwks_url.clone(), Duration::from_secs(refresh_interval)).await?,
        )
    } else {
        let client = reqwest::Client::new();
        let jwks = fetch_jwks(&client, app_config.jwks_url.clone()).await?;
        Arc::new(StaticJwksDiscovery::new(jwks))
    };
    let application = Application::new(authority, jwks_discovery);

    server::run(application, app_config.into()).await?;
    Ok(())
}