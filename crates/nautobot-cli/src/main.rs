#![doc = include_str!("../docs/cli.md")]

use clap::{Args, Parser, Subcommand, ValueEnum};
use comfy_table::{Cell, ContentArrangement, Table};
use nautobot::{Client, ClientConfig};
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde_json::{Value, to_string_pretty};
use std::fmt;
use std::fs;
use std::path::PathBuf;
use terminal_size::{Width, terminal_size};

#[async_trait::async_trait]
trait ApiClient {
    async fn request_raw(
        &self,
        method: Method,
        path: &str,
        body: Option<&Value>,
    ) -> Result<Value, Box<dyn std::error::Error>>;
    async fn graphql(
        &self,
        query: &str,
        variables: Option<&Value>,
    ) -> Result<Value, Box<dyn std::error::Error>>;
    async fn status(&self) -> Result<Value, Box<dyn std::error::Error>>;
    async fn metrics(&self) -> Result<Value, Box<dyn std::error::Error>>;
    async fn schema(
        &self,
        format: Option<&str>,
        lang: Option<&str>,
    ) -> Result<Value, Box<dyn std::error::Error>>;
}

struct NautobotApiClient {
    inner: Client,
}

#[async_trait::async_trait]
impl ApiClient for NautobotApiClient {
    async fn request_raw(
        &self,
        method: Method,
        path: &str,
        body: Option<&Value>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        Ok(self.inner.request_raw(method, path, body).await?)
    }

    async fn graphql(
        &self,
        query: &str,
        variables: Option<&Value>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let data = self
            .inner
            .graphql()
            .query(query, variables.cloned())
            .await?;
        Ok(data)
    }

    async fn status(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let status = self.inner.status().status().await?;
        Ok(serde_json::to_value(status)?)
    }

    async fn metrics(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let metrics = self.inner.metrics().metrics().await?;
        Ok(Value::String(metrics))
    }

    async fn schema(
        &self,
        format: Option<&str>,
        lang: Option<&str>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        if lang.is_some() {
            return Err("schema lang is not supported by nautobot".into());
        }
        match format.unwrap_or("json") {
            "json" => Ok(self.inner.core().swagger_json().await?),
            "yaml" => Ok(Value::String(self.inner.core().swagger_yaml().await?)),
            other => Err(format!("unsupported schema format: {other}").into()),
        }
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum OutputFormat {
    Json,
    Yaml,
    Table,
}

#[derive(Clone, Debug)]
struct OutputConfig {
    format: OutputFormat,
    select: Option<String>,
    dry_run: bool,
}

#[derive(Debug)]
struct RequestError {
    method: Method,
    path: String,
    source: Box<dyn std::error::Error + 'static>,
}

impl RequestError {
    fn new(
        method: Method,
        path: impl Into<String>,
        source: Box<dyn std::error::Error + 'static>,
    ) -> Self {
        Self {
            method,
            path: path.into(),
            source,
        }
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(message) = format_nautobot_error(&self.method, &self.path, self.source.as_ref())
        {
            return write!(f, "{message}");
        }
        write!(
            f,
            "request failed: {} {}: {}",
            self.method.as_str(),
            self.path,
            self.source
        )
    }
}

impl std::error::Error for RequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.source)
    }
}

#[derive(Clone, Copy)]
struct ResourceEntry {
    name: &'static str,
    path: &'static str,
}

const DCIM_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "cables",
        path: "dcim/cables/",
    },
    ResourceEntry {
        name: "console-connections",
        path: "dcim/console-connections/",
    },
    ResourceEntry {
        name: "console-port-templates",
        path: "dcim/console-port-templates/",
    },
    ResourceEntry {
        name: "console-ports",
        path: "dcim/console-ports/",
    },
    ResourceEntry {
        name: "console-server-port-templates",
        path: "dcim/console-server-port-templates/",
    },
    ResourceEntry {
        name: "console-server-ports",
        path: "dcim/console-server-ports/",
    },
    ResourceEntry {
        name: "controller-managed-device-groups",
        path: "dcim/controller-managed-device-groups/",
    },
    ResourceEntry {
        name: "controllers",
        path: "dcim/controllers/",
    },
    ResourceEntry {
        name: "device-bay-templates",
        path: "dcim/device-bay-templates/",
    },
    ResourceEntry {
        name: "device-bays",
        path: "dcim/device-bays/",
    },
    ResourceEntry {
        name: "device-families",
        path: "dcim/device-families/",
    },
    ResourceEntry {
        name: "device-redundancy-groups",
        path: "dcim/device-redundancy-groups/",
    },
    ResourceEntry {
        name: "device-types-to-software-image-files",
        path: "dcim/device-types-to-software-image-files/",
    },
    ResourceEntry {
        name: "device-types",
        path: "dcim/device-types/",
    },
    ResourceEntry {
        name: "devices",
        path: "dcim/devices/",
    },
    ResourceEntry {
        name: "front-port-templates",
        path: "dcim/front-port-templates/",
    },
    ResourceEntry {
        name: "front-ports",
        path: "dcim/front-ports/",
    },
    ResourceEntry {
        name: "interface-connections",
        path: "dcim/interface-connections/",
    },
    ResourceEntry {
        name: "interface-redundancy-group-associations",
        path: "dcim/interface-redundancy-group-associations/",
    },
    ResourceEntry {
        name: "interface-redundancy-groups",
        path: "dcim/interface-redundancy-groups/",
    },
    ResourceEntry {
        name: "interface-templates",
        path: "dcim/interface-templates/",
    },
    ResourceEntry {
        name: "interface-vdc-assignments",
        path: "dcim/interface-vdc-assignments/",
    },
    ResourceEntry {
        name: "interfaces",
        path: "dcim/interfaces/",
    },
    ResourceEntry {
        name: "inventory-items",
        path: "dcim/inventory-items/",
    },
    ResourceEntry {
        name: "location-types",
        path: "dcim/location-types/",
    },
    ResourceEntry {
        name: "locations",
        path: "dcim/locations/",
    },
    ResourceEntry {
        name: "manufacturers",
        path: "dcim/manufacturers/",
    },
    ResourceEntry {
        name: "module-bay-templates",
        path: "dcim/module-bay-templates/",
    },
    ResourceEntry {
        name: "module-bays",
        path: "dcim/module-bays/",
    },
    ResourceEntry {
        name: "module-families",
        path: "dcim/module-families/",
    },
    ResourceEntry {
        name: "module-types",
        path: "dcim/module-types/",
    },
    ResourceEntry {
        name: "modules",
        path: "dcim/modules/",
    },
    ResourceEntry {
        name: "platforms",
        path: "dcim/platforms/",
    },
    ResourceEntry {
        name: "power-connections",
        path: "dcim/power-connections/",
    },
    ResourceEntry {
        name: "power-feeds",
        path: "dcim/power-feeds/",
    },
    ResourceEntry {
        name: "power-outlet-templates",
        path: "dcim/power-outlet-templates/",
    },
    ResourceEntry {
        name: "power-outlets",
        path: "dcim/power-outlets/",
    },
    ResourceEntry {
        name: "power-panels",
        path: "dcim/power-panels/",
    },
    ResourceEntry {
        name: "power-port-templates",
        path: "dcim/power-port-templates/",
    },
    ResourceEntry {
        name: "power-ports",
        path: "dcim/power-ports/",
    },
    ResourceEntry {
        name: "rack-groups",
        path: "dcim/rack-groups/",
    },
    ResourceEntry {
        name: "rack-reservations",
        path: "dcim/rack-reservations/",
    },
    ResourceEntry {
        name: "racks",
        path: "dcim/racks/",
    },
    ResourceEntry {
        name: "rear-port-templates",
        path: "dcim/rear-port-templates/",
    },
    ResourceEntry {
        name: "rear-ports",
        path: "dcim/rear-ports/",
    },
    ResourceEntry {
        name: "software-image-files",
        path: "dcim/software-image-files/",
    },
    ResourceEntry {
        name: "software-versions",
        path: "dcim/software-versions/",
    },
    ResourceEntry {
        name: "virtual-chassis",
        path: "dcim/virtual-chassis/",
    },
    ResourceEntry {
        name: "virtual-device-contexts",
        path: "dcim/virtual-device-contexts/",
    },
];

const IPAM_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "ip-address-to-interface",
        path: "ipam/ip-address-to-interface/",
    },
    ResourceEntry {
        name: "ip-addresses",
        path: "ipam/ip-addresses/",
    },
    ResourceEntry {
        name: "namespaces",
        path: "ipam/namespaces/",
    },
    ResourceEntry {
        name: "prefix-location-assignments",
        path: "ipam/prefix-location-assignments/",
    },
    ResourceEntry {
        name: "prefixes",
        path: "ipam/prefixes/",
    },
    ResourceEntry {
        name: "rirs",
        path: "ipam/rirs/",
    },
    ResourceEntry {
        name: "route-targets",
        path: "ipam/route-targets/",
    },
    ResourceEntry {
        name: "services",
        path: "ipam/services/",
    },
    ResourceEntry {
        name: "vlan-groups",
        path: "ipam/vlan-groups/",
    },
    ResourceEntry {
        name: "vlan-location-assignments",
        path: "ipam/vlan-location-assignments/",
    },
    ResourceEntry {
        name: "vlans",
        path: "ipam/vlans/",
    },
    ResourceEntry {
        name: "vrf-device-assignments",
        path: "ipam/vrf-device-assignments/",
    },
    ResourceEntry {
        name: "vrf-prefix-assignments",
        path: "ipam/vrf-prefix-assignments/",
    },
    ResourceEntry {
        name: "vrfs",
        path: "ipam/vrfs/",
    },
];

const CIRCUITS_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "circuit-terminations",
        path: "circuits/circuit-terminations/",
    },
    ResourceEntry {
        name: "circuit-types",
        path: "circuits/circuit-types/",
    },
    ResourceEntry {
        name: "circuits",
        path: "circuits/circuits/",
    },
    ResourceEntry {
        name: "provider-networks",
        path: "circuits/provider-networks/",
    },
    ResourceEntry {
        name: "providers",
        path: "circuits/providers/",
    },
];

const CLOUD_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "cloud-accounts",
        path: "cloud/cloud-accounts/",
    },
    ResourceEntry {
        name: "cloud-network-prefix-assignments",
        path: "cloud/cloud-network-prefix-assignments/",
    },
    ResourceEntry {
        name: "cloud-networks",
        path: "cloud/cloud-networks/",
    },
    ResourceEntry {
        name: "cloud-resource-types",
        path: "cloud/cloud-resource-types/",
    },
    ResourceEntry {
        name: "cloud-service-network-assignments",
        path: "cloud/cloud-service-network-assignments/",
    },
    ResourceEntry {
        name: "cloud-services",
        path: "cloud/cloud-services/",
    },
];

const TENANCY_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "tenant-groups",
        path: "tenancy/tenant-groups/",
    },
    ResourceEntry {
        name: "tenants",
        path: "tenancy/tenants/",
    },
];

const EXTRAS_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "computed-fields",
        path: "extras/computed-fields/",
    },
    ResourceEntry {
        name: "config-context-schemas",
        path: "extras/config-context-schemas/",
    },
    ResourceEntry {
        name: "config-contexts",
        path: "extras/config-contexts/",
    },
    ResourceEntry {
        name: "contact-associations",
        path: "extras/contact-associations/",
    },
    ResourceEntry {
        name: "contacts",
        path: "extras/contacts/",
    },
    ResourceEntry {
        name: "content-types",
        path: "extras/content-types/",
    },
    ResourceEntry {
        name: "custom-field-choices",
        path: "extras/custom-field-choices/",
    },
    ResourceEntry {
        name: "custom-fields",
        path: "extras/custom-fields/",
    },
    ResourceEntry {
        name: "custom-links",
        path: "extras/custom-links/",
    },
    ResourceEntry {
        name: "dynamic-group-memberships",
        path: "extras/dynamic-group-memberships/",
    },
    ResourceEntry {
        name: "dynamic-groups",
        path: "extras/dynamic-groups/",
    },
    ResourceEntry {
        name: "export-templates",
        path: "extras/export-templates/",
    },
    ResourceEntry {
        name: "external-integrations",
        path: "extras/external-integrations/",
    },
    ResourceEntry {
        name: "file-proxies",
        path: "extras/file-proxies/",
    },
    ResourceEntry {
        name: "git-repositories",
        path: "extras/git-repositories/",
    },
    ResourceEntry {
        name: "graphql-queries",
        path: "extras/graphql-queries/",
    },
    ResourceEntry {
        name: "image-attachments",
        path: "extras/image-attachments/",
    },
    ResourceEntry {
        name: "job-buttons",
        path: "extras/job-buttons/",
    },
    ResourceEntry {
        name: "job-hooks",
        path: "extras/job-hooks/",
    },
    ResourceEntry {
        name: "job-logs",
        path: "extras/job-logs/",
    },
    ResourceEntry {
        name: "job-queue-assignments",
        path: "extras/job-queue-assignments/",
    },
    ResourceEntry {
        name: "job-queues",
        path: "extras/job-queues/",
    },
    ResourceEntry {
        name: "job-results",
        path: "extras/job-results/",
    },
    ResourceEntry {
        name: "jobs",
        path: "extras/jobs/",
    },
    ResourceEntry {
        name: "metadata-choices",
        path: "extras/metadata-choices/",
    },
    ResourceEntry {
        name: "metadata-types",
        path: "extras/metadata-types/",
    },
    ResourceEntry {
        name: "notes",
        path: "extras/notes/",
    },
    ResourceEntry {
        name: "object-changes",
        path: "extras/object-changes/",
    },
    ResourceEntry {
        name: "object-metadata",
        path: "extras/object-metadata/",
    },
    ResourceEntry {
        name: "relationship-associations",
        path: "extras/relationship-associations/",
    },
    ResourceEntry {
        name: "relationships",
        path: "extras/relationships/",
    },
    ResourceEntry {
        name: "roles",
        path: "extras/roles/",
    },
    ResourceEntry {
        name: "saved-views",
        path: "extras/saved-views/",
    },
    ResourceEntry {
        name: "scheduled-jobs",
        path: "extras/scheduled-jobs/",
    },
    ResourceEntry {
        name: "secrets-groups-associations",
        path: "extras/secrets-groups-associations/",
    },
    ResourceEntry {
        name: "secrets-groups",
        path: "extras/secrets-groups/",
    },
    ResourceEntry {
        name: "secrets",
        path: "extras/secrets/",
    },
    ResourceEntry {
        name: "static-group-associations",
        path: "extras/static-group-associations/",
    },
    ResourceEntry {
        name: "statuses",
        path: "extras/statuses/",
    },
    ResourceEntry {
        name: "tags",
        path: "extras/tags/",
    },
    ResourceEntry {
        name: "teams",
        path: "extras/teams/",
    },
    ResourceEntry {
        name: "user-saved-view-associations",
        path: "extras/user-saved-view-associations/",
    },
    ResourceEntry {
        name: "webhooks",
        path: "extras/webhooks/",
    },
];

const USERS_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "groups",
        path: "users/groups/",
    },
    ResourceEntry {
        name: "permissions",
        path: "users/permissions/",
    },
    ResourceEntry {
        name: "tokens",
        path: "users/tokens/",
    },
    ResourceEntry {
        name: "users",
        path: "users/users/",
    },
];

const VIRTUALIZATION_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "cluster-groups",
        path: "virtualization/cluster-groups/",
    },
    ResourceEntry {
        name: "cluster-types",
        path: "virtualization/cluster-types/",
    },
    ResourceEntry {
        name: "clusters",
        path: "virtualization/clusters/",
    },
    ResourceEntry {
        name: "interfaces",
        path: "virtualization/interfaces/",
    },
    ResourceEntry {
        name: "virtual-machines",
        path: "virtualization/virtual-machines/",
    },
];

const WIRELESS_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "controller-managed-device-group-radio-profile-assignments",
        path: "wireless/controller-managed-device-group-radio-profile-assignments/",
    },
    ResourceEntry {
        name: "controller-managed-device-group-wireless-network-assignments",
        path: "wireless/controller-managed-device-group-wireless-network-assignments/",
    },
    ResourceEntry {
        name: "radio-profiles",
        path: "wireless/radio-profiles/",
    },
    ResourceEntry {
        name: "supported-data-rates",
        path: "wireless/supported-data-rates/",
    },
    ResourceEntry {
        name: "wireless-networks",
        path: "wireless/wireless-networks/",
    },
];

#[derive(Parser)]
#[command(name = "nautobot-cli")]
#[command(about = "CLI tool for testing Nautobot API client", long_about = None)]
struct Cli {
    /// Nautobot instance URL
    #[arg(short, long, env)]
    url: String,

    /// API token
    #[arg(short, long, env)]
    token: String,

    /// Output format (json, yaml, table)
    #[arg(long, value_enum, default_value = "json")]
    output: OutputFormat,

    /// Select a field from the response (dot path)
    #[arg(long)]
    select: Option<String>,

    /// Print the request and skip write operations
    #[arg(long)]
    dry_run: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List resources by group (or all resources)
    Resources {
        /// Resource group name (dcim, ipam, circuits, cloud, tenancy, extras, users, virtualization, wireless)
        group: Option<String>,
    },
    /// DCIM resources (devices, racks, interfaces, ...)
    Dcim {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// IPAM resources (prefixes, addresses, vlans, ...)
    Ipam {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Circuits resources (providers, circuits, ...)
    Circuits {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Cloud resources (accounts, networks, services, ...)
    Cloud {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Tenancy resources (tenants, contacts, ...)
    Tenancy {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Extras resources (tags, jobs, custom fields, ...)
    Extras {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Users resources (users, groups, tokens, ...)
    Users {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Virtualization resources (clusters, vms, ...)
    Virtualization {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Wireless resources (profiles, networks, ...)
    Wireless {
        resource: String,
        #[command(subcommand)]
        action: ResourceAction,
    },
    /// Fetch current user config
    UsersConfig,
    /// Fetch Nautobot status
    Status,
    /// Fetch OpenAPI schema
    Schema {
        /// Schema format (json, yaml)
        #[arg(long)]
        format: Option<String>,
    },
    /// Run a read-only graphql query
    Graphql {
        #[command(flatten)]
        input: GraphqlInput,
    },
    /// Find a device connected to a peer device/interface
    ConnectedDevice {
        /// Peer device name
        #[arg(long)]
        peer_device: String,
        /// Peer interface name
        #[arg(long)]
        peer_interface: String,
    },
    /// Fetch Prometheus metrics
    Metrics,
    /// Make a raw API request (covers all endpoints)
    Raw {
        /// HTTP method (GET, POST, PATCH, PUT, DELETE)
        #[arg(long)]
        method: String,
        /// API path, e.g. "dcim/devices/"
        #[arg(long)]
        path: String,
        /// Query string parameters (repeatable key=value)
        #[arg(long = "query")]
        query: Vec<String>,
        #[command(flatten)]
        input: JsonInputOptional,
    },
}

#[derive(Subcommand)]
enum ResourceAction {
    /// List resources
    List {
        /// Query string parameters (repeatable key=value)
        #[arg(long = "query")]
        query: Vec<String>,
    },
    /// Get a resource by id (UUID string)
    Get { id: String },
    /// Create a resource
    Create {
        #[command(flatten)]
        input: JsonInput,
    },
    /// Update a resource (PUT)
    Update {
        id: String,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Patch a resource
    Patch {
        id: String,
        #[command(flatten)]
        input: JsonInput,
    },
    /// Delete a resource
    Delete { id: String },
}

#[derive(Args, Debug)]
struct JsonInput {
    /// JSON payload string
    #[arg(long, required_unless_present = "file")]
    json: Option<String>,
    /// JSON payload file path
    #[arg(long, required_unless_present = "json")]
    file: Option<PathBuf>,
}

#[derive(Args, Debug)]
struct JsonInputOptional {
    /// JSON payload string
    #[arg(long)]
    json: Option<String>,
    /// JSON payload file path
    #[arg(long)]
    file: Option<PathBuf>,
}

#[derive(Args, Debug)]
struct GraphqlInput {
    /// GraphQL query string
    #[arg(long, required_unless_present = "query_file")]
    query: Option<String>,
    /// GraphQL query file path
    #[arg(long, required_unless_present = "query")]
    query_file: Option<PathBuf>,
    /// JSON variables payload
    #[arg(long)]
    vars: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config = ClientConfig::new(&cli.url, &cli.token);
    let client = Client::new(config)?;
    let api = NautobotApiClient { inner: client };
    let output = OutputConfig {
        format: cli.output,
        select: cli.select.clone(),
        dry_run: cli.dry_run,
    };

    match cli.command {
        Commands::Resources { group } => {
            print_resources(group.as_deref());
        }
        Commands::Dcim { resource, action } => {
            handle_resource_group(&api, &output, "dcim", DCIM_RESOURCES, &resource, action).await?;
        }
        Commands::Ipam { resource, action } => {
            handle_resource_group(&api, &output, "ipam", IPAM_RESOURCES, &resource, action).await?;
        }
        Commands::Circuits { resource, action } => {
            handle_resource_group(
                &api,
                &output,
                "circuits",
                CIRCUITS_RESOURCES,
                &resource,
                action,
            )
            .await?;
        }
        Commands::Cloud { resource, action } => {
            handle_resource_group(&api, &output, "cloud", CLOUD_RESOURCES, &resource, action)
                .await?;
        }
        Commands::Tenancy { resource, action } => {
            handle_resource_group(
                &api,
                &output,
                "tenancy",
                TENANCY_RESOURCES,
                &resource,
                action,
            )
            .await?;
        }
        Commands::Extras { resource, action } => {
            handle_resource_group(&api, &output, "extras", EXTRAS_RESOURCES, &resource, action)
                .await?;
        }
        Commands::Users { resource, action } => {
            handle_resource_group(&api, &output, "users", USERS_RESOURCES, &resource, action)
                .await?;
        }
        Commands::Virtualization { resource, action } => {
            handle_resource_group(
                &api,
                &output,
                "virtualization",
                VIRTUALIZATION_RESOURCES,
                &resource,
                action,
            )
            .await?;
        }
        Commands::Wireless { resource, action } => {
            handle_resource_group(
                &api,
                &output,
                "wireless",
                WIRELESS_RESOURCES,
                &resource,
                action,
            )
            .await?;
        }
        Commands::UsersConfig => {
            let response =
                request_raw_with_context(&api, Method::GET, "users/config/", None).await?;
            print_output(&response, &output)?;
        }
        Commands::Status => {
            let value = api
                .status()
                .await
                .map_err(|err| wrap_request_error(Method::GET, "status/", err))?;
            print_output(&value, &output)?;
        }
        Commands::Schema { format } => {
            let schema_path = build_schema_path(format.as_deref())?;
            let value = api
                .schema(format.as_deref(), None)
                .await
                .map_err(|err| wrap_request_error(Method::GET, &schema_path, err))?;
            print_output(&value, &output)?;
        }
        Commands::Graphql { input } => {
            let query = load_graphql_query(&input)?;
            let vars = load_graphql_vars(&input)?;
            let response = api
                .graphql(&query, vars.as_ref())
                .await
                .map_err(|err| wrap_request_error(Method::POST, "graphql/", err))?;
            print_output(&response, &output)?;
        }
        Commands::ConnectedDevice {
            peer_device,
            peer_interface,
        } => {
            let path = append_query(
                "dcim/connected-device/",
                &[
                    format!("peer_device={}", peer_device),
                    format!("peer_interface={}", peer_interface),
                ],
            )?;
            let response = request_raw_with_context(&api, Method::GET, &path, None).await?;
            print_output(&response, &output)?;
        }
        Commands::Metrics => {
            let value = api
                .metrics()
                .await
                .map_err(|err| wrap_request_error(Method::GET, "metrics/", err))?;
            print_output(&value, &output)?;
        }
        Commands::Raw {
            method,
            path,
            query,
            input,
        } => {
            let method = Method::from_bytes(method.as_bytes())?;
            let body: Option<Value> = load_json_optional(&input)?;
            let path = normalize_api_path(&path);
            let full_path = append_query(&path, &query)?;
            if output.dry_run && method != Method::GET {
                print_dry_run(method, &full_path, None, body.as_ref())?;
            } else {
                let response =
                    request_raw_with_context(&api, method, &full_path, body.as_ref()).await?;
                print_output(&response, &output)?;
            }
        }
    }

    Ok(())
}

async fn handle_resource_group(
    client: &impl ApiClient,
    output: &OutputConfig,
    group: &str,
    resources: &[ResourceEntry],
    resource: &str,
    action: ResourceAction,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = find_resource_path(resources, resource).ok_or_else(|| {
        format!(
            "unknown {} resource '{}'. use `netbox-cli resources {}` to list options.",
            group, resource, group
        )
    })?;
    handle_resource_action(client, output, path, action).await
}

async fn handle_resource_action(
    client: &impl ApiClient,
    output: &OutputConfig,
    path: &str,
    action: ResourceAction,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = normalize_api_path(path);
    match action {
        ResourceAction::List { query } => {
            let full_path = append_query(&path, &query)?;
            let response = request_raw_with_context(client, Method::GET, &full_path, None).await?;
            print_output(&response, output)?;
        }
        ResourceAction::Get { id } => {
            let full_path = resource_path_with_id(&path, &id);
            let response = request_raw_with_context(client, Method::GET, &full_path, None).await?;
            print_output(&response, output)?;
        }
        ResourceAction::Create { input } => {
            let body: Value = load_json(&input)?;
            if output.dry_run {
                print_dry_run(Method::POST, &path, None, Some(&body))?;
            } else {
                let response =
                    request_raw_with_context(client, Method::POST, &path, Some(&body)).await?;
                print_output(&response, output)?;
            }
        }
        ResourceAction::Update { id, input } => {
            let body: Value = load_json(&input)?;
            let full_path = resource_path_with_id(&path, &id);
            if output.dry_run {
                print_dry_run(Method::PUT, &full_path, None, Some(&body))?;
            } else {
                let response =
                    request_raw_with_context(client, Method::PUT, &full_path, Some(&body)).await?;
                print_output(&response, output)?;
            }
        }
        ResourceAction::Patch { id, input } => {
            let body: Value = load_json(&input)?;
            let full_path = resource_path_with_id(&path, &id);
            if output.dry_run {
                print_dry_run(Method::PATCH, &full_path, None, Some(&body))?;
            } else {
                let response =
                    request_raw_with_context(client, Method::PATCH, &full_path, Some(&body))
                        .await?;
                print_output(&response, output)?;
            }
        }
        ResourceAction::Delete { id } => {
            let full_path = resource_path_with_id(&path, &id);
            if output.dry_run {
                print_dry_run(Method::DELETE, &full_path, None, None)?;
            } else {
                let response =
                    request_raw_with_context(client, Method::DELETE, &full_path, None).await?;
                if response == Value::Null {
                    println!("deleted {}", id);
                } else {
                    print_output(&response, output)?;
                }
            }
        }
    }

    Ok(())
}

fn print_resources(group: Option<&str>) {
    match group {
        None => {
            println!("dcim");
            list_resource_group(DCIM_RESOURCES);
            println!("ipam");
            list_resource_group(IPAM_RESOURCES);
            println!("circuits");
            list_resource_group(CIRCUITS_RESOURCES);
            println!("cloud");
            list_resource_group(CLOUD_RESOURCES);
            println!("tenancy");
            list_resource_group(TENANCY_RESOURCES);
            println!("extras");
            list_resource_group(EXTRAS_RESOURCES);
            println!("users");
            list_resource_group(USERS_RESOURCES);
            println!("virtualization");
            list_resource_group(VIRTUALIZATION_RESOURCES);
            println!("wireless");
            list_resource_group(WIRELESS_RESOURCES);
        }
        Some("dcim") => list_resource_group(DCIM_RESOURCES),
        Some("ipam") => list_resource_group(IPAM_RESOURCES),
        Some("circuits") => list_resource_group(CIRCUITS_RESOURCES),
        Some("cloud") => list_resource_group(CLOUD_RESOURCES),
        Some("tenancy") => list_resource_group(TENANCY_RESOURCES),
        Some("extras") => list_resource_group(EXTRAS_RESOURCES),
        Some("users") => list_resource_group(USERS_RESOURCES),
        Some("virtualization") => list_resource_group(VIRTUALIZATION_RESOURCES),
        Some("wireless") => list_resource_group(WIRELESS_RESOURCES),
        Some(other) => {
            println!("unknown group '{}'", other);
        }
    }
}

fn list_resource_group(resources: &[ResourceEntry]) {
    for entry in resources {
        println!("  {}", entry.name);
    }
}

fn find_resource_path(resources: &[ResourceEntry], name: &str) -> Option<&'static str> {
    resources
        .iter()
        .find(|entry| entry.name == name)
        .map(|entry| entry.path)
}

fn resource_path_with_id(path: &str, id: &str) -> String {
    format!("{}/{}/", path.trim_end_matches('/'), id)
}

fn normalize_api_path(path: &str) -> String {
    let trimmed = path.trim_start_matches('/');
    match trimmed.strip_prefix("api/") {
        Some(stripped) => stripped.to_string(),
        None => trimmed.to_string(),
    }
}

fn print_output(value: &Value, output: &OutputConfig) -> Result<(), Box<dyn std::error::Error>> {
    let formatted = format_output(value, output)?;
    println!("{formatted}");
    Ok(())
}

async fn request_raw_with_context(
    client: &impl ApiClient,
    method: Method,
    path: &str,
    body: Option<&Value>,
) -> Result<Value, Box<dyn std::error::Error>> {
    client
        .request_raw(method.clone(), path, body)
        .await
        .map_err(|err| wrap_request_error(method, path, err))
}

fn wrap_request_error(
    method: Method,
    path: &str,
    err: Box<dyn std::error::Error + 'static>,
) -> Box<dyn std::error::Error> {
    Box::new(RequestError::new(method, path, err))
}

fn format_output(
    value: &Value,
    output: &OutputConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let selected = match output.select.as_deref() {
        Some(path) => select_value(value, path),
        None => value.clone(),
    };

    match output.format {
        OutputFormat::Json => Ok(to_string_pretty(&selected)?),
        OutputFormat::Yaml => Ok(serde_yaml::to_string(&selected)?),
        OutputFormat::Table => Ok(format_table(&selected)),
    }
}

fn format_table(value: &Value) -> String {
    let width = terminal_width().unwrap_or(120).min(u16::MAX as usize) as u16;
    if let Value::Object(map) = value {
        if let Some(Value::Array(items)) = map.get("results") {
            let summary = format_table_summary(map);
            let table = table_from_items(items, width);
            return if summary.is_empty() {
                table
            } else {
                format!("{summary}\n{table}")
            };
        }
    }

    match value {
        Value::Array(items) => table_from_items(items, width),
        Value::Object(map) => {
            let mut table = base_table(width);
            let headers: Vec<String> = map.keys().cloned().collect();
            table.set_header(headers.iter().map(Cell::new));
            let row = headers
                .iter()
                .map(|key| Cell::new(value_to_cell(map.get(key))))
                .collect::<Vec<_>>();
            table.add_row(row);
            table.to_string()
        }
        _ => {
            let mut table = base_table(width);
            table.set_header(vec![Cell::new("value")]);
            table.add_row(vec![Cell::new(value_to_cell(Some(value)))]);
            table.to_string()
        }
    }
}

fn terminal_width() -> Option<usize> {
    terminal_size().map(|(Width(width), _)| width as usize)
}

fn value_to_cell(value: Option<&Value>) -> String {
    match value {
        Some(Value::Null) | None => "".to_string(),
        Some(Value::String(value)) => value.clone(),
        Some(Value::Number(value)) => value.to_string(),
        Some(Value::Bool(value)) => value.to_string(),
        Some(Value::Array(items)) => format!("[{}]", items.len()),
        Some(Value::Object(map)) => extract_display(map)
            .or_else(|| {
                map.get("id")
                    .and_then(Value::as_i64)
                    .map(|id| id.to_string())
            })
            .unwrap_or_else(|| compact_json(&Value::Object(map.clone()))),
    }
}

fn base_table(width: u16) -> Table {
    let mut table = Table::new();
    table
        .load_preset(comfy_table::presets::ASCII_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(width);
    table
}

fn table_from_items(items: &[Value], width: u16) -> String {
    let mut table = base_table(width);
    if let Some(Value::Object(first)) = items.first() {
        let headers = infer_columns(items, first);
        table.set_header(headers.iter().map(Cell::new));
        for item in items {
            if let Value::Object(map) = item {
                let row = headers
                    .iter()
                    .map(|key| Cell::new(value_to_cell(map.get(key))))
                    .collect::<Vec<_>>();
                table.add_row(row);
            } else {
                table.add_row(vec![Cell::new(value_to_cell(Some(item)))]);
            }
        }
    } else {
        table.set_header(vec![Cell::new("value")]);
        for item in items {
            table.add_row(vec![Cell::new(value_to_cell(Some(item)))]);
        }
    }
    table.to_string()
}

fn infer_columns(items: &[Value], first: &serde_json::Map<String, Value>) -> Vec<String> {
    let preferred = [
        "id",
        "name",
        "display",
        "slug",
        "status",
        "site",
        "role",
        "device_type",
        "manufacturer",
        "model",
        "url",
    ];

    let mut columns = Vec::new();
    for key in preferred {
        if first.contains_key(key) {
            columns.push(key.to_string());
        }
    }

    if columns.is_empty() {
        columns = first.keys().take(6).cloned().collect();
    }

    if columns.len() < 6 {
        let mut additional = first
            .keys()
            .filter(|key| !columns.iter().any(|col| col == *key))
            .take(6 - columns.len())
            .cloned()
            .collect::<Vec<_>>();
        columns.append(&mut additional);
    }

    if columns.len() > 6 {
        columns.truncate(6);
    }

    if columns.len() > 1 && items.iter().any(|item| matches!(item, Value::Object(_))) {
        columns
    } else {
        vec!["value".to_string()]
    }
}

fn format_table_summary(map: &serde_json::Map<String, Value>) -> String {
    let count = map
        .get("count")
        .and_then(Value::as_i64)
        .map(|v| v.to_string());
    let next = map
        .get("next")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let previous = map
        .get("previous")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();

    let mut parts = Vec::new();
    if let Some(count) = count {
        parts.push(format!("count: {count}"));
    }
    if !next.is_empty() {
        parts.push(format!("next: {next}"));
    }
    if !previous.is_empty() {
        parts.push(format!("previous: {previous}"));
    }
    parts.join(" | ")
}

fn extract_display(map: &serde_json::Map<String, Value>) -> Option<String> {
    for key in ["display", "name", "label", "value", "slug"] {
        if let Some(Value::String(value)) = map.get(key) {
            return Some(value.clone());
        }
    }
    None
}

fn compact_json(value: &Value) -> String {
    let raw = serde_json::to_string(value).unwrap_or_else(|_| "<invalid>".to_string());
    if raw.len() > 120 {
        format!("{}...", &raw[..117])
    } else {
        raw
    }
}

fn format_nautobot_error(
    method: &Method,
    path: &str,
    err: &(dyn std::error::Error + 'static),
) -> Option<String> {
    let nautobot_err = err.downcast_ref::<nautobot::Error>()?;
    let nautobot::Error::ApiError {
        status,
        message,
        body,
    } = nautobot_err
    else {
        return None;
    };

    let mut detail = format!("status {}", status);
    if let Some(request_id) = extract_request_id(body) {
        detail.push_str(&format!(", request_id {request_id}"));
    }
    let mut summary = format!("request failed: {} {} ({detail})", method.as_str(), path);
    if !message.is_empty() {
        summary.push_str(": ");
        summary.push_str(message);
    }
    Some(summary)
}

fn extract_request_id(body: &str) -> Option<String> {
    let value: Value = serde_json::from_str(body).ok()?;
    for key in ["request_id", "requestId", "request-id"] {
        if let Some(Value::String(id)) = value.get(key) {
            return Some(id.clone());
        }
    }
    None
}

fn select_value(value: &Value, path: &str) -> Value {
    let segments: Vec<&str> = path.split('.').filter(|seg| !seg.is_empty()).collect();
    select_value_segments(value, &segments)
}

fn select_value_segments(value: &Value, segments: &[&str]) -> Value {
    if segments.is_empty() {
        return value.clone();
    }

    match value {
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|item| select_value_segments(item, segments))
                .collect(),
        ),
        Value::Object(map) => map
            .get(segments[0])
            .map(|next| select_value_segments(next, &segments[1..]))
            .unwrap_or(Value::Null),
        _ => Value::Null,
    }
}

fn print_dry_run(
    method: Method,
    path: &str,
    query: Option<&[String]>,
    body: Option<&Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let full_path = match query {
        Some(query) => append_query(path, query)?,
        None => path.to_string(),
    };
    let payload = dry_run_payload(method, &full_path, body);
    println!("{}", to_string_pretty(&payload)?);
    Ok(())
}

fn dry_run_payload(method: Method, path: &str, body: Option<&Value>) -> Value {
    serde_json::json!({
        "method": method.as_str(),
        "path": path,
        "body": body,
    })
}

fn load_json<T>(input: &JsonInput) -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let content = if let Some(json) = &input.json {
        json.clone()
    } else if let Some(path) = &input.file {
        fs::read_to_string(path)?
    } else {
        return Err("Provide --json or --file".into());
    };

    Ok(serde_json::from_str(&content)?)
}

fn load_json_optional<T>(input: &JsonInputOptional) -> Result<Option<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let content = if let Some(json) = &input.json {
        Some(json.clone())
    } else if let Some(path) = &input.file {
        Some(fs::read_to_string(path)?)
    } else {
        None
    };

    match content {
        Some(content) => Ok(Some(serde_json::from_str(&content)?)),
        None => Ok(None),
    }
}

fn load_graphql_query(input: &GraphqlInput) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(query) = &input.query {
        return Ok(query.clone());
    }
    if let Some(path) = &input.query_file {
        return Ok(fs::read_to_string(path)?);
    }
    Err("Provide --query or --query-file".into())
}

fn load_graphql_vars(input: &GraphqlInput) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match &input.vars {
        Some(vars) => Ok(Some(serde_json::from_str(vars)?)),
        None => Ok(None),
    }
}

fn append_query(path: &str, query: &[String]) -> Result<String, Box<dyn std::error::Error>> {
    let pairs = parse_query_pairs(query)?;
    if pairs.is_empty() {
        return Ok(path.to_string());
    }

    let query_string = serde_urlencoded::to_string(pairs)?;
    let separator = if path.contains('?') { "&" } else { "?" };
    Ok(format!("{}{}{}", path, separator, query_string))
}

fn parse_query_pairs(
    query: &[String],
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut pairs = Vec::with_capacity(query.len());
    for item in query {
        let mut parts = item.splitn(2, '=');
        let key = parts.next().unwrap_or_default();
        let value = parts.next();
        if key.is_empty() || value.is_none() {
            return Err(format!("Invalid query parameter: {}", item).into());
        }
        pairs.push((key.to_string(), value.unwrap().to_string()));
    }
    Ok(pairs)
}

fn build_schema_path(format: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    match format.unwrap_or("json") {
        "json" => Ok("swagger.json".to_string()),
        "yaml" => Ok("swagger.yaml".to_string()),
        other => Err(format!("unsupported schema format: {other}").into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::env;
    use std::error::Error;
    use std::sync::{Arc, Mutex};

    fn parse_args(args: &[&str]) -> Cli {
        Cli::parse_from(args)
    }

    fn base_args() -> Vec<&'static str> {
        vec![
            "netbox-cli",
            "--url",
            "http://localhost:8000",
            "--token",
            "token",
        ]
    }

    fn env_api_client() -> Result<Option<NautobotApiClient>, Box<dyn Error>> {
        let token = match std::env::var("NAUTOBOT_TOKEN") {
            Ok(token) => token,
            Err(_) => return Ok(None),
        };
        let url =
            std::env::var("NAUTOBOT_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
        let mut config = ClientConfig::new(url, token).with_max_retries(0);
        if std::env::var("NAUTOBOT_INSECURE").as_deref() == Ok("1") {
            config = config.with_ssl_verification(false);
        }
        let client = Client::new(config)?;
        Ok(Some(NautobotApiClient { inner: client }))
    }

    #[derive(Clone, Debug, PartialEq)]
    struct RecordedCall {
        method: Method,
        path: String,
        body: Option<Value>,
    }

    struct FakeApiClient {
        calls: Arc<Mutex<Vec<RecordedCall>>>,
        next: Arc<Mutex<Value>>,
    }

    impl FakeApiClient {
        fn new(response: Value) -> Self {
            Self {
                calls: Arc::new(Mutex::new(Vec::new())),
                next: Arc::new(Mutex::new(response)),
            }
        }

        fn calls(&self) -> Vec<RecordedCall> {
            self.calls.lock().unwrap().clone()
        }
    }

    struct ErrorApiClient;

    #[async_trait::async_trait]
    impl ApiClient for ErrorApiClient {
        async fn request_raw(
            &self,
            _method: Method,
            _path: &str,
            _body: Option<&Value>,
        ) -> Result<Value, Box<dyn std::error::Error>> {
            Err("api error".into())
        }

        async fn graphql(
            &self,
            _query: &str,
            _variables: Option<&Value>,
        ) -> Result<Value, Box<dyn std::error::Error>> {
            Err("api error".into())
        }

        async fn status(&self) -> Result<Value, Box<dyn std::error::Error>> {
            Err("api error".into())
        }

        async fn metrics(&self) -> Result<Value, Box<dyn std::error::Error>> {
            Err("api error".into())
        }

        async fn schema(
            &self,
            _format: Option<&str>,
            _lang: Option<&str>,
        ) -> Result<Value, Box<dyn std::error::Error>> {
            Err("api error".into())
        }
    }

    #[async_trait::async_trait]
    impl ApiClient for FakeApiClient {
        async fn request_raw(
            &self,
            method: Method,
            path: &str,
            body: Option<&Value>,
        ) -> Result<Value, Box<dyn std::error::Error>> {
            let body = body.cloned();
            self.calls.lock().unwrap().push(RecordedCall {
                method,
                path: path.to_string(),
                body,
            });
            Ok(self.next.lock().unwrap().clone())
        }

        async fn graphql(
            &self,
            _query: &str,
            _variables: Option<&Value>,
        ) -> Result<Value, Box<dyn std::error::Error>> {
            Ok(self.next.lock().unwrap().clone())
        }

        async fn status(&self) -> Result<Value, Box<dyn std::error::Error>> {
            Ok(self.next.lock().unwrap().clone())
        }

        async fn metrics(&self) -> Result<Value, Box<dyn std::error::Error>> {
            Ok(self.next.lock().unwrap().clone())
        }

        async fn schema(
            &self,
            _format: Option<&str>,
            _lang: Option<&str>,
        ) -> Result<Value, Box<dyn std::error::Error>> {
            Ok(self.next.lock().unwrap().clone())
        }
    }

    fn output_config() -> OutputConfig {
        OutputConfig {
            format: OutputFormat::Json,
            select: None,
            dry_run: false,
        }
    }

    #[test]
    fn load_json_from_inline() {
        let input = JsonInput {
            json: Some(r#"{"name":"carrier","slug":"carrier"}"#.to_string()),
            file: None,
        };
        let value: Value = load_json(&input).unwrap();
        assert_eq!(value["name"], "carrier");
        assert_eq!(value["slug"], "carrier");
    }

    #[test]
    fn load_json_from_file() {
        let mut path = env::temp_dir();
        path.push("netbox-cli-test.json");
        fs::write(&path, r#"{"name":"carrier","slug":"carrier"}"#).unwrap();

        let input = JsonInput {
            json: None,
            file: Some(path.clone()),
        };
        let value: Value = load_json(&input).unwrap();
        assert_eq!(value["name"], "carrier");
        assert_eq!(value["slug"], "carrier");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn load_json_requires_input() {
        let input = JsonInput {
            json: None,
            file: None,
        };
        let result: Result<Value, _> = load_json(&input);
        assert!(result.is_err());
    }

    #[test]
    fn load_json_rejects_invalid_json() {
        let input = JsonInput {
            json: Some("{invalid}".to_string()),
            file: None,
        };
        let result: Result<Value, _> = load_json(&input);
        assert!(result.is_err());
    }

    #[test]
    fn load_json_optional_handles_none() {
        let input = JsonInputOptional {
            json: None,
            file: None,
        };
        let value: Option<Value> = load_json_optional(&input).unwrap();
        assert!(value.is_none());
    }

    #[test]
    fn load_json_optional_rejects_invalid_json() {
        let input = JsonInputOptional {
            json: Some("{invalid}".to_string()),
            file: None,
        };
        let result: Result<Option<Value>, _> = load_json_optional(&input);
        assert!(result.is_err());
    }

    #[test]
    fn load_graphql_query_prefers_inline() {
        let input = GraphqlInput {
            query: Some("{ devices { name } }".to_string()),
            query_file: None,
            vars: None,
        };
        let query = load_graphql_query(&input).unwrap();
        assert_eq!(query, "{ devices { name } }");
    }

    #[test]
    fn load_graphql_query_reads_file() {
        let mut path = env::temp_dir();
        path.push("netbox-cli-graphql.graphql");
        fs::write(&path, "{ devices { name } }").unwrap();

        let input = GraphqlInput {
            query: None,
            query_file: Some(path.clone()),
            vars: None,
        };
        let query = load_graphql_query(&input).unwrap();
        assert_eq!(query, "{ devices { name } }");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn load_graphql_vars_parses_json() {
        let input = GraphqlInput {
            query: Some("{ devices { name } }".to_string()),
            query_file: None,
            vars: Some(r#"{"limit":5}"#.to_string()),
        };
        let vars = load_graphql_vars(&input).unwrap().unwrap();
        assert_eq!(vars["limit"], 5);
    }

    #[test]
    fn append_query_encodes_pairs() {
        let path = "dcim/devices/";
        let query = vec!["name=leaf 1".to_string(), "limit=5".to_string()];
        let full = append_query(path, &query).unwrap();
        assert_eq!(full, "dcim/devices/?name=leaf+1&limit=5");
    }

    #[test]
    fn append_query_rejects_missing_value() {
        let path = "dcim/devices/";
        let query = vec!["name".to_string()];
        let result = append_query(path, &query);
        assert!(result.is_err());
    }

    #[test]
    fn append_query_appends_when_query_present() {
        let path = "dcim/devices/?name=leaf-1";
        let query = vec!["limit=5".to_string()];
        let full = append_query(path, &query).unwrap();
        assert_eq!(full, "dcim/devices/?name=leaf-1&limit=5");
    }

    #[test]
    fn parse_query_pairs_rejects_empty_key() {
        let query = vec!["=value".to_string()];
        let result = parse_query_pairs(&query);
        assert!(result.is_err());
    }

    #[test]
    fn normalize_api_path_strips_prefix() {
        assert_eq!(normalize_api_path("api/dcim/devices/"), "dcim/devices/");
        assert_eq!(normalize_api_path("/api/dcim/devices/"), "dcim/devices/");
        assert_eq!(normalize_api_path("dcim/devices/"), "dcim/devices/");
        assert_eq!(normalize_api_path("/dcim/devices/"), "dcim/devices/");
    }

    #[test]
    fn resource_path_with_id_appends_trailing_slash() {
        let path = resource_path_with_id("dcim/devices/", "42");
        assert_eq!(path, "dcim/devices/42/");
    }

    #[test]
    fn select_value_handles_arrays() {
        let value = json!({
            "results": [
                {"name": "a"},
                {"name": "b"}
            ]
        });
        let selected = select_value(&value, "results.name");
        assert_eq!(selected, json!(["a", "b"]));
    }

    #[test]
    fn format_table_handles_objects() {
        let value = json!({"name": "leaf-1", "status": "active"});
        let table = format_table(&value);
        assert!(table.contains("name"));
        assert!(table.contains("leaf-1"));
    }

    #[test]
    fn dry_run_payload_includes_path_and_body() {
        let payload = dry_run_payload(
            Method::POST,
            "dcim/devices/",
            Some(&json!({"name":"leaf-1"})),
        );
        assert_eq!(payload["method"], "POST");
        assert_eq!(payload["path"], "dcim/devices/");
        assert_eq!(payload["body"]["name"], "leaf-1");
    }

    #[test]
    fn format_netbox_error_includes_status_path_and_request_id() {
        let body = r#"{"request_id":"req-123","detail":"bad"}"#.to_string();
        let err = nautobot::Error::ApiError {
            status: 400,
            message: "bad".to_string(),
            body,
        };
        let wrapped = RequestError::new(Method::POST, "dcim/devices/", Box::new(err));
        let message = wrapped.to_string();
        assert!(message.contains("POST"));
        assert!(message.contains("dcim/devices/"));
        assert!(message.contains("status 400"));
        assert!(message.contains("request_id req-123"));
        assert!(message.contains("bad"));
    }

    #[test]
    fn build_schema_path_returns_swagger_json() {
        let path = build_schema_path(Some("json")).unwrap();
        assert_eq!(path, "swagger.json");
    }

    #[test]
    fn build_schema_path_returns_swagger_yaml() {
        let path = build_schema_path(Some("yaml")).unwrap();
        assert_eq!(path, "swagger.yaml");
    }

    #[test]
    fn format_table_flattens_results() {
        let value = json!({
            "count": 2,
            "next": null,
            "previous": null,
            "results": [
                {"id": 1, "name": "alpha"},
                {"id": 2, "name": "beta"}
            ]
        });
        let table = format_table(&value);
        assert!(table.contains("count: 2"));
        assert!(table.contains("alpha"));
        assert!(table.contains("beta"));
    }

    #[test]
    fn find_resource_path_matches_known_resource() {
        let path = find_resource_path(DCIM_RESOURCES, "devices");
        assert_eq!(path, Some("dcim/devices/"));
        let missing = find_resource_path(DCIM_RESOURCES, "not-a-device");
        assert!(missing.is_none());
    }

    #[test]
    fn parse_resources_command_with_group() {
        let mut args = base_args();
        args.extend(["resources", "dcim"]);
        let cli = parse_args(&args);
        match cli.command {
            Commands::Resources { group } => {
                assert_eq!(group.as_deref(), Some("dcim"));
            }
            _ => panic!("expected resources command"),
        }
    }

    #[test]
    fn parse_dcim_list_command_with_query() {
        let mut args = base_args();
        args.extend([
            "dcim",
            "devices",
            "list",
            "--query",
            "name=leaf-1",
            "--query",
            "limit=5",
        ]);
        let cli = parse_args(&args);
        match cli.command {
            Commands::Dcim { resource, action } => {
                assert_eq!(resource, "devices");
                match action {
                    ResourceAction::List { query } => {
                        assert_eq!(query, vec!["name=leaf-1", "limit=5"]);
                    }
                    _ => panic!("expected list action"),
                }
            }
            _ => panic!("expected dcim command"),
        }
    }

    #[test]
    fn parse_raw_command_with_json() {
        let mut args = base_args();
        args.extend([
            "raw",
            "--method",
            "POST",
            "--path",
            "api/dcim/sites/",
            "--query",
            "name=dc1",
            "--json",
            r#"{"name":"dc1"}"#,
        ]);
        let cli = parse_args(&args);
        match cli.command {
            Commands::Raw {
                method,
                path,
                query,
                input,
            } => {
                assert_eq!(method, "POST");
                assert_eq!(path, "api/dcim/sites/");
                assert_eq!(query, vec!["name=dc1"]);
                assert!(input.json.is_some());
                assert!(input.file.is_none());
            }
            _ => panic!("expected raw command"),
        }
    }

    #[tokio::test]
    async fn handle_resource_action_list_calls_get() {
        let client = FakeApiClient::new(json!({"ok": true}));
        let action = ResourceAction::List {
            query: vec!["name=leaf-1".to_string()],
        };
        handle_resource_action(&client, &output_config(), "dcim/devices/", action)
            .await
            .unwrap();
        let calls = client.calls();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].method, Method::GET);
        assert_eq!(calls[0].path, "dcim/devices/?name=leaf-1");
        assert!(calls[0].body.is_none());
    }

    #[tokio::test]
    async fn handle_resource_action_get_calls_get() {
        let client = FakeApiClient::new(json!({"ok": true}));
        let action = ResourceAction::Get { id: "42".to_string() };
        handle_resource_action(&client, &output_config(), "dcim/devices/", action)
            .await
            .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].method, Method::GET);
        assert_eq!(calls[0].path, "dcim/devices/42/");
    }

    #[tokio::test]
    async fn handle_resource_action_create_calls_post() {
        let client = FakeApiClient::new(json!({"ok": true}));
        let input = JsonInput {
            json: Some(r#"{"name":"leaf-1"}"#.to_string()),
            file: None,
        };
        let action = ResourceAction::Create { input };
        handle_resource_action(&client, &output_config(), "dcim/devices/", action)
            .await
            .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].method, Method::POST);
        assert_eq!(calls[0].path, "dcim/devices/");
        assert_eq!(calls[0].body.as_ref().unwrap()["name"], "leaf-1");
    }

    #[tokio::test]
    async fn handle_resource_action_update_calls_put() {
        let client = FakeApiClient::new(json!({"ok": true}));
        let input = JsonInput {
            json: Some(r#"{"name":"leaf-1"}"#.to_string()),
            file: None,
        };
        let action = ResourceAction::Update { id: "7".to_string(), input };
        handle_resource_action(&client, &output_config(), "dcim/devices/", action)
            .await
            .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].method, Method::PUT);
        assert_eq!(calls[0].path, "dcim/devices/7/");
    }

    #[tokio::test]
    async fn handle_resource_action_patch_calls_patch() {
        let client = FakeApiClient::new(json!({"ok": true}));
        let input = JsonInput {
            json: Some(r#"{"name":"leaf-1"}"#.to_string()),
            file: None,
        };
        let action = ResourceAction::Patch { id: "7".to_string(), input };
        handle_resource_action(&client, &output_config(), "dcim/devices/", action)
            .await
            .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].method, Method::PATCH);
        assert_eq!(calls[0].path, "dcim/devices/7/");
    }

    #[tokio::test]
    async fn handle_resource_action_delete_calls_delete() {
        let client = FakeApiClient::new(Value::Null);
        let action = ResourceAction::Delete { id: "7".to_string() };
        handle_resource_action(&client, &output_config(), "dcim/devices/", action)
            .await
            .unwrap();
        let calls = client.calls();
        assert_eq!(calls[0].method, Method::DELETE);
        assert_eq!(calls[0].path, "dcim/devices/7/");
    }

    #[tokio::test]
    async fn handle_resource_group_unknown_resource_errors() {
        let client = FakeApiClient::new(json!({"ok": true}));
        let result = handle_resource_group(
            &client,
            &output_config(),
            "dcim",
            DCIM_RESOURCES,
            "not-a-device",
            ResourceAction::List { query: vec![] },
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn handle_resource_action_bubbles_api_error() {
        let client = ErrorApiClient;
        let action = ResourceAction::List { query: vec![] };
        let result =
            handle_resource_action(&client, &output_config(), "dcim/devices/", action).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn handle_resource_action_create_dry_run_skips_api() {
        let client = ErrorApiClient;
        let mut output = output_config();
        output.dry_run = true;
        let input = JsonInput {
            json: Some(r#"{"name":"leaf-1"}"#.to_string()),
            file: None,
        };
        let action = ResourceAction::Create { input };
        handle_resource_action(&client, &output, "dcim/devices/", action)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn smoke_status() -> Result<(), Box<dyn Error>> {
        let Some(api) = env_api_client()? else {
            eprintln!("NAUTOBOT_TOKEN not set; skipping smoke_status");
            return Ok(());
        };
        let _ = api.status().await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn smoke_list_devices() -> Result<(), Box<dyn Error>> {
        let Some(api) = env_api_client()? else {
            eprintln!("NAUTOBOT_TOKEN not set; skipping smoke_list_devices");
            return Ok(());
        };
        handle_resource_action(
            &api,
            &output_config(),
            "dcim/devices/",
            ResourceAction::List {
                query: vec!["limit=1".to_string()],
            },
        )
        .await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn smoke_output_formats() -> Result<(), Box<dyn Error>> {
        let Some(api) = env_api_client()? else {
            eprintln!("NAUTOBOT_TOKEN not set; skipping smoke_output_formats");
            return Ok(());
        };
        let status = api.status().await?;
        for format in [OutputFormat::Json, OutputFormat::Yaml, OutputFormat::Table] {
            let output = OutputConfig {
                format,
                select: None,
                dry_run: false,
            };
            let rendered = format_output(&status, &output)?;
            assert!(
                !rendered.trim().is_empty(),
                "expected output for {format:?}"
            );
        }
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn smoke_select_output() -> Result<(), Box<dyn Error>> {
        let Some(api) = env_api_client()? else {
            eprintln!("NAUTOBOT_TOKEN not set; skipping smoke_select_output");
            return Ok(());
        };
        let status = api.status().await?;
        let output = OutputConfig {
            format: OutputFormat::Json,
            select: Some("netbox-version".to_string()),
            dry_run: false,
        };
        let rendered = format_output(&status, &output)?;
        assert!(!rendered.trim().is_empty());
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn smoke_users_config() -> Result<(), Box<dyn Error>> {
        let Some(api) = env_api_client()? else {
            eprintln!("NAUTOBOT_TOKEN not set; skipping smoke_users_config");
            return Ok(());
        };
        let _ = api.request_raw(Method::GET, "users/config/", None).await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn smoke_raw_tag_roundtrip() -> Result<(), Box<dyn Error>> {
        let Some(api) = env_api_client()? else {
            eprintln!("NAUTOBOT_TOKEN not set; skipping smoke_raw_tag_roundtrip");
            return Ok(());
        };

        let name = format!(
            "cli-raw-tag-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );
        let body = json!({
            "name": name,
            "slug": name,
            "color": "9e9e9e",
            "content_types": ["dcim.device"],
        });
        let created = api
            .request_raw(Method::POST, "extras/tags/", Some(&body))
            .await?;
        let tag_id = created
            .get("id")
            .and_then(|value| value.as_str())
            .ok_or("missing tag id")?;
        let path = format!("extras/tags/{}/", tag_id);
        let _ = api.request_raw(Method::DELETE, &path, None).await?;
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn smoke_resource_crud_tag() -> Result<(), Box<dyn Error>> {
        let Some(api) = env_api_client()? else {
            eprintln!("NAUTOBOT_TOKEN not set; skipping smoke_resource_crud_tag");
            return Ok(());
        };

        let name = format!(
            "cli-resource-tag-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );
        let create = JsonInput {
            json: Some(format!(
                r#"{{"name":"{0}","slug":"{0}","color":"9e9e9e","content_types":["dcim.device"]}}"#,
                name
            )),
            file: None,
        };
        handle_resource_action(
            &api,
            &output_config(),
            "extras/tags/",
            ResourceAction::Create { input: create },
        )
        .await?;

        let list_path = format!("extras/tags/?name={}", name);
        let list = api.request_raw(Method::GET, &list_path, None).await?;
        let tag_id = list
            .get("results")
            .and_then(|value| value.as_array())
            .and_then(|results| results.first())
            .and_then(|value| value.get("id"))
            .and_then(|value| value.as_str())
            .ok_or("missing tag id")?
            .to_string();

        let update = JsonInput {
            json: Some(format!(
                r#"{{"name":"{0}-updated","slug":"{0}-updated","color":"2196f3","content_types":["dcim.device"]}}"#,
                name
            )),
            file: None,
        };
        handle_resource_action(
            &api,
            &output_config(),
            "extras/tags/",
            ResourceAction::Update {
                id: tag_id.clone(),
                input: update,
            },
        )
        .await?;

        let patch = JsonInput {
            json: Some(r#"{"description":"cli smoke test"}"#.to_string()),
            file: None,
        };
        handle_resource_action(
            &api,
            &output_config(),
            "extras/tags/",
            ResourceAction::Patch {
                id: tag_id.clone(),
                input: patch,
            },
        )
        .await?;

        handle_resource_action(
            &api,
            &output_config(),
            "extras/tags/",
            ResourceAction::Delete { id: tag_id },
        )
        .await?;
        Ok(())
    }
}
