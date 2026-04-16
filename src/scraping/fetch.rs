use crate::PhilEntriesError;
use reqwest::{
    Client,
    header::{
        ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION, HeaderMap, HeaderValue,
        UPGRADE_INSECURE_REQUESTS, USER_AGENT,
    },
};

pub const ENTRIES_INDEX_PAGE_URL: &'static str = "https://plato.stanford.edu/contents.html";
pub const ENTRY_BASE_URL: &'static str = "https://plato.stanford.edu";

/// Fetches the SEP index page containing all philosophy entries.
///
/// The index page lists all available encyclopedia entries (e.g. "Free Will", "Consciousness").
///
/// # Arguments
///
/// * `client` - HTTP client
/// * `url` - URL of the table of contents
///
/// # Returns
///
/// Raw HTML content of the index page.
///
/// # Errors
///
/// Returns `PhilEntriesError` if the request fails or response cannot be parsed.
async fn fetch_index_page(client: &Client, url: &str) -> Result<String, PhilEntriesError> {
    Ok(client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?)
}

#[cfg(test)]
mod fetch_index_page_tests {
    use super::*;
    use httpmock::Method::GET;

    #[tokio::test]
    async fn handles_success() {
        let server = httpmock::MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/top-trending-remote-jobs");

            then.status(200).header("content-type", "text/html").body(
                "<!DOCTYPE html><title>200 OK</title><p>The request succeeded. The resource has been fetched and transmitted in the message body.</p>",
            );
        });

        let client = build_client();

        let server_url = format!("{}{}", server.base_url(), "/top-trending-remote-jobs");
        let result = fetch_index_page(&client, server_url.as_str())
            .await
            .unwrap();
        let expected = "<!DOCTYPE html><title>200 OK</title><p>The request succeeded. The resource has been fetched and transmitted in the message body.</p>"
            .to_string();

        // Ensure the specified mock was called exactly one time.
        mock.assert();

        assert_eq!(
            result, expected,
            "server_url {:?} produced an unexpected result: got {:?}, but expected {:?}",
            server_url, result, expected
        );
    }

    #[tokio::test]
    async fn handles_failure() {
        let server = httpmock::MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/top-trending-remote-jobs");

            then.status(500).header("content-type", "text/html").body(
                "<!DOCTYPE html><title>500 Internal Server Error</title><p>The server has encountered a situation it does not know how to handle.</p>",
            );
        });

        let client = build_client();
        let server_url = format!("{}{}", server.base_url(), "/top-trending-remote-jobs");
        let result = fetch_index_page(&client, server_url.as_str()).await;

        // Ensure the specified mock was called exactly one time.
        mock.assert();

        assert!(
            matches!(result, Err(PhilEntriesError::Request(_))),
            "server_url {:?} produced an unexpected result: got {:?}, but expected error",
            server_url,
            result,
        );
    }
}

/// Fetches a single SEP entry page.
///
/// Each entry represents a philosophical article (e.g. "Free Will").
///
/// # Arguments
///
/// * `client` - HTTP client
/// * `url` - URL of the entry
///
/// # Returns
///
/// Raw HTML of the entry page.
async fn fetch_entry(client: &Client, url: &str) -> Result<String, PhilEntriesError> {
    Ok(client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?)
}

#[cfg(test)]
mod fetch_entry_tests {}

pub fn build_client() -> Client {
    let mut headers = HeaderMap::new();

    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
         (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
        ),
    );

    headers.insert(
        ACCEPT,
        HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        ),
    );

    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));

    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );

    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));

    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));

    Client::builder()
        .default_headers(headers)
        .http1_only() // 🔥 helps with fingerprinting
        .build()
        .unwrap()
}
