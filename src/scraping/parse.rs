use crate::PhilEntriesError;
use scraper::Html;

/// Parses an index page into an HTML document.
///
/// An index page contains a list of philosophy entries (e.g. SEP or IEP articles). This function
/// converts raw HTML into a structured DOM for further processing.
///
/// # Arguments
///
/// * `raw_html` - The raw HTML content of the index page.
///
/// # Returns
///
/// A parsed `Html` document that can be queried using CSS selectors.
///
/// # Errors
///
/// Returns `PhilEntriesError` if the HTML cannot be parsed.
fn parse_index_page(raw_html: &str) -> Result<Html, PhilEntriesError> {
    Ok(Html::parse_document(raw_html))
}

#[cfg(test)]
mod parse_index_page_tests {}

/// Parses an entry page into an HTML document.
///
/// An entry page contains a full philosophical article from sources like SEP or IEP.
///
/// # Arguments
///
/// * `raw_html` - The raw HTML content of the entry page.
///
/// # Returns
///
/// A parsed `Html` document for structured extraction.
///
/// # Errors
///
/// Returns `PhilEntriesError` if parsing fails.
fn parse_entry_page(raw_html: &str) -> Result<Html, PhilEntriesError> {
    Ok(Html::parse_document(raw_html))
}

#[cfg(test)]
mod parse_entry_tests {}
