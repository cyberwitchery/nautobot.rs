//! wireless endpoints.
//!
//! basic usage:
//! ```no_run
//! # use nautobot::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
//! let items = client.wireless().controller_managed_device_group_radio_profile_assignments().list(None).await?;
//! println!("{}", items.count);
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::resource::Resource;

/// ControllerManagedDeviceGroupRadioProfileAssignment model.
pub type ControllerManagedDeviceGroupRadioProfileAssignment =
    crate::models::ControllerManagedDeviceGroupRadioProfileAssignment;
/// ControllerManagedDeviceGroupWirelessNetworkAssignment model.
pub type ControllerManagedDeviceGroupWirelessNetworkAssignment =
    crate::models::ControllerManagedDeviceGroupWirelessNetworkAssignment;
/// RadioProfile model.
pub type RadioProfile = crate::models::RadioProfile;
/// SupportedDataRate model.
pub type SupportedDataRate = crate::models::SupportedDataRate;
/// WirelessNetwork model.
pub type WirelessNetwork = crate::models::WirelessNetwork;

/// resource for wireless/controller-managed-device-group-radio-profile-assignments/.
pub type ControllerManagedDeviceGroupRadioProfileAssignmentsApi =
    Resource<crate::models::ControllerManagedDeviceGroupRadioProfileAssignment>;
/// resource for wireless/controller-managed-device-group-wireless-network-assignments/.
pub type ControllerManagedDeviceGroupWirelessNetworkAssignmentsApi =
    Resource<crate::models::ControllerManagedDeviceGroupWirelessNetworkAssignment>;
/// resource for wireless/radio-profiles/.
pub type RadioProfilesApi = Resource<crate::models::RadioProfile>;
/// resource for wireless/supported-data-rates/.
pub type SupportedDataRatesApi = Resource<crate::models::SupportedDataRate>;
/// resource for wireless/wireless-networks/.
pub type WirelessNetworksApi = Resource<crate::models::WirelessNetwork>;

/// api for wireless endpoints.
#[derive(Clone)]
pub struct WirelessApi {
    client: Client,
}

impl WirelessApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// returns the controller managed device group radio profile assignments resource.
    pub fn controller_managed_device_group_radio_profile_assignments(
        &self,
    ) -> ControllerManagedDeviceGroupRadioProfileAssignmentsApi {
        Resource::new(
            self.client.clone(),
            "wireless/controller-managed-device-group-radio-profile-assignments/",
        )
    }

    /// returns the controller managed device group wireless network assignments resource.
    pub fn controller_managed_device_group_wireless_network_assignments(
        &self,
    ) -> ControllerManagedDeviceGroupWirelessNetworkAssignmentsApi {
        Resource::new(
            self.client.clone(),
            "wireless/controller-managed-device-group-wireless-network-assignments/",
        )
    }

    /// returns the radio profiles resource.
    pub fn radio_profiles(&self) -> RadioProfilesApi {
        Resource::new(self.client.clone(), "wireless/radio-profiles/")
    }

    /// returns the supported data rates resource.
    pub fn supported_data_rates(&self) -> SupportedDataRatesApi {
        Resource::new(self.client.clone(), "wireless/supported-data-rates/")
    }

    /// returns the wireless networks resource.
    pub fn wireless_networks(&self) -> WirelessNetworksApi {
        Resource::new(self.client.clone(), "wireless/wireless-networks/")
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
    async fn wireless_controller_group_assignments_hit_expected_paths() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = WirelessApi::new(client);
        let list_response = json!({
            "count": 0,
            "next": null,
            "previous": null,
            "results": []
        });

        server.mock(|when, then| {
            when.method(GET)
                .path("/api/wireless/controller-managed-device-group-radio-profile-assignments/");
            then.status(200).json_body(list_response.clone());
        });

        server.mock(|when, then| {
            when.method(GET).path(
                "/api/wireless/controller-managed-device-group-wireless-network-assignments/",
            );
            then.status(200).json_body(list_response.clone());
        });

        let radio_profiles = api
            .controller_managed_device_group_radio_profile_assignments()
            .list(None)
            .await
            .unwrap();
        assert!(radio_profiles.results.is_empty());

        let wireless_networks = api
            .controller_managed_device_group_wireless_network_assignments()
            .list(None)
            .await
            .unwrap();
        assert!(wireless_networks.results.is_empty());
    }
}
