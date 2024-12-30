use std::path::PathBuf;

use clap::Parser;

pub mod application;
pub mod config;
pub mod database;
pub mod domain;
pub mod logger;
pub mod server;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
    /// Sets a port to start a backbone server
    #[arg(short, long, value_name = "FILE")]
    pub port: Option<u16>,
    /// Sets a database host
    #[arg(long)]
    pub database_host: Option<String>,
    /// Sets a database port
    #[arg(long)]
    pub database_port: Option<String>,
    /// Sets a database name
    #[arg(long)]
    pub database_name: Option<String>,
    /// Sets a database username
    #[arg(long)]
    pub database_username: Option<String>,
    /// Sets a database password
    #[arg(long)]
    pub database_password: Option<String>,
}

pub trait IntoAnyhow<T> {
    fn anyhow(self) -> anyhow::Result<T>;
}

impl<T, E> IntoAnyhow<T> for std::result::Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn anyhow(self) -> anyhow::Result<T> {
        self.map_err(|e| e.into())
    }
}
