//! # philentries
//!
//! A concurrent data pipeline for extracting and structuring philosophical encyclopedia entries
//! from authoritative sources such as the Stanford Encyclopedia of Philosophy (SEP), with support
//! for future sources like the Internet Encyclopedia of Philosophy (IEP).
//!
//! ## Pipeline Overview
//!
//! The system operates as a multi-stage concurrent ingestion pipeline:
//!
//! ### 1. Entry Discovery (Index Pages)
//!
//! For each index page:
//!
//! 1. Fetch the index page HTML
//! 2. Parse the document
//! 3. Extract entry links (articles)
//!
//! ### 2. Entry Processing (Fan-out)
//!
//! For each discovered entry:
//!
//! 1. Fetch the entry HTML
//! 2. Parse the document
//! 3. Extract structured philosophical content
//!
//! ## Output
//!
//! Each entry is normalized into a structured format containing:
//!
//! - Title
//! - Authors
//! - Content
//! - Bibliography
//! - Source
//!

use thiserror::Error;

mod scraping;

#[derive(Error, Debug)]
pub enum PhilEntriesError {
    #[error(transparent)]
    Request(#[from] reqwest::Error),

    #[error("failed to parse HTML")]
    Parse,

    #[error("failed to extract structured entry data")]
    Extract,
}

#[derive(Debug, Default)]
pub struct Entry {
    pub title: String,
    pub authors: Vec<String>,
    pub content: String,
    pub bibliography: Vec<String>,
    pub source: Source,
}

#[derive(Debug, Default)]
pub enum Source {
    SEP {
        url: String,
    },

    #[default]
    Other,
}

pub async fn run() -> Result<(), PhilEntriesError> {
    Ok(())
}
