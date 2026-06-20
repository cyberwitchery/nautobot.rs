//! metrics endpoint.
//!
//! basic usage:
//! ```no_run
//! # use nautobot::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
//! let metrics = client.metrics().metrics().await?;
//! println!("{}", metrics.lines().next().unwrap_or(""));
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::error::{Error, Result};
use reqwest::Method;

/// api for metrics endpoint.
#[derive(Clone)]
pub struct MetricsApi {
    client: Client,
}

impl MetricsApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// fetch the prometheus metrics payload.
    pub async fn metrics(&self) -> Result<String> {
        let path = "metrics/";
        self.client
            .retry_loop(Method::GET, path, false, |_attempt| async move {
                let url = self.client.config().build_url(path)?;
                let request = self.client.http_client().get(url);
                let response = self
                    .client
                    .execute_request(&Method::GET, path, request)
                    .await?;
                let status = response.status();
                let body = response.text().await.map_err(Error::from)?;
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from_response(status, body))
                }
            })
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ClientConfig, HttpHooks};
    use httpmock::prelude::HttpMockRequest;
    use httpmock::{Method::GET, MockServer};
    use reqwest::StatusCode;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn metrics_hits_expected_path() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = MetricsApi::new(client);

        server.mock(|when, then| {
            when.method(GET).path("/api/metrics/");
            then.status(200).body("# HELP test_metric 1");
        });

        let metrics = api.metrics().await.unwrap();
        assert!(metrics.contains("test_metric"));
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn metrics_retries_on_429() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(2);
        let client = Client::new(config).unwrap();
        let api = MetricsApi::new(client);

        let call_count = Arc::new(AtomicUsize::new(0));
        let counter = call_count.clone();
        let fail = server.mock(|when, then| {
            when.method(GET)
                .path("/api/metrics/")
                .is_true(move |_: &HttpMockRequest| counter.fetch_add(1, Ordering::SeqCst) == 0);
            then.status(429).body("rate limited");
        });
        let succeed = server.mock(|when, then| {
            when.method(GET).path("/api/metrics/");
            then.status(200).body("# HELP up 1");
        });

        let metrics = api.metrics().await.unwrap();
        assert!(metrics.contains("up"));
        fail.assert_calls(1);
        succeed.assert_calls(1);
    }

    struct StatusCapture {
        statuses: Arc<Mutex<Vec<u16>>>,
    }

    impl HttpHooks for StatusCapture {
        fn on_response(
            &self,
            _method: &reqwest::Method,
            _path: &str,
            status: StatusCode,
            _duration: Duration,
        ) {
            self.statuses.lock().unwrap().push(status.as_u16());
        }
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn metrics_invokes_hooks() {
        let server = MockServer::start();
        let statuses = Arc::new(Mutex::new(Vec::new()));
        let hook = StatusCapture {
            statuses: statuses.clone(),
        };
        let config = ClientConfig::new(server.base_url(), "token")
            .with_max_retries(0)
            .with_http_hooks(hook);
        let client = Client::new(config).unwrap();
        let api = MetricsApi::new(client);

        server.mock(|when, then| {
            when.method(GET).path("/api/metrics/");
            then.status(200).body("# HELP up 1");
        });

        api.metrics().await.unwrap();
        let captured = statuses.lock().unwrap().clone();
        assert_eq!(captured, vec![200]);
    }
}
