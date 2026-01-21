//! virtualization endpoints.
//!
//! basic usage:
//! ```no_run
//! # use nautobot::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
//! let items = client.virtualization().cluster_groups().list(None).await?;
//! println!("{}", items.count);
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::resource::Resource;

/// ClusterGroup model.
pub type ClusterGroup = crate::models::ClusterGroup;
/// ClusterType model.
pub type ClusterType = crate::models::ClusterType;
/// Cluster model.
pub type Cluster = crate::models::Cluster;
/// VmInterface model.
pub type VmInterface = crate::models::VmInterface;
/// VirtualMachine model.
pub type VirtualMachine = crate::models::VirtualMachine;

/// resource for virtualization/cluster-groups/.
pub type ClusterGroupsApi = Resource<crate::models::ClusterGroup>;
/// resource for virtualization/cluster-types/.
pub type ClusterTypesApi = Resource<crate::models::ClusterType>;
/// resource for virtualization/clusters/.
pub type ClustersApi = Resource<crate::models::Cluster>;
/// resource for virtualization/interfaces/.
pub type InterfacesApi = Resource<crate::models::VmInterface>;
/// resource for virtualization/virtual-machines/.
pub type VirtualMachinesApi = Resource<crate::models::VirtualMachine>;

/// api for virtualization endpoints.
#[derive(Clone)]
pub struct VirtualizationApi {
    client: Client,
}

impl VirtualizationApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// returns the cluster groups resource.
    pub fn cluster_groups(&self) -> ClusterGroupsApi {
        Resource::new(self.client.clone(), "virtualization/cluster-groups/")
    }

    /// returns the cluster types resource.
    pub fn cluster_types(&self) -> ClusterTypesApi {
        Resource::new(self.client.clone(), "virtualization/cluster-types/")
    }

    /// returns the clusters resource.
    pub fn clusters(&self) -> ClustersApi {
        Resource::new(self.client.clone(), "virtualization/clusters/")
    }

    /// returns the interfaces resource.
    pub fn interfaces(&self) -> InterfacesApi {
        Resource::new(self.client.clone(), "virtualization/interfaces/")
    }

    /// returns the virtual machines resource.
    pub fn virtual_machines(&self) -> VirtualMachinesApi {
        Resource::new(self.client.clone(), "virtualization/virtual-machines/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;

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
    fn virtualization_accessors_return_expected_paths() {
        let api = VirtualizationApi::new(test_client());

        assert_path(api.cluster_groups(), "virtualization/cluster-groups/");
        assert_path(api.cluster_types(), "virtualization/cluster-types/");
        assert_path(api.clusters(), "virtualization/clusters/");
        assert_path(api.interfaces(), "virtualization/interfaces/");
        assert_path(api.virtual_machines(), "virtualization/virtual-machines/");
    }
}
