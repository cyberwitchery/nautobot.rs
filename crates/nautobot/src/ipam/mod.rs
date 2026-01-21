//! ipam endpoints.
//!
//! basic usage:
//! ```no_run
//! # use nautobot::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
//! let items = client.ipam().ip_address_to_interface().list(None).await?;
//! println!("{}", items.count);
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::error::Result;
use crate::pagination::Page;
use crate::query::QueryBuilder;
use crate::resource::Resource;

/// IpAddressToInterface model.
pub type IpAddressToInterface = crate::models::IpAddressToInterface;
/// AvailableIp model.
pub type AvailableIp = crate::models::AvailableIp;
/// IpAddress model.
pub type IpAddress = crate::models::IpAddress;
/// Namespace model.
pub type Namespace = crate::models::Namespace;
/// PrefixLocationAssignment model.
pub type PrefixLocationAssignment = crate::models::PrefixLocationAssignment;
/// PrefixLengthRequest model.
pub type PrefixLengthRequest = crate::models::PrefixLengthRequest;
/// Prefix model.
pub type Prefix = crate::models::Prefix;
/// Rir model.
pub type Rir = crate::models::Rir;
/// RouteTarget model.
pub type RouteTarget = crate::models::RouteTarget;
/// Service model.
pub type Service = crate::models::Service;
/// AvailablePrefix model.
pub type AvailablePrefix = crate::models::AvailablePrefix;
/// VlanGroup model.
pub type VlanGroup = crate::models::VlanGroup;
/// VlanLocationAssignment model.
pub type VlanLocationAssignment = crate::models::VlanLocationAssignment;
/// Vlan model.
pub type Vlan = crate::models::Vlan;
/// VlanAllocationRequest model.
pub type VlanAllocationRequest = crate::models::VlanAllocationRequest;
/// VrfDeviceAssignment model.
pub type VrfDeviceAssignment = crate::models::VrfDeviceAssignment;
/// VrfPrefixAssignment model.
pub type VrfPrefixAssignment = crate::models::VrfPrefixAssignment;
/// Vrf model.
pub type Vrf = crate::models::Vrf;
/// IpAllocationRequest model.
pub type IpAllocationRequest = crate::models::IpAllocationRequest;

/// resource for ipam/ip-address-to-interface/.
pub type IpAddressToInterfaceApi = Resource<crate::models::IpAddressToInterface>;
/// resource for ipam/ip-addresses/.
pub type IpAddressesApi = Resource<crate::models::IpAddress>;
/// resource for ipam/namespaces/.
pub type NamespacesApi = Resource<crate::models::Namespace>;
/// resource for ipam/prefix-location-assignments/.
pub type PrefixLocationAssignmentsApi = Resource<crate::models::PrefixLocationAssignment>;
/// resource for ipam/prefixes/.
pub type PrefixesApi = Resource<crate::models::Prefix>;
/// resource for ipam/rirs/.
pub type RirsApi = Resource<crate::models::Rir>;
/// resource for ipam/route-targets/.
pub type RouteTargetsApi = Resource<crate::models::RouteTarget>;
/// resource for ipam/services/.
pub type ServicesApi = Resource<crate::models::Service>;
/// resource for ipam/vlan-groups/.
pub type VlanGroupsApi = Resource<crate::models::VlanGroup>;
/// resource for ipam/vlan-location-assignments/.
pub type VlanLocationAssignmentsApi = Resource<crate::models::VlanLocationAssignment>;
/// resource for ipam/vlans/.
pub type VlansApi = Resource<crate::models::Vlan>;
/// resource for ipam/vrf-device-assignments/.
pub type VrfDeviceAssignmentsApi = Resource<crate::models::VrfDeviceAssignment>;
/// resource for ipam/vrf-prefix-assignments/.
pub type VrfPrefixAssignmentsApi = Resource<crate::models::VrfPrefixAssignment>;
/// resource for ipam/vrfs/.
pub type VrfsApi = Resource<crate::models::Vrf>;

/// api for ipam endpoints.
#[derive(Clone)]
pub struct IpamApi {
    client: Client,
}

impl IpamApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// returns the ip address to interface resource.
    pub fn ip_address_to_interface(&self) -> IpAddressToInterfaceApi {
        Resource::new(self.client.clone(), "ipam/ip-address-to-interface/")
    }

    /// returns the ip addresses resource.
    pub fn ip_addresses(&self) -> IpAddressesApi {
        Resource::new(self.client.clone(), "ipam/ip-addresses/")
    }

    /// returns the namespaces resource.
    pub fn namespaces(&self) -> NamespacesApi {
        Resource::new(self.client.clone(), "ipam/namespaces/")
    }

    /// returns the prefix location assignments resource.
    pub fn prefix_location_assignments(&self) -> PrefixLocationAssignmentsApi {
        Resource::new(self.client.clone(), "ipam/prefix-location-assignments/")
    }

    /// list available IPs for a prefix by id.
    pub async fn prefix_available_ips(
        &self,
        id: &str,
        query: Option<QueryBuilder>,
    ) -> Result<Page<AvailableIp>> {
        let query = query.unwrap_or_default();
        self.client
            .get_with_params(&format!("ipam/prefixes/{}/available-ips/", id), &query)
            .await
    }

    /// allocate IPs from a prefix by id.
    pub async fn allocate_prefix_ips(
        &self,
        id: &str,
        request: &[IpAllocationRequest],
    ) -> Result<Page<IpAddress>> {
        self.client
            .post(&format!("ipam/prefixes/{}/available-ips/", id), request)
            .await
    }

    /// list available prefixes for a prefix by id.
    pub async fn prefix_available_prefixes(
        &self,
        id: &str,
        query: Option<QueryBuilder>,
    ) -> Result<Page<AvailablePrefix>> {
        let query = query.unwrap_or_default();
        self.client
            .get_with_params(&format!("ipam/prefixes/{}/available-prefixes/", id), &query)
            .await
    }

    /// allocate prefixes from a parent prefix by id.
    pub async fn allocate_prefixes(
        &self,
        id: &str,
        request: &PrefixLengthRequest,
    ) -> Result<Page<Prefix>> {
        self.client
            .post(
                &format!("ipam/prefixes/{}/available-prefixes/", id),
                request,
            )
            .await
    }

    /// returns the prefixes resource.
    pub fn prefixes(&self) -> PrefixesApi {
        Resource::new(self.client.clone(), "ipam/prefixes/")
    }

    /// returns the rirs resource.
    pub fn rirs(&self) -> RirsApi {
        Resource::new(self.client.clone(), "ipam/rirs/")
    }

    /// returns the route targets resource.
    pub fn route_targets(&self) -> RouteTargetsApi {
        Resource::new(self.client.clone(), "ipam/route-targets/")
    }

    /// returns the services resource.
    pub fn services(&self) -> ServicesApi {
        Resource::new(self.client.clone(), "ipam/services/")
    }

    /// returns the vlan groups resource.
    pub fn vlan_groups(&self) -> VlanGroupsApi {
        Resource::new(self.client.clone(), "ipam/vlan-groups/")
    }

    /// list available VLAN IDs for a VLAN group by id.
    pub async fn vlan_group_available_vlans(
        &self,
        id: &str,
        query: Option<QueryBuilder>,
    ) -> Result<Page<i32>> {
        let query = query.unwrap_or_default();
        self.client
            .get_with_params(&format!("ipam/vlan-groups/{}/available-vlans/", id), &query)
            .await
    }

    /// allocate VLANs from a VLAN group by id.
    pub async fn allocate_vlans(
        &self,
        id: &str,
        request: &[VlanAllocationRequest],
    ) -> Result<Page<Vlan>> {
        self.client
            .post(
                &format!("ipam/vlan-groups/{}/available-vlans/", id),
                request,
            )
            .await
    }

    /// returns the vlan location assignments resource.
    pub fn vlan_location_assignments(&self) -> VlanLocationAssignmentsApi {
        Resource::new(self.client.clone(), "ipam/vlan-location-assignments/")
    }

    /// returns the vlans resource.
    pub fn vlans(&self) -> VlansApi {
        Resource::new(self.client.clone(), "ipam/vlans/")
    }

    /// returns the vrf device assignments resource.
    pub fn vrf_device_assignments(&self) -> VrfDeviceAssignmentsApi {
        Resource::new(self.client.clone(), "ipam/vrf-device-assignments/")
    }

    /// returns the vrf prefix assignments resource.
    pub fn vrf_prefix_assignments(&self) -> VrfPrefixAssignmentsApi {
        Resource::new(self.client.clone(), "ipam/vrf-prefix-assignments/")
    }

    /// returns the vrfs resource.
    pub fn vrfs(&self) -> VrfsApi {
        Resource::new(self.client.clone(), "ipam/vrfs/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use httpmock::{Method::GET, Method::POST, MockServer};
    use serde_json::json;

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn ipam_available_resources_hit_expected_paths() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = IpamApi::new(client);
        let list_response = json!({
            "count": 0,
            "next": null,
            "previous": null,
            "results": []
        });

        server.mock(|when, then| {
            when.method(GET).path("/api/ipam/prefixes/1/available-ips/");
            then.status(200).json_body(list_response.clone());
        });
        server.mock(|when, then| {
            when.method(GET)
                .path("/api/ipam/prefixes/1/available-prefixes/");
            then.status(200).json_body(list_response.clone());
        });
        server.mock(|when, then| {
            when.method(GET)
                .path("/api/ipam/vlan-groups/1/available-vlans/");
            then.status(200).json_body(list_response.clone());
        });

        server.mock(|when, then| {
            when.method(POST)
                .path("/api/ipam/prefixes/1/available-ips/")
                .json_body(json!([{"status": {}}]));
            then.status(201).json_body(list_response.clone());
        });
        server.mock(|when, then| {
            when.method(POST)
                .path("/api/ipam/prefixes/1/available-prefixes/")
                .json_body(json!({"prefix_length": 29, "status": {}}));
            then.status(201).json_body(list_response.clone());
        });
        server.mock(|when, then| {
            when.method(POST)
                .path("/api/ipam/vlan-groups/1/available-vlans/")
                .json_body(json!([{"name": "vlan-1", "status": {}}]));
            then.status(201).json_body(list_response.clone());
        });

        let available_ips = api.prefix_available_ips("1", None).await.unwrap();
        assert!(available_ips.results.is_empty());

        let available_prefixes = api.prefix_available_prefixes("1", None).await.unwrap();
        assert!(available_prefixes.results.is_empty());

        let available_vlans = api.vlan_group_available_vlans("1", None).await.unwrap();
        assert!(available_vlans.results.is_empty());

        let status = crate::models::BulkWritableCableRequestStatus::new();
        let ip_request = IpAllocationRequest::new(status.clone());
        let allocated_ips = api.allocate_prefix_ips("1", &[ip_request]).await.unwrap();
        assert!(allocated_ips.results.is_empty());

        let prefix_request = PrefixLengthRequest::new(29, status.clone());
        let allocated_prefixes = api.allocate_prefixes("1", &prefix_request).await.unwrap();
        assert!(allocated_prefixes.results.is_empty());

        let vlan_request = VlanAllocationRequest::new("vlan-1".to_string(), status);
        let allocated_vlans = api.allocate_vlans("1", &[vlan_request]).await.unwrap();
        assert!(allocated_vlans.results.is_empty());
    }
}
