//! # ai-domain-harvester
//!
//! A concurrent data pipeline for extracting `.ai` domain sales from forum threads.
//!
//! ## Pipeline Overview
//!
//! The system operates as a two-stage pipeline:
//!
//! ### 1. Thread Discovery (Index Pages)
//!
//! For each index page:
//!
//! 1. Fetch the index page HTML
//! 2. Parse the document
//! 3. Extract thread identifiers (slugs)
//!
//! ### 2. Thread Processing (Fan-out)
//!
//! For each discovered thread:
//!
//! 1. Fetch the thread HTML
//! 2. Parse the document
//! 3. Extract and normalize domain sales
//!
//! ## Notes
//!
//! - The pipeline is designed to run concurrently using async streams
//! - HTML parsing is separated from network fetching for testability

use thiserror::Error;

mod scraping;

#[derive(Error, Debug)]
pub enum AiDomainHarvesterError {
    #[error(transparent)]
    Request(#[from] reqwest::Error),

    #[error("failed to parse HTML")]
    Parse,

    #[error("failed to extract data")]
    Extract,
}

pub struct Sale {
    domain: String,
    price_usd: u64,
    date: String,
    venue: Option<String>,
}

pub async fn run() -> Result<(), AiDomainHarvesterError> {
    Ok(())
}
