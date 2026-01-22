//! core endpoints.
//!
//! basic usage:
//! ```no_run
//! # use nautobot::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
//! let request = nautobot::models::RenderJinjaRequest::new("{{ hello }}".to_string());
//! let rendered = client.core().render_jinja_template(&request).await?;
//! println!("{}", rendered.rendered_template.unwrap_or_default());
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::error::{Error, Result};
use reqwest::Method;

/// render-jinja response model.
pub type RenderJinja = crate::models::RenderJinja;
/// render-jinja request model.
pub type RenderJinjaRequest = crate::models::RenderJinjaRequest;

/// api for core endpoints.
#[derive(Clone)]
pub struct CoreApi {
    client: Client,
}

impl CoreApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// render a jinja template.
    pub async fn render_jinja_template(&self, request: &RenderJinjaRequest) -> Result<RenderJinja> {
        self.client
            .post("core/render-jinja-template/", request)
            .await
    }

    /// fetch the swagger json schema.
    pub async fn swagger_json(&self) -> Result<serde_json::Value> {
        self.client
            .request_raw(Method::GET, "swagger.json", None)
            .await
    }

    /// fetch the swagger yaml schema.
    pub async fn swagger_yaml(&self) -> Result<String> {
        let url = self.client.config().build_url("swagger.yaml")?;
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
    use httpmock::{Method::POST, MockServer};
    use serde_json::json;

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn core_render_jinja_hits_expected_path() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = CoreApi::new(client);

        server.mock(|when, then| {
            when.method(POST).path("/api/core/render-jinja-template/");
            then.status(200)
                .json_body(json!({ "template_code": "tmpl" }));
        });

        let response = api
            .render_jinja_template(&RenderJinjaRequest::new("tmpl".to_string()))
            .await
            .unwrap();
        assert_eq!(response.template_code, "tmpl");
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn core_swagger_fetches_expected_paths() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = CoreApi::new(client);

        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/api/swagger.json");
            then.status(200).json_body(json!({"openapi": "3.0.0"}));
        });
        server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/api/swagger.yaml");
            then.status(200).body("openapi: 3.0.0");
        });

        let json_value = api.swagger_json().await.unwrap();
        assert_eq!(json_value["openapi"], "3.0.0");

        let yaml_value = api.swagger_yaml().await.unwrap();
        assert!(yaml_value.contains("openapi: 3.0.0"));
    }
}
