//! users endpoints.
//!
//! basic usage:
//! ```no_run
//! # use nautobot::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
//! let items = client.users().groups().list(None).await?;
//! println!("{}", items.count);
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::error::Result;
use crate::resource::Resource;
use serde_json::Value;

/// Group model.
pub type Group = crate::models::Group;
/// ObjectPermission model.
pub type ObjectPermission = crate::models::ObjectPermission;
/// Token model.
pub type Token = crate::models::Token;
/// User model.
pub type User = crate::models::User;

/// resource for users/groups/.
pub type GroupsApi = Resource<crate::models::Group>;
/// resource for users/permissions/.
pub type PermissionsApi = Resource<crate::models::ObjectPermission>;
/// resource for users/tokens/.
pub type TokensApi = Resource<crate::models::Token>;
/// resource for users/users/.
pub type UsersResource = Resource<crate::models::User>;

/// api for users endpoints.
#[derive(Clone)]
pub struct UsersApi {
    client: Client,
}

impl UsersApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// fetch user configuration settings.
    pub async fn config(&self) -> Result<Value> {
        self.client.get("users/config/").await
    }

    /// returns the groups resource.
    pub fn groups(&self) -> GroupsApi {
        Resource::new(self.client.clone(), "users/groups/")
    }

    /// returns the permissions resource.
    pub fn permissions(&self) -> PermissionsApi {
        Resource::new(self.client.clone(), "users/permissions/")
    }

    /// returns the tokens resource.
    pub fn tokens(&self) -> TokensApi {
        Resource::new(self.client.clone(), "users/tokens/")
    }

    /// returns the users resource.
    pub fn users(&self) -> UsersResource {
        Resource::new(self.client.clone(), "users/users/")
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
    fn users_accessors_return_expected_paths() {
        let api = UsersApi::new(test_client());

        assert_path(api.groups(), "users/groups/");
        assert_path(api.permissions(), "users/permissions/");
        assert_path(api.tokens(), "users/tokens/");
        assert_path(api.users(), "users/users/");
    }

    #[cfg_attr(miri, ignore)]
    #[tokio::test]
    async fn users_config_hits_expected_path() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = Client::new(config).unwrap();
        let api = UsersApi::new(client);

        server.mock(|when, then| {
            when.method(GET).path("/api/users/config/");
            then.status(200).json_body(json!({ "theme": "light" }));
        });

        let value = api.config().await.unwrap();
        assert_eq!(value.get("theme"), Some(&json!("light")));
    }
}
