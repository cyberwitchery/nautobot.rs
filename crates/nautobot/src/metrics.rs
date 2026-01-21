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
        let url = self.client.config().build_url("metrics/")?;
        let response = self.client.http_client().get(url).send().await?;
        let status = response.status();
        let body = response.text().await.map_err(Error::from)?;
        if status.is_success() {
            Ok(body)
        } else {
            Err(Error::from_response(status, body))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use httpmock::{Method::GET, MockServer};

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
}
