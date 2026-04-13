use anyhow::Context;
use clap::Parser;

mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::Config::parse();

    ai_domain_harvester::run()
        .await
        .context("failed to execute pipeline")?;

    Ok(())
}
