use crate::AiDomainHarvesterError;

const THREAD_INDEX_PAGE_BASE_URL: &'static str = "https://www.namepros.com";
const THREAD_INDEX_PAGE_PATH: &'static str = "/forums/daily-domain-sales.383";

const THREAD_BASE_URL: &'static str = "https://www.namepros.com/threads/";

/// Fetches a thread index page from the NamePros "Daily Domain Sales" forum.
///
/// Given a page number, this function retrieves the corresponding index page containing a list of
/// threads (each representing a day's domain sales).
///
/// # Arguments
///
/// * `page` - The page number to fetch (e.g. `2` for `/page-2`).
///
/// # Returns
///
/// Returns the raw HTML of the thread index page as a `String`.
///
/// # Errors
///
/// Returns `AiDomainHarvesterError` if the request fails, the response cannot be read, or a
/// non-success status code is returned.
///
/// # Example
///
/// ```no_run
/// let client = reqwest::Client::new();
/// let html = fetch_thread_index_page(&client, 1).await?;
/// ```
async fn fetch_thread_index_page(
    client: &reqwest::Client,
    base_url: &str,
    page: u64,
) -> Result<String, AiDomainHarvesterError> {
    Ok(format!(""))
}

#[cfg(test)]
mod fetch_thread_index_page_tests {
    use super::*;
    use httpmock::Method::GET;

    #[tokio::test]
    async fn handles_success() {
        let server = httpmock::MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(GET)
                .path("/forums/daily-domain-sales.383/page-1");

            then.status(200).header("content-type", "text/html").body(
                "<!DOCTYPE html><title>200 OK</title><p>The request succeeded. The resource
                    has been fetched and transmitted in the message body.</p>",
            );
        });

        let client = reqwest::Client::new();
        let server_url = server.base_url();
        let result = fetch_thread_index_page(&client, server_url.as_str(), 1)
            .await
            .unwrap();
        let expected = "<!DOCTYPE html><title>200 OK</title><p>The request succeeded. The resource
                    has been fetched and transmitted in the message body.</p>"
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
                .path("/forums/daily-domain-sales.383/page-1");

            then.status(500).header("content-type", "text/html").body(
                "<!DOCTYPE html><title>500 Internal Server Error</title><p>The server has
                    encountered a situation it does not know how to handle. This error is generic,
                    indicating that the server cannot find a more appropriate 5XX status code to
                    respond with.</p>",
            );
        });

        let client = reqwest::Client::new();
        let server_url = server.base_url();
        let result = fetch_thread_index_page(&client, server_url.as_str(), 1).await;

        // Ensure the specified mock was called exactly one time.
        mock.assert();

        assert!(
            matches!(result, Err(AiDomainHarvesterError::Request(_))),
            "server_url {:?} produced an unexpected result: got {:?}, but expected error",
            server_url,
            result,
        );
    }
}

/// Fetches a single thread page from the NamePros forum.
///
/// A thread contains the detailed list of domain sales for a specific day. The thread is identified
/// by its slug.
///
/// # Arguments
///
/// * `thread_slug` - The thread slug (e.g.
///   `"19-net-sold-for-52-088-stagehand-com-for-42-000.1383854"`).
///
/// # Returns
///
/// Returns the raw HTML of the thread page as a `String`.
///
/// # Errors
///
/// Returns `AiDomainHarvesterError` if the request fails, the response cannot be read, or a
/// non-success status code is returned.
///
/// # Example
///
/// ```no_run
/// let client = reqwest::Client::new();
/// let html = fetch_thread(&client, "example-thread-slug").await?;
/// ```
async fn fetch_thread(
    client: &reqwest::Client,
    base_url: &str,
    thread_slug: &str,
) -> Result<String, AiDomainHarvesterError> {
    Ok(format!(""))
}

#[cfg(test)]
mod fetch_thread_tests {}
