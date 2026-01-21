//! cloud endpoints.
//!
//! basic usage:
//! ```no_run
//! # use nautobot::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
//! let items = client.cloud().cloud_accounts().list(None).await?;
//! println!("{}", items.count);
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::resource::Resource;

/// CloudAccount model.
pub type CloudAccount = crate::models::CloudAccount;
/// CloudNetworkPrefixAssignment model.
pub type CloudNetworkPrefixAssignment = crate::models::CloudNetworkPrefixAssignment;
/// CloudNetwork model.
pub type CloudNetwork = crate::models::CloudNetwork;
/// CloudResourceType model.
pub type CloudResourceType = crate::models::CloudResourceType;
/// CloudServiceNetworkAssignment model.
pub type CloudServiceNetworkAssignment = crate::models::CloudServiceNetworkAssignment;
/// CloudService model.
pub type CloudService = crate::models::CloudService;

/// resource for cloud/cloud-accounts/.
pub type CloudAccountsApi = Resource<crate::models::CloudAccount>;
/// resource for cloud/cloud-network-prefix-assignments/.
pub type CloudNetworkPrefixAssignmentsApi = Resource<crate::models::CloudNetworkPrefixAssignment>;
/// resource for cloud/cloud-networks/.
pub type CloudNetworksApi = Resource<crate::models::CloudNetwork>;
/// resource for cloud/cloud-resource-types/.
pub type CloudResourceTypesApi = Resource<crate::models::CloudResourceType>;
/// resource for cloud/cloud-service-network-assignments/.
pub type CloudServiceNetworkAssignmentsApi = Resource<crate::models::CloudServiceNetworkAssignment>;
/// resource for cloud/cloud-services/.
pub type CloudServicesApi = Resource<crate::models::CloudService>;

/// api for cloud endpoints.
#[derive(Clone)]
pub struct CloudApi {
    client: Client,
}

impl CloudApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// returns the cloud accounts resource.
    pub fn cloud_accounts(&self) -> CloudAccountsApi {
        Resource::new(self.client.clone(), "cloud/cloud-accounts/")
    }

    /// returns the cloud network prefix assignments resource.
    pub fn cloud_network_prefix_assignments(&self) -> CloudNetworkPrefixAssignmentsApi {
        Resource::new(
            self.client.clone(),
            "cloud/cloud-network-prefix-assignments/",
        )
    }

    /// returns the cloud networks resource.
    pub fn cloud_networks(&self) -> CloudNetworksApi {
        Resource::new(self.client.clone(), "cloud/cloud-networks/")
    }

    /// returns the cloud resource types resource.
    pub fn cloud_resource_types(&self) -> CloudResourceTypesApi {
        Resource::new(self.client.clone(), "cloud/cloud-resource-types/")
    }

    /// returns the cloud service network assignments resource.
    pub fn cloud_service_network_assignments(&self) -> CloudServiceNetworkAssignmentsApi {
        Resource::new(
            self.client.clone(),
            "cloud/cloud-service-network-assignments/",
        )
    }

    /// returns the cloud services resource.
    pub fn cloud_services(&self) -> CloudServicesApi {
        Resource::new(self.client.clone(), "cloud/cloud-services/")
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
    async fn cloud_assignment_resources_hit_expected_paths() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = CloudApi::new(client);
        let list_response = json!({
            "count": 0,
            "next": null,
            "previous": null,
            "results": []
        });

        server.mock(|when, then| {
            when.method(GET)
                .path("/api/cloud/cloud-network-prefix-assignments/");
            then.status(200).json_body(list_response.clone());
        });

        server.mock(|when, then| {
            when.method(GET)
                .path("/api/cloud/cloud-service-network-assignments/");
            then.status(200).json_body(list_response.clone());
        });

        let network_prefix = api
            .cloud_network_prefix_assignments()
            .list(None)
            .await
            .unwrap();
        assert!(network_prefix.results.is_empty());

        let service_network = api
            .cloud_service_network_assignments()
            .list(None)
            .await
            .unwrap();
        assert!(service_network.results.is_empty());
    }
}
