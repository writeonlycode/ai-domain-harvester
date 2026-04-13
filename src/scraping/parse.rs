use crate::AiDomainHarvesterError;
use scraper::Html;

/// Parses a thread index page into an HTML document.
///
/// A thread index page contains a list of threads, each representing
/// a day's domain sales.
///
/// # Arguments
///
/// * `raw_html` - The raw HTML content of the index page.
///
/// # Returns
///
/// Returns a parsed `Html` document that can be queried using selectors.
///
/// # Errors
///
/// Returns `AiDomainHarvesterError` if parsing fails.
///
/// # Notes
///
/// This function does not perform any data extraction. It is only responsible
/// for converting raw HTML into a DOM-like structure for further processing.
fn parse_thread_index_page(raw_html: &str) -> Result<Html, AiDomainHarvesterError> {
    Ok(Html::new_document())
}

/// Parses a thread page into an HTML document.
///
/// A thread page contains the detailed list of domain sales for a specific day.
///
/// # Arguments
///
/// * `raw_html` - The raw HTML content of the thread page.
///
/// # Returns
///
/// Returns a parsed `Html` document that can be queried using selectors.
///
/// # Errors
///
/// Returns `AiDomainHarvesterError` if parsing fails.
///
/// # Notes
///
/// This function does not perform any data extraction. It is only responsible
/// for preparing the document for downstream extraction logic.
fn parse_thread(raw_html: &str) -> Result<Html, AiDomainHarvesterError> {
    Ok(Html::new_document())
}
