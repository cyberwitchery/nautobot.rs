//! ui endpoints.
//!
//! basic usage:
//! ```no_run
//! # use nautobot::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
//! let request = nautobot::models::RenderJinjaRequest { template: "{{ hello }}".to_string() };
//! let rendered = client.ui().render_jinja_template(&request).await?;
//! println!("{}", rendered.result.unwrap_or_default());
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::error::Result;

/// render-jinja response model.
pub type RenderJinja = crate::models::RenderJinja;
/// render-jinja request model.
pub type RenderJinjaRequest = crate::models::RenderJinjaRequest;

/// api for ui endpoints.
#[derive(Clone)]
pub struct UiApi {
    client: Client,
}

impl UiApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// render a jinja template via the ui endpoint.
    pub async fn render_jinja_template(&self, request: &RenderJinjaRequest) -> Result<RenderJinja> {
        self.client
            .post("ui/core/render-jinja-template/", request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use httpmock::{Method::POST, MockServer};
    use serde_json::json;

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn ui_render_jinja_hits_expected_path() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = UiApi::new(client);

        server.mock(|when, then| {
            when.method(POST)
                .path("/api/ui/core/render-jinja-template/");
            then.status(200)
                .json_body(json!({ "template_code": "tmpl" }));
        });

        let response = api
            .render_jinja_template(&RenderJinjaRequest::new("tmpl".to_string()))
            .await
            .unwrap();
        assert_eq!(response.template_code, "tmpl");
    }
}
