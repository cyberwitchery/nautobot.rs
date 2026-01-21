//! dcim endpoints.
//!
//! basic usage:
//! ```no_run
//! # use nautobot::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
//! let items = client.dcim().cables().list(None).await?;
//! println!("{}", items.count);
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::error::Result;
use crate::pagination::Page;
use crate::query::QueryBuilder;
use crate::resource::Resource;
use serde::Serialize;

/// Cable model.
pub type Cable = crate::models::Cable;
/// CablePath model.
pub type CablePath = crate::models::CablePath;
/// Device model.
pub type Device = crate::models::Device;
/// DeviceNapalm model.
pub type DeviceNapalm = crate::models::DeviceNapalm;
/// ConsolePort model.
pub type ConsolePort = crate::models::ConsolePort;
/// ConsolePortTemplate model.
pub type ConsolePortTemplate = crate::models::ConsolePortTemplate;
/// ConsoleServerPortTemplate model.
pub type ConsoleServerPortTemplate = crate::models::ConsoleServerPortTemplate;
/// ConsoleServerPort model.
pub type ConsoleServerPort = crate::models::ConsoleServerPort;
/// ControllerManagedDeviceGroup model.
pub type ControllerManagedDeviceGroup = crate::models::ControllerManagedDeviceGroup;
/// Controller model.
pub type Controller = crate::models::Controller;
/// DeviceBayTemplate model.
pub type DeviceBayTemplate = crate::models::DeviceBayTemplate;
/// DeviceBay model.
pub type DeviceBay = crate::models::DeviceBay;
/// DeviceFamily model.
pub type DeviceFamily = crate::models::DeviceFamily;
/// DeviceRedundancyGroup model.
pub type DeviceRedundancyGroup = crate::models::DeviceRedundancyGroup;
/// DeviceTypeToSoftwareImageFile model.
pub type DeviceTypeToSoftwareImageFile = crate::models::DeviceTypeToSoftwareImageFile;
/// DeviceType model.
pub type DeviceType = crate::models::DeviceType;
/// FrontPortTemplate model.
pub type FrontPortTemplate = crate::models::FrontPortTemplate;
/// FrontPort model.
pub type FrontPort = crate::models::FrontPort;
/// InterfaceConnection model.
pub type InterfaceConnection = crate::models::InterfaceConnection;
/// InterfaceRedundancyGroupAssociation model.
pub type InterfaceRedundancyGroupAssociation = crate::models::InterfaceRedundancyGroupAssociation;
/// InterfaceRedundancyGroup model.
pub type InterfaceRedundancyGroup = crate::models::InterfaceRedundancyGroup;
/// InterfaceTemplate model.
pub type InterfaceTemplate = crate::models::InterfaceTemplate;
/// InterfaceVdcAssignment model.
pub type InterfaceVdcAssignment = crate::models::InterfaceVdcAssignment;
/// Interface model.
pub type Interface = crate::models::Interface;
/// InventoryItem model.
pub type InventoryItem = crate::models::InventoryItem;
/// LocationType model.
pub type LocationType = crate::models::LocationType;
/// Location model.
pub type Location = crate::models::Location;
/// Manufacturer model.
pub type Manufacturer = crate::models::Manufacturer;
/// ModuleBayTemplate model.
pub type ModuleBayTemplate = crate::models::ModuleBayTemplate;
/// ModuleBay model.
pub type ModuleBay = crate::models::ModuleBay;
/// ModuleFamily model.
pub type ModuleFamily = crate::models::ModuleFamily;
/// ModuleType model.
pub type ModuleType = crate::models::ModuleType;
/// Module model.
pub type Module = crate::models::Module;
/// Platform model.
pub type Platform = crate::models::Platform;
/// PowerPort model.
pub type PowerPort = crate::models::PowerPort;
/// PowerFeed model.
pub type PowerFeed = crate::models::PowerFeed;
/// PowerOutletTemplate model.
pub type PowerOutletTemplate = crate::models::PowerOutletTemplate;
/// PowerOutlet model.
pub type PowerOutlet = crate::models::PowerOutlet;
/// PowerPanel model.
pub type PowerPanel = crate::models::PowerPanel;
/// PowerPortTemplate model.
pub type PowerPortTemplate = crate::models::PowerPortTemplate;
/// RackGroup model.
pub type RackGroup = crate::models::RackGroup;
/// RackReservation model.
pub type RackReservation = crate::models::RackReservation;
/// Rack model.
pub type Rack = crate::models::Rack;
/// RackUnit model.
pub type RackUnit = crate::models::RackUnit;
/// RearPortTemplate model.
pub type RearPortTemplate = crate::models::RearPortTemplate;
/// RearPort model.
pub type RearPort = crate::models::RearPort;
/// SoftwareImageFile model.
pub type SoftwareImageFile = crate::models::SoftwareImageFile;
/// SoftwareVersion model.
pub type SoftwareVersion = crate::models::SoftwareVersion;
/// VirtualChassis model.
pub type VirtualChassis = crate::models::VirtualChassis;
/// VirtualDeviceContext model.
pub type VirtualDeviceContext = crate::models::VirtualDeviceContext;

#[derive(Serialize)]
struct ConnectedDeviceQuery<'a> {
    peer_device: &'a str,
    peer_interface: &'a str,
    #[serde(flatten)]
    extra: Option<QueryBuilder>,
}

/// resource for dcim/cables/.
pub type CablesApi = Resource<crate::models::Cable>;
/// resource for dcim/console-connections/.
pub type ConsoleConnectionsApi = Resource<crate::models::ConsolePort>;
/// resource for dcim/console-port-templates/.
pub type ConsolePortTemplatesApi = Resource<crate::models::ConsolePortTemplate>;
/// resource for dcim/console-ports/.
pub type ConsolePortsApi = Resource<crate::models::ConsolePort>;
/// resource for dcim/console-server-port-templates/.
pub type ConsoleServerPortTemplatesApi = Resource<crate::models::ConsoleServerPortTemplate>;
/// resource for dcim/console-server-ports/.
pub type ConsoleServerPortsApi = Resource<crate::models::ConsoleServerPort>;
/// resource for dcim/controller-managed-device-groups/.
pub type ControllerManagedDeviceGroupsApi = Resource<crate::models::ControllerManagedDeviceGroup>;
/// resource for dcim/controllers/.
pub type ControllersApi = Resource<crate::models::Controller>;
/// resource for dcim/device-bay-templates/.
pub type DeviceBayTemplatesApi = Resource<crate::models::DeviceBayTemplate>;
/// resource for dcim/device-bays/.
pub type DeviceBaysApi = Resource<crate::models::DeviceBay>;
/// resource for dcim/device-families/.
pub type DeviceFamiliesApi = Resource<crate::models::DeviceFamily>;
/// resource for dcim/device-redundancy-groups/.
pub type DeviceRedundancyGroupsApi = Resource<crate::models::DeviceRedundancyGroup>;
/// resource for dcim/device-types-to-software-image-files/.
pub type DeviceTypesToSoftwareImageFilesApi =
    Resource<crate::models::DeviceTypeToSoftwareImageFile>;
/// resource for dcim/device-types/.
pub type DeviceTypesApi = Resource<crate::models::DeviceType>;
/// resource for dcim/devices/.
pub type DevicesApi = Resource<crate::models::Device>;
/// resource for dcim/front-port-templates/.
pub type FrontPortTemplatesApi = Resource<crate::models::FrontPortTemplate>;
/// resource for dcim/front-ports/.
pub type FrontPortsApi = Resource<crate::models::FrontPort>;
/// resource for dcim/interface-connections/.
pub type InterfaceConnectionsApi = Resource<crate::models::InterfaceConnection>;
/// resource for dcim/interface-redundancy-group-associations/.
pub type InterfaceRedundancyGroupAssociationsApi =
    Resource<crate::models::InterfaceRedundancyGroupAssociation>;
/// resource for dcim/interface-redundancy-groups/.
pub type InterfaceRedundancyGroupsApi = Resource<crate::models::InterfaceRedundancyGroup>;
/// resource for dcim/interface-templates/.
pub type InterfaceTemplatesApi = Resource<crate::models::InterfaceTemplate>;
/// resource for dcim/interface-vdc-assignments/.
pub type InterfaceVdcAssignmentsApi = Resource<crate::models::InterfaceVdcAssignment>;
/// resource for dcim/interfaces/.
pub type InterfacesApi = Resource<crate::models::Interface>;
/// resource for dcim/inventory-items/.
pub type InventoryItemsApi = Resource<crate::models::InventoryItem>;
/// resource for dcim/location-types/.
pub type LocationTypesApi = Resource<crate::models::LocationType>;
/// resource for dcim/locations/.
pub type LocationsApi = Resource<crate::models::Location>;
/// resource for dcim/manufacturers/.
pub type ManufacturersApi = Resource<crate::models::Manufacturer>;
/// resource for dcim/module-bay-templates/.
pub type ModuleBayTemplatesApi = Resource<crate::models::ModuleBayTemplate>;
/// resource for dcim/module-bays/.
pub type ModuleBaysApi = Resource<crate::models::ModuleBay>;
/// resource for dcim/module-families/.
pub type ModuleFamiliesApi = Resource<crate::models::ModuleFamily>;
/// resource for dcim/module-types/.
pub type ModuleTypesApi = Resource<crate::models::ModuleType>;
/// resource for dcim/modules/.
pub type ModulesApi = Resource<crate::models::Module>;
/// resource for dcim/platforms/.
pub type PlatformsApi = Resource<crate::models::Platform>;
/// resource for dcim/power-connections/.
pub type PowerConnectionsApi = Resource<crate::models::PowerPort>;
/// resource for dcim/power-feeds/.
pub type PowerFeedsApi = Resource<crate::models::PowerFeed>;
/// resource for dcim/power-outlet-templates/.
pub type PowerOutletTemplatesApi = Resource<crate::models::PowerOutletTemplate>;
/// resource for dcim/power-outlets/.
pub type PowerOutletsApi = Resource<crate::models::PowerOutlet>;
/// resource for dcim/power-panels/.
pub type PowerPanelsApi = Resource<crate::models::PowerPanel>;
/// resource for dcim/power-port-templates/.
pub type PowerPortTemplatesApi = Resource<crate::models::PowerPortTemplate>;
/// resource for dcim/power-ports/.
pub type PowerPortsApi = Resource<crate::models::PowerPort>;
/// resource for dcim/rack-groups/.
pub type RackGroupsApi = Resource<crate::models::RackGroup>;
/// resource for dcim/rack-reservations/.
pub type RackReservationsApi = Resource<crate::models::RackReservation>;
/// resource for dcim/racks/.
pub type RacksApi = Resource<crate::models::Rack>;
/// resource for dcim/rear-port-templates/.
pub type RearPortTemplatesApi = Resource<crate::models::RearPortTemplate>;
/// resource for dcim/rear-ports/.
pub type RearPortsApi = Resource<crate::models::RearPort>;
/// resource for dcim/software-image-files/.
pub type SoftwareImageFilesApi = Resource<crate::models::SoftwareImageFile>;
/// resource for dcim/software-versions/.
pub type SoftwareVersionsApi = Resource<crate::models::SoftwareVersion>;
/// resource for dcim/virtual-chassis/.
pub type VirtualChassisApi = Resource<crate::models::VirtualChassis>;
/// resource for dcim/virtual-device-contexts/.
pub type VirtualDeviceContextsApi = Resource<crate::models::VirtualDeviceContext>;

/// api for dcim endpoints.
#[derive(Clone)]
pub struct DcimApi {
    client: Client,
}

impl DcimApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// returns the cables resource.
    pub fn cables(&self) -> CablesApi {
        Resource::new(self.client.clone(), "dcim/cables/")
    }

    /// returns the device connected to a peer device/interface pair.
    pub async fn connected_device(
        &self,
        peer_device: &str,
        peer_interface: &str,
        query: Option<QueryBuilder>,
    ) -> Result<Vec<crate::models::Device>> {
        let params = ConnectedDeviceQuery {
            peer_device,
            peer_interface,
            extra: query,
        };
        self.client
            .get_with_params("dcim/connected-device/", &params)
            .await
    }

    /// fetch napalm data for a device by id.
    pub async fn device_napalm(&self, id: &str) -> Result<DeviceNapalm> {
        self.client
            .get(&format!("dcim/devices/{}/napalm/", id))
            .await
    }

    /// list cable paths for a front port by id.
    pub async fn front_port_paths(
        &self,
        id: &str,
        query: Option<QueryBuilder>,
    ) -> Result<Page<CablePath>> {
        let query = query.unwrap_or_default();
        self.client
            .get_with_params(&format!("dcim/front-ports/{}/paths/", id), &query)
            .await
    }

    /// list cable paths for a rear port by id.
    pub async fn rear_port_paths(
        &self,
        id: &str,
        query: Option<QueryBuilder>,
    ) -> Result<Page<CablePath>> {
        let query = query.unwrap_or_default();
        self.client
            .get_with_params(&format!("dcim/rear-ports/{}/paths/", id), &query)
            .await
    }

    /// fetch the rack elevation units for a rack by id.
    pub async fn rack_elevation(
        &self,
        id: &str,
        query: Option<QueryBuilder>,
    ) -> Result<Page<RackUnit>> {
        let query = query.unwrap_or_default();
        self.client
            .get_with_params(&format!("dcim/racks/{}/elevation/", id), &query)
            .await
    }

    /// returns the console connections resource.
    pub fn console_connections(&self) -> ConsoleConnectionsApi {
        Resource::new(self.client.clone(), "dcim/console-connections/")
    }

    /// returns the console port templates resource.
    pub fn console_port_templates(&self) -> ConsolePortTemplatesApi {
        Resource::new(self.client.clone(), "dcim/console-port-templates/")
    }

    /// returns the console ports resource.
    pub fn console_ports(&self) -> ConsolePortsApi {
        Resource::new(self.client.clone(), "dcim/console-ports/")
    }

    /// trace a console port path by id.
    pub async fn console_port_trace(&self, id: &str) -> Result<crate::models::ConsolePort> {
        self.client
            .get(&format!("dcim/console-ports/{}/trace/", id))
            .await
    }

    /// returns the console server port templates resource.
    pub fn console_server_port_templates(&self) -> ConsoleServerPortTemplatesApi {
        Resource::new(self.client.clone(), "dcim/console-server-port-templates/")
    }

    /// returns the console server ports resource.
    pub fn console_server_ports(&self) -> ConsoleServerPortsApi {
        Resource::new(self.client.clone(), "dcim/console-server-ports/")
    }

    /// trace a console server port path by id.
    pub async fn console_server_port_trace(
        &self,
        id: &str,
    ) -> Result<crate::models::ConsoleServerPort> {
        self.client
            .get(&format!("dcim/console-server-ports/{}/trace/", id))
            .await
    }

    /// returns the controller managed device groups resource.
    pub fn controller_managed_device_groups(&self) -> ControllerManagedDeviceGroupsApi {
        Resource::new(
            self.client.clone(),
            "dcim/controller-managed-device-groups/",
        )
    }

    /// returns the controllers resource.
    pub fn controllers(&self) -> ControllersApi {
        Resource::new(self.client.clone(), "dcim/controllers/")
    }

    /// returns the device bay templates resource.
    pub fn device_bay_templates(&self) -> DeviceBayTemplatesApi {
        Resource::new(self.client.clone(), "dcim/device-bay-templates/")
    }

    /// returns the device bays resource.
    pub fn device_bays(&self) -> DeviceBaysApi {
        Resource::new(self.client.clone(), "dcim/device-bays/")
    }

    /// returns the device families resource.
    pub fn device_families(&self) -> DeviceFamiliesApi {
        Resource::new(self.client.clone(), "dcim/device-families/")
    }

    /// returns the device redundancy groups resource.
    pub fn device_redundancy_groups(&self) -> DeviceRedundancyGroupsApi {
        Resource::new(self.client.clone(), "dcim/device-redundancy-groups/")
    }

    /// returns the device types to software image files resource.
    pub fn device_types_to_software_image_files(&self) -> DeviceTypesToSoftwareImageFilesApi {
        Resource::new(
            self.client.clone(),
            "dcim/device-types-to-software-image-files/",
        )
    }

    /// returns the device types resource.
    pub fn device_types(&self) -> DeviceTypesApi {
        Resource::new(self.client.clone(), "dcim/device-types/")
    }

    /// returns the devices resource.
    pub fn devices(&self) -> DevicesApi {
        Resource::new(self.client.clone(), "dcim/devices/")
    }

    /// returns the front port templates resource.
    pub fn front_port_templates(&self) -> FrontPortTemplatesApi {
        Resource::new(self.client.clone(), "dcim/front-port-templates/")
    }

    /// returns the front ports resource.
    pub fn front_ports(&self) -> FrontPortsApi {
        Resource::new(self.client.clone(), "dcim/front-ports/")
    }

    /// returns the interface connections resource.
    pub fn interface_connections(&self) -> InterfaceConnectionsApi {
        Resource::new(self.client.clone(), "dcim/interface-connections/")
    }

    /// returns the interface redundancy group associations resource.
    pub fn interface_redundancy_group_associations(
        &self,
    ) -> InterfaceRedundancyGroupAssociationsApi {
        Resource::new(
            self.client.clone(),
            "dcim/interface-redundancy-group-associations/",
        )
    }

    /// returns the interface redundancy groups resource.
    pub fn interface_redundancy_groups(&self) -> InterfaceRedundancyGroupsApi {
        Resource::new(self.client.clone(), "dcim/interface-redundancy-groups/")
    }

    /// returns the interface templates resource.
    pub fn interface_templates(&self) -> InterfaceTemplatesApi {
        Resource::new(self.client.clone(), "dcim/interface-templates/")
    }

    /// returns the interface vdc assignments resource.
    pub fn interface_vdc_assignments(&self) -> InterfaceVdcAssignmentsApi {
        Resource::new(self.client.clone(), "dcim/interface-vdc-assignments/")
    }

    /// returns the interfaces resource.
    pub fn interfaces(&self) -> InterfacesApi {
        Resource::new(self.client.clone(), "dcim/interfaces/")
    }

    /// trace an interface path by id.
    pub async fn interface_trace(&self, id: &str) -> Result<crate::models::Interface> {
        self.client
            .get(&format!("dcim/interfaces/{}/trace/", id))
            .await
    }

    /// returns the inventory items resource.
    pub fn inventory_items(&self) -> InventoryItemsApi {
        Resource::new(self.client.clone(), "dcim/inventory-items/")
    }

    /// returns the location types resource.
    pub fn location_types(&self) -> LocationTypesApi {
        Resource::new(self.client.clone(), "dcim/location-types/")
    }

    /// returns the locations resource.
    pub fn locations(&self) -> LocationsApi {
        Resource::new(self.client.clone(), "dcim/locations/")
    }

    /// returns the manufacturers resource.
    pub fn manufacturers(&self) -> ManufacturersApi {
        Resource::new(self.client.clone(), "dcim/manufacturers/")
    }

    /// returns the module bay templates resource.
    pub fn module_bay_templates(&self) -> ModuleBayTemplatesApi {
        Resource::new(self.client.clone(), "dcim/module-bay-templates/")
    }

    /// returns the module bays resource.
    pub fn module_bays(&self) -> ModuleBaysApi {
        Resource::new(self.client.clone(), "dcim/module-bays/")
    }

    /// returns the module families resource.
    pub fn module_families(&self) -> ModuleFamiliesApi {
        Resource::new(self.client.clone(), "dcim/module-families/")
    }

    /// returns the module types resource.
    pub fn module_types(&self) -> ModuleTypesApi {
        Resource::new(self.client.clone(), "dcim/module-types/")
    }

    /// returns the modules resource.
    pub fn modules(&self) -> ModulesApi {
        Resource::new(self.client.clone(), "dcim/modules/")
    }

    /// returns the platforms resource.
    pub fn platforms(&self) -> PlatformsApi {
        Resource::new(self.client.clone(), "dcim/platforms/")
    }

    /// returns the power connections resource.
    pub fn power_connections(&self) -> PowerConnectionsApi {
        Resource::new(self.client.clone(), "dcim/power-connections/")
    }

    /// returns the power feeds resource.
    pub fn power_feeds(&self) -> PowerFeedsApi {
        Resource::new(self.client.clone(), "dcim/power-feeds/")
    }

    /// trace a power feed path by id.
    pub async fn power_feed_trace(&self, id: &str) -> Result<crate::models::PowerFeed> {
        self.client
            .get(&format!("dcim/power-feeds/{}/trace/", id))
            .await
    }

    /// returns the power outlet templates resource.
    pub fn power_outlet_templates(&self) -> PowerOutletTemplatesApi {
        Resource::new(self.client.clone(), "dcim/power-outlet-templates/")
    }

    /// returns the power outlets resource.
    pub fn power_outlets(&self) -> PowerOutletsApi {
        Resource::new(self.client.clone(), "dcim/power-outlets/")
    }

    /// trace a power outlet path by id.
    pub async fn power_outlet_trace(&self, id: &str) -> Result<crate::models::PowerOutlet> {
        self.client
            .get(&format!("dcim/power-outlets/{}/trace/", id))
            .await
    }

    /// returns the power panels resource.
    pub fn power_panels(&self) -> PowerPanelsApi {
        Resource::new(self.client.clone(), "dcim/power-panels/")
    }

    /// returns the power port templates resource.
    pub fn power_port_templates(&self) -> PowerPortTemplatesApi {
        Resource::new(self.client.clone(), "dcim/power-port-templates/")
    }

    /// returns the power ports resource.
    pub fn power_ports(&self) -> PowerPortsApi {
        Resource::new(self.client.clone(), "dcim/power-ports/")
    }

    /// trace a power port path by id.
    pub async fn power_port_trace(&self, id: &str) -> Result<crate::models::PowerPort> {
        self.client
            .get(&format!("dcim/power-ports/{}/trace/", id))
            .await
    }

    /// returns the rack groups resource.
    pub fn rack_groups(&self) -> RackGroupsApi {
        Resource::new(self.client.clone(), "dcim/rack-groups/")
    }

    /// returns the rack reservations resource.
    pub fn rack_reservations(&self) -> RackReservationsApi {
        Resource::new(self.client.clone(), "dcim/rack-reservations/")
    }

    /// returns the racks resource.
    pub fn racks(&self) -> RacksApi {
        Resource::new(self.client.clone(), "dcim/racks/")
    }

    /// returns the rear port templates resource.
    pub fn rear_port_templates(&self) -> RearPortTemplatesApi {
        Resource::new(self.client.clone(), "dcim/rear-port-templates/")
    }

    /// returns the rear ports resource.
    pub fn rear_ports(&self) -> RearPortsApi {
        Resource::new(self.client.clone(), "dcim/rear-ports/")
    }

    /// returns the software image files resource.
    pub fn software_image_files(&self) -> SoftwareImageFilesApi {
        Resource::new(self.client.clone(), "dcim/software-image-files/")
    }

    /// returns the software versions resource.
    pub fn software_versions(&self) -> SoftwareVersionsApi {
        Resource::new(self.client.clone(), "dcim/software-versions/")
    }

    /// returns the virtual chassis resource.
    pub fn virtual_chassis(&self) -> VirtualChassisApi {
        Resource::new(self.client.clone(), "dcim/virtual-chassis/")
    }

    /// returns the virtual device contexts resource.
    pub fn virtual_device_contexts(&self) -> VirtualDeviceContextsApi {
        Resource::new(self.client.clone(), "dcim/virtual-device-contexts/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use httpmock::{Method::GET, MockServer};
    use serde_json::json;

    fn test_client() -> Client {
        let config = ClientConfig::new("https://nautobot.example.com", "token");
        Client::new(config).unwrap()
    }

    fn assert_path<T>(resource: Resource<T>, expected: &str)
    where
        T: serde::de::DeserializeOwned,
    {
        let paginator = resource.paginate(None).unwrap();
        assert_eq!(paginator.next_url(), Some(expected));
    }

    #[test]
    fn dcim_accessors_return_expected_paths() {
        let api = DcimApi::new(test_client());

        assert_path(api.cables(), "dcim/cables/");
        assert_path(api.console_connections(), "dcim/console-connections/");
        assert_path(api.console_port_templates(), "dcim/console-port-templates/");
        assert_path(api.console_ports(), "dcim/console-ports/");
        assert_path(
            api.console_server_port_templates(),
            "dcim/console-server-port-templates/",
        );
        assert_path(api.console_server_ports(), "dcim/console-server-ports/");
        assert_path(
            api.controller_managed_device_groups(),
            "dcim/controller-managed-device-groups/",
        );
        assert_path(api.controllers(), "dcim/controllers/");
        assert_path(api.device_bay_templates(), "dcim/device-bay-templates/");
        assert_path(api.device_bays(), "dcim/device-bays/");
        assert_path(api.device_families(), "dcim/device-families/");
        assert_path(
            api.device_redundancy_groups(),
            "dcim/device-redundancy-groups/",
        );
        assert_path(
            api.device_types_to_software_image_files(),
            "dcim/device-types-to-software-image-files/",
        );
        assert_path(api.device_types(), "dcim/device-types/");
        assert_path(api.devices(), "dcim/devices/");
        assert_path(api.front_port_templates(), "dcim/front-port-templates/");
        assert_path(api.front_ports(), "dcim/front-ports/");
        assert_path(api.interface_connections(), "dcim/interface-connections/");
        assert_path(
            api.interface_redundancy_group_associations(),
            "dcim/interface-redundancy-group-associations/",
        );
        assert_path(
            api.interface_redundancy_groups(),
            "dcim/interface-redundancy-groups/",
        );
        assert_path(api.interface_templates(), "dcim/interface-templates/");
        assert_path(
            api.interface_vdc_assignments(),
            "dcim/interface-vdc-assignments/",
        );
        assert_path(api.interfaces(), "dcim/interfaces/");
        assert_path(api.inventory_items(), "dcim/inventory-items/");
        assert_path(api.location_types(), "dcim/location-types/");
        assert_path(api.locations(), "dcim/locations/");
        assert_path(api.manufacturers(), "dcim/manufacturers/");
        assert_path(api.module_bay_templates(), "dcim/module-bay-templates/");
        assert_path(api.module_bays(), "dcim/module-bays/");
        assert_path(api.module_families(), "dcim/module-families/");
        assert_path(api.module_types(), "dcim/module-types/");
        assert_path(api.modules(), "dcim/modules/");
        assert_path(api.platforms(), "dcim/platforms/");
        assert_path(api.power_connections(), "dcim/power-connections/");
        assert_path(api.power_feeds(), "dcim/power-feeds/");
        assert_path(
            api.power_outlet_templates(),
            "dcim/power-outlet-templates/",
        );
        assert_path(api.power_outlets(), "dcim/power-outlets/");
        assert_path(api.power_panels(), "dcim/power-panels/");
        assert_path(api.power_port_templates(), "dcim/power-port-templates/");
        assert_path(api.power_ports(), "dcim/power-ports/");
        assert_path(api.rack_groups(), "dcim/rack-groups/");
        assert_path(api.rack_reservations(), "dcim/rack-reservations/");
        assert_path(api.racks(), "dcim/racks/");
        assert_path(api.rear_port_templates(), "dcim/rear-port-templates/");
        assert_path(api.rear_ports(), "dcim/rear-ports/");
        assert_path(api.software_image_files(), "dcim/software-image-files/");
        assert_path(api.software_versions(), "dcim/software-versions/");
        assert_path(api.virtual_chassis(), "dcim/virtual-chassis/");
        assert_path(
            api.virtual_device_contexts(),
            "dcim/virtual-device-contexts/",
        );
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn dcim_connected_device_and_traces_hit_expected_paths() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = DcimApi::new(client);

        server.mock(|when, then| {
            when.method(GET)
                .path("/api/dcim/connected-device/")
                .query_param("peer_device", "dev1")
                .query_param("peer_interface", "eth0");
            then.status(200).json_body(json!([]));
        });

        server.mock(|when, then| {
            when.method(GET).path("/api/dcim/console-ports/1/trace/");
            then.status(200).json_body(json!({"name": "console-1"}));
        });
        server.mock(|when, then| {
            when.method(GET)
                .path("/api/dcim/console-server-ports/1/trace/");
            then.status(200).json_body(json!({"name": "cs-1"}));
        });
        server.mock(|when, then| {
            when.method(GET).path("/api/dcim/interfaces/1/trace/");
            then.status(200).json_body(json!({
                "name": "eth0",
                "type": {},
                "status": {}
            }));
        });
        server.mock(|when, then| {
            when.method(GET).path("/api/dcim/power-feeds/1/trace/");
            then.status(200).json_body(json!({
                "name": "feed-1",
                "power_panel": {},
                "status": {}
            }));
        });
        server.mock(|when, then| {
            when.method(GET).path("/api/dcim/power-outlets/1/trace/");
            then.status(200).json_body(json!({"name": "outlet-1"}));
        });
        server.mock(|when, then| {
            when.method(GET).path("/api/dcim/power-ports/1/trace/");
            then.status(200).json_body(json!({"name": "port-1"}));
        });

        let connected = api.connected_device("dev1", "eth0", None).await.unwrap();
        assert!(connected.is_empty());

        let _ = api.console_port_trace("1").await.unwrap();
        let _ = api.console_server_port_trace("1").await.unwrap();
        let _ = api.interface_trace("1").await.unwrap();
        let _ = api.power_feed_trace("1").await.unwrap();
        let _ = api.power_outlet_trace("1").await.unwrap();
        let _ = api.power_port_trace("1").await.unwrap();
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn dcim_assignment_resources_hit_expected_paths() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = DcimApi::new(client);
        let list_response = json!({
            "count": 0,
            "next": null,
            "previous": null,
            "results": []
        });

        server.mock(|when, then| {
            when.method(GET)
                .path("/api/dcim/controller-managed-device-groups/");
            then.status(200).json_body(list_response.clone());
        });
        server.mock(|when, then| {
            when.method(GET)
                .path("/api/dcim/device-types-to-software-image-files/");
            then.status(200).json_body(list_response.clone());
        });
        server.mock(|when, then| {
            when.method(GET)
                .path("/api/dcim/interface-redundancy-group-associations/");
            then.status(200).json_body(list_response.clone());
        });

        let controller_groups = api
            .controller_managed_device_groups()
            .list(None)
            .await
            .unwrap();
        assert!(controller_groups.results.is_empty());

        let device_types_to_software = api
            .device_types_to_software_image_files()
            .list(None)
            .await
            .unwrap();
        assert!(device_types_to_software.results.is_empty());

        let interface_associations = api
            .interface_redundancy_group_associations()
            .list(None)
            .await
            .unwrap();
        assert!(interface_associations.results.is_empty());
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn dcim_special_endpoints_hit_expected_paths() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = DcimApi::new(client);
        let list_response = json!({
            "count": 0,
            "next": null,
            "previous": null,
            "results": []
        });

        server.mock(|when, then| {
            when.method(GET).path("/api/dcim/devices/1/napalm/");
            then.status(200).json_body(json!({ "method": {} }));
        });
        server.mock(|when, then| {
            when.method(GET).path("/api/dcim/front-ports/1/paths/");
            then.status(200).json_body(list_response.clone());
        });
        server.mock(|when, then| {
            when.method(GET).path("/api/dcim/rear-ports/1/paths/");
            then.status(200).json_body(list_response.clone());
        });
        server.mock(|when, then| {
            when.method(GET).path("/api/dcim/racks/1/elevation/");
            then.status(200).json_body(list_response.clone());
        });

        let napalm = api.device_napalm("1").await.unwrap();
        assert!(napalm.method.is_empty());

        let front_paths = api.front_port_paths("1", None).await.unwrap();
        assert!(front_paths.results.is_empty());

        let rear_paths = api.rear_port_paths("1", None).await.unwrap();
        assert!(rear_paths.results.is_empty());

        let elevation = api.rack_elevation("1", None).await.unwrap();
        assert!(elevation.results.is_empty());
    }
}
