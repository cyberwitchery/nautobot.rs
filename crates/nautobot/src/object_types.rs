//! every model nautobot knows, with the rest route that serves it.
//!
//! nautobot reports its models as content types, but not their routes. a
//! route serves a model when its list response is the model's serializer,
//! which nautobot names `<Model>Serializer`. where several routes list one
//! serializer, the one that accepts creates serves it.
//!
//! basic usage:
//! ```no_run
//! # use nautobot::{Client, ClientConfig};
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = Client::new(ClientConfig::new("https://nautobot.example.com", "token"))?;
//! let types = client.object_types().await?;
//! if let Some(interfaces) = types.resource::<serde_json::Value>("dcim.interface") {
//!     let page = interfaces.list(None).await?;
//!     println!("{} interfaces", page.count);
//! }
//! let ty = types.for_url("https://nautobot.example.com/api/virtualization/interfaces/1/");
//! println!("{:?}", ty.map(|ty| ty.name()));
//! # Ok(())
//! # }
//! ```

use crate::Client;
use crate::error::Result;
use crate::resource::Resource;
use serde::Deserialize;
use serde::de::{DeserializeOwned, IgnoredAny};
use std::collections::BTreeMap;
use std::sync::Arc;

/// a nautobot model and the route that serves it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectType {
    name: String,
    app_label: String,
    model: String,
    route: Option<String>,
}

impl ObjectType {
    /// `app_label.model`, e.g. `dcim.interface`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// the django app label, e.g. `dcim`.
    pub fn app_label(&self) -> &str {
        &self.app_label
    }

    /// the django model name, e.g. `interface`.
    pub fn model(&self) -> &str {
        &self.model
    }

    /// the route under `/api/`, e.g. `dcim/interfaces/`. `None` when no route
    /// lists this model, or no single one does.
    pub fn route(&self) -> Option<&str> {
        self.route.as_deref()
    }
}

#[derive(Debug, Default)]
pub(crate) struct ObjectTypeIndex {
    by_name: BTreeMap<String, ObjectType>,
    by_route: BTreeMap<String, String>,
}

/// the object types of one nautobot, from [`Client::object_types`].
#[derive(Clone)]
pub struct ObjectTypes {
    client: Client,
    index: Arc<ObjectTypeIndex>,
}

impl ObjectTypes {
    pub(crate) fn new(client: Client, index: Arc<ObjectTypeIndex>) -> Self {
        Self { client, index }
    }

    /// the object type named `app_label.model`.
    pub fn get(&self, name: &str) -> Option<&ObjectType> {
        self.index.by_name.get(name)
    }

    /// a resource on the route of the object type named `app_label.model`,
    /// or `None` when there is no such type or it has no route.
    pub fn resource<T: DeserializeOwned>(&self, name: &str) -> Option<Resource<T>> {
        let route = self.get(name)?.route()?;
        Some(Resource::new(self.client.clone(), route.to_string()))
    }

    /// the object type whose route a url or path points into, with or
    /// without an object id, e.g. `.../api/dcim/interfaces/<id>/`.
    pub fn for_url(&self, url: &str) -> Option<&ObjectType> {
        let name = self.index.by_route.get(&route_of(url)?)?;
        self.index.by_name.get(name)
    }

    /// every object type, ordered by name.
    pub fn iter(&self) -> impl Iterator<Item = &ObjectType> {
        self.index.by_name.values()
    }
}

/// the route part of a url or path: what follows `/api/`, without a trailing
/// object id or query.
fn route_of(url: &str) -> Option<String> {
    let path = url.split(['?', '#']).next().unwrap_or_default();
    let path = match path.find("/api/") {
        Some(at) => &path[at + "/api/".len()..],
        None => path.trim_start_matches('/'),
    };
    let mut segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if segments.last().is_some_and(|last| is_object_id(last)) {
        segments.pop();
    }
    if segments.is_empty() {
        return None;
    }
    Some(format!("{}/", segments.join("/")))
}

fn is_object_id(segment: &str) -> bool {
    let hex = segment.chars().filter(char::is_ascii_hexdigit).count();
    let dashes = segment.chars().filter(|c| *c == '-').count();
    segment.chars().all(|c| c.is_ascii_digit()) || (hex == 32 && dashes == 4)
}

/// the parts of the openapi schema this reads; serde skips everything else.
#[derive(Deserialize)]
pub(crate) struct OpenApi {
    paths: BTreeMap<String, PathItem>,
}

#[derive(Deserialize)]
struct PathItem {
    get: Option<Operation>,
    post: Option<IgnoredAny>,
}

#[derive(Deserialize)]
struct Operation {
    #[serde(default)]
    responses: BTreeMap<String, Response>,
}

#[derive(Deserialize)]
struct Response {
    #[serde(default)]
    content: BTreeMap<String, Media>,
}

#[derive(Deserialize)]
struct Media {
    schema: Option<SchemaRef>,
}

#[derive(Deserialize)]
struct SchemaRef {
    #[serde(rename = "$ref")]
    reference: Option<String>,
}

impl PathItem {
    /// the serializer this route lists, from `Paginated<Serializer>List`.
    fn listed_serializer(&self) -> Option<&str> {
        let reference = self
            .get
            .as_ref()?
            .responses
            .get("200")?
            .content
            .get("application/json")?
            .schema
            .as_ref()?
            .reference
            .as_deref()?;
        reference
            .rsplit('/')
            .next()?
            .strip_prefix("Paginated")?
            .strip_suffix("List")
    }
}

/// pair every content type with the route that lists its model, the one that
/// also creates it where several list it. a route two content types share a
/// model name for goes to the one whose app label it sits under; anything
/// still ambiguous gets no route.
pub(crate) fn index(schema: &OpenApi, content_types: Vec<(String, String)>) -> ObjectTypeIndex {
    let mut routes: BTreeMap<String, Vec<(&str, bool)>> = BTreeMap::new();
    for (path, item) in &schema.paths {
        if path.contains('{') {
            continue;
        }
        if let Some(serializer) = item.listed_serializer() {
            routes
                .entry(serializer.to_lowercase())
                .or_default()
                .push((path.trim_start_matches('/'), item.post.is_some()));
        }
    }

    let mut claims: BTreeMap<&str, Vec<&(String, String)>> = BTreeMap::new();
    for content_type in &content_types {
        if let Some(route) = routes.get(&content_type.1).and_then(|r| single_route(r)) {
            claims.entry(route).or_default().push(content_type);
        }
    }
    let mut owner: BTreeMap<String, &str> = BTreeMap::new();
    for (route, claimants) in claims {
        let in_app: Vec<_> = claimants
            .iter()
            .filter(|claimant| route.split('/').next() == Some(claimant.0.as_str()))
            .collect();
        let winner = match (claimants.as_slice(), in_app.as_slice()) {
            ([only], _) => *only,
            (_, [only]) => **only,
            _ => continue,
        };
        owner.insert(format!("{}.{}", winner.0, winner.1), route);
    }

    let mut index = ObjectTypeIndex::default();
    for (app_label, model) in content_types {
        let name = format!("{app_label}.{model}");
        let route = owner.get(&name).map(|route| route.to_string());
        if let Some(route) = &route {
            index.by_route.insert(route.clone(), name.clone());
        }
        index.by_name.insert(
            name.clone(),
            ObjectType {
                name,
                app_label,
                model,
                route,
            },
        );
    }
    index
}

/// the one route listing a serializer, or of several, the one writable one.
fn single_route<'a>(routes: &[(&'a str, bool)]) -> Option<&'a str> {
    if let [(route, _)] = routes {
        return Some(route);
    }
    let mut writable = routes.iter().filter(|(_, writable)| *writable);
    match (writable.next(), writable.next()) {
        (Some((route, _)), None) => Some(route),
        _ => None,
    }
}

/// fetch the openapi schema and the content types, and pair them.
pub(crate) async fn fetch(client: &Client) -> Result<ObjectTypeIndex> {
    let schema: OpenApi = client.get("swagger.json").await?;
    let content_types = client
        .extras()
        .content_types()
        .paginate(None)?
        .collect_all()
        .await?
        .into_iter()
        .map(|content_type| (content_type.app_label, content_type.model))
        .collect();
    Ok(index(&schema, content_types))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn list(serializer: &str, writable: bool) -> serde_json::Value {
        let get = json!({"responses": {"200": {"content": {"application/json": {
            "schema": {"$ref": format!("#/components/schemas/Paginated{serializer}List")}
        }}}}});
        if writable {
            json!({"get": get, "post": {}})
        } else {
            json!({"get": get})
        }
    }

    fn index_of(paths: serde_json::Value, content_types: &[(&str, &str)]) -> ObjectTypeIndex {
        let schema: OpenApi = serde_json::from_value(json!({"paths": paths})).unwrap();
        let content_types = content_types
            .iter()
            .map(|(app, model)| (app.to_string(), model.to_string()))
            .collect();
        index(&schema, content_types)
    }

    fn route<'a>(index: &'a ObjectTypeIndex, name: &str) -> Option<&'a str> {
        index.by_name.get(name)?.route()
    }

    #[test]
    fn a_model_takes_the_route_listing_its_serializer() {
        let index = index_of(
            json!({
                "/dcim/location-types/": list("LocationType", true),
                "/virtualization/interfaces/": list("VMInterface", true),
                "/users/groups/": list("Group", true),
            }),
            &[
                ("dcim", "locationtype"),
                ("virtualization", "vminterface"),
                ("auth", "group"),
                ("users", "admingroup"),
            ],
        );

        assert_eq!(
            route(&index, "dcim.locationtype"),
            Some("dcim/location-types/")
        );
        assert_eq!(
            route(&index, "virtualization.vminterface"),
            Some("virtualization/interfaces/")
        );
        // the route sits under another app than the model's.
        assert_eq!(route(&index, "auth.group"), Some("users/groups/"));
        assert_eq!(route(&index, "users.admingroup"), None);
    }

    /// `console-connections` lists console ports but creates nothing.
    #[test]
    fn of_several_routes_the_writable_one_serves_the_model() {
        let index = index_of(
            json!({
                "/dcim/console-connections/": list("ConsolePort", false),
                "/dcim/console-ports/": list("ConsolePort", true),
                "/dcim/console-ports/{id}/": list("ConsolePort", true),
            }),
            &[("dcim", "consoleport")],
        );

        assert_eq!(
            route(&index, "dcim.consoleport"),
            Some("dcim/console-ports/")
        );
    }

    /// job results are only ever listed, never created through the api.
    #[test]
    fn a_read_only_route_alone_serves_its_model() {
        let index = index_of(
            json!({"/extras/job-results/": list("JobResult", false)}),
            &[("extras", "jobresult")],
        );

        assert_eq!(
            route(&index, "extras.jobresult"),
            Some("extras/job-results/")
        );
    }

    #[test]
    fn a_shared_model_name_goes_to_the_app_the_route_sits_under() {
        let index = index_of(
            json!({"/extras/tags/": list("Tag", true)}),
            &[("extras", "tag"), ("taggit", "tag")],
        );

        assert_eq!(route(&index, "extras.tag"), Some("extras/tags/"));
        assert_eq!(route(&index, "taggit.tag"), None);
    }

    #[test]
    fn a_model_two_routes_serve_has_no_route() {
        let index = index_of(
            json!({
                "/dcim/widgets/": list("Widget", true),
                "/ipam/widgets/": list("Widget", true),
            }),
            &[("dcim", "widget")],
        );

        assert_eq!(route(&index, "dcim.widget"), None);
        assert!(index.by_name.contains_key("dcim.widget"));
    }

    #[test]
    fn route_of_drops_the_base_the_id_and_the_query() {
        for (url, expected) in [
            (
                "https://nautobot.example.com/api/dcim/interfaces/1f0c4a2e-8d3b-4c5e-9f6a-7b8c9d0e1f2a/",
                Some("dcim/interfaces/"),
            ),
            ("/api/dcim/interfaces/?limit=1", Some("dcim/interfaces/")),
            ("dcim/interfaces/42/", Some("dcim/interfaces/")),
            ("dcim/interfaces", Some("dcim/interfaces/")),
            ("https://nautobot.example.com/api/", None),
        ] {
            assert_eq!(route_of(url).as_deref(), expected, "{url}");
        }
    }
}
