//! tenancy endpoints.
//!
//! basic usage:
//! ```no_run
//! # use nautobot::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
//! let items = client.tenancy().tenant_groups().list(None).await?;
//! println!("{}", items.count);
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::resource::Resource;

/// TenantGroup model.
pub type TenantGroup = crate::models::TenantGroup;
/// Tenant model.
pub type Tenant = crate::models::Tenant;

/// resource for tenancy/tenant-groups/.
pub type TenantGroupsApi = Resource<crate::models::TenantGroup>;
/// resource for tenancy/tenants/.
pub type TenantsApi = Resource<crate::models::Tenant>;

/// api for tenancy endpoints.
#[derive(Clone)]
pub struct TenancyApi {
    client: Client,
}

impl TenancyApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// returns the tenant groups resource.
    pub fn tenant_groups(&self) -> TenantGroupsApi {
        Resource::new(self.client.clone(), "tenancy/tenant-groups/")
    }

    /// returns the tenants resource.
    pub fn tenants(&self) -> TenantsApi {
        Resource::new(self.client.clone(), "tenancy/tenants/")
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
    fn tenancy_accessors_return_expected_paths() {
        let api = TenancyApi::new(test_client());

        assert_path(api.tenant_groups(), "tenancy/tenant-groups/");
        assert_path(api.tenants(), "tenancy/tenants/");
    }
}
