use crate::{Entry, PhilEntriesError};
use scraper::Html;

/// Extracts entry URLs from an index page.
///
/// An index page contains links to individual philosophy entries (e.g. SEP or IEP articles).
/// This function parses the HTML document and returns the unique URLs used to fetch full entry
/// pages.
///
/// # Arguments
///
/// * `html` - A parsed HTML document representing an index page.
///
/// # Returns
///
/// A vector of entry URLs.
///
/// # Errors
///
/// Returns `PhilEntriesError` if the expected document structure cannot be parsed or
/// required elements are missing.
fn extract_entry_slugs(html: &Html) -> Result<Vec<String>, PhilEntriesError> {
    Ok(Vec::new())
}

#[cfg(test)]
mod extract_entry_slugs_tests {}

/// Extracts structured philosophical content from an entry page.
///
/// An entry page contains a full philosophical article. This function parses the document,
/// extracts meaningful sections, and normalizes them into structured data.
///
/// # Arguments
///
/// * `html` - A parsed HTML document representing a philosophy entry page.
///
/// # Returns
///
/// An `Entry` record containing structured philosophical content.
///
/// # Errors
///
/// Returns `PhilEntriesError` if the document cannot be parsed or required content is missing.
fn extract_entries(html: &Html) -> Result<Entry, PhilEntriesError> {
    Ok(Entry::default())
}

#[cfg(test)]
mod extract_entries_tests {}
