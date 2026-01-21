//! circuits endpoints.
//!
//! basic usage:
//! ```no_run
//! # use nautobot::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
//! let items = client.circuits().circuit_terminations().list(None).await?;
//! println!("{}", items.count);
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::error::Result;
use crate::resource::Resource;

/// CircuitTermination model.
pub type CircuitTermination = crate::models::CircuitTermination;
/// CircuitType model.
pub type CircuitType = crate::models::CircuitType;
/// Circuit model.
pub type Circuit = crate::models::Circuit;
/// ProviderNetwork model.
pub type ProviderNetwork = crate::models::ProviderNetwork;
/// Provider model.
pub type Provider = crate::models::Provider;

/// resource for circuits/circuit-terminations/.
pub type CircuitTerminationsApi = Resource<crate::models::CircuitTermination>;
/// resource for circuits/circuit-types/.
pub type CircuitTypesApi = Resource<crate::models::CircuitType>;
/// resource for circuits/circuits/.
pub type CircuitsResource = Resource<crate::models::Circuit>;
/// resource for circuits/provider-networks/.
pub type ProviderNetworksApi = Resource<crate::models::ProviderNetwork>;
/// resource for circuits/providers/.
pub type ProvidersApi = Resource<crate::models::Provider>;

/// api for circuits endpoints.
#[derive(Clone)]
pub struct CircuitsApi {
    client: Client,
}

impl CircuitsApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// returns the circuit terminations resource.
    pub fn circuit_terminations(&self) -> CircuitTerminationsApi {
        Resource::new(self.client.clone(), "circuits/circuit-terminations/")
    }

    /// trace a circuit termination path by id.
    pub async fn circuit_termination_trace(
        &self,
        id: &str,
    ) -> Result<crate::models::CircuitTermination> {
        self.client
            .get(&format!("circuits/circuit-terminations/{}/trace/", id))
            .await
    }

    /// returns the circuit types resource.
    pub fn circuit_types(&self) -> CircuitTypesApi {
        Resource::new(self.client.clone(), "circuits/circuit-types/")
    }

    /// returns the circuits resource.
    pub fn circuits(&self) -> CircuitsResource {
        Resource::new(self.client.clone(), "circuits/circuits/")
    }

    /// returns the provider networks resource.
    pub fn provider_networks(&self) -> ProviderNetworksApi {
        Resource::new(self.client.clone(), "circuits/provider-networks/")
    }

    /// returns the providers resource.
    pub fn providers(&self) -> ProvidersApi {
        Resource::new(self.client.clone(), "circuits/providers/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use httpmock::{Method::GET, MockServer};
    use serde_json::json;

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn circuits_trace_hits_expected_path() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = CircuitsApi::new(client);

        server.mock(|when, then| {
            when.method(GET)
                .path("/api/circuits/circuit-terminations/1/trace/");
            then.status(200)
                .json_body(json!({"term_side": "A", "circuit": {}}));
        });

        let _ = api.circuit_termination_trace("1").await.unwrap();
    }
}
