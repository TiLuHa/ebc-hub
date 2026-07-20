use std::collections::HashMap;

use ebc_hub::config::Config;
use ebc_hub::db_access::Storage;
use ebc_hub::db_access::models::{BatteryIntake, Test, TestConfig};
use ebc_hub::ebc;
use ebc_hub::ebc::frame::OutboundFrame;
use ebc_hub::ebc_runner::{self, EbcRunner};

use color_eyre::eyre::{Result, eyre};
use ebc_hub::test_runner::runner::TestRunner;
use sqlx::SqlitePool;
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    dotenvy::dotenv()?;

    let pool = SqlitePool::connect("sqlite:data/ebc-hub.db").await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let text = std::fs::read_to_string("config/config.toml")?;
    let config: Config = toml::from_str(&text)?;

    let storage = Storage::connect("sqlite:data/ebc-hub.db").await?;

    let ebc = ebc::Device::new(&config.ebc.get("1").unwrap().port)?;
    let ebc_runner = ebc_runner::EbcRunner::new(ebc)?;
    let ebc_cmd_tx = ebc_runner.cmd_tx();
    let ebc_runner_thread = tokio::task::spawn(ebc_runner.run());

    let test_runner = TestRunner::new(storage, ebc_cmd_tx).await?;
    let test_cmd_tx = test_runner.cmd_tx();
    let test_runner_thread = tokio::task::spawn(test_runner.run());

    Ok(())
}
