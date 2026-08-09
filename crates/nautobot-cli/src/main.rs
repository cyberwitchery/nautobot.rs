#![doc = include_str!("../docs/cli.md")]

mod cli;
mod config;

use clap::{Args, Parser, Subcommand};
use cli::{
    CIRCUITS_RESOURCES, CLOUD_RESOURCES, DCIM_RESOURCES, EXTRAS_RESOURCES, IPAM_RESOURCES,
    OutputConfig, OutputFormat, TENANCY_RESOURCES, USERS_RESOURCES, VIRTUALIZATION_RESOURCES,
    WIRELESS_RESOURCES, append_query, build_schema_path, handle_config_command,
    handle_resource_group, load_graphql_query, load_graphql_vars, load_json_optional,
    normalize_api_path, print_dry_run, print_output, print_resources, request_raw_with_context,
    wrap_request_error,
};
use config::{Profile, load_config};
use nautobot::{Client, ClientConfig};
use reqwest::Method;
use serde_json::Value;
use std::path::PathBuf;
use std::time::Duration;

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

#[derive(Parser)]
#[command(name = "nautobot-cli")]
#[command(about = "cli client for the nautobot api", long_about = None)]
struct Cli {
    /// Nautobot instance URL (overrides config file)
    #[arg(short, long, env = "NAUTOBOT_URL")]
    url: Option<String>,

    /// API token (overrides config file)
    #[arg(short, long, env = "NAUTOBOT_TOKEN")]
    token: Option<String>,

    /// config profile to use (default: "default")
    #[arg(short, long, default_value = "default")]
    profile: String,

    /// output format (json, yaml, table)
    #[arg(long, value_enum)]
    output: Option<OutputFormat>,

    /// select a field from the response (dot path)
    #[arg(long)]
    select: Option<String>,

    /// columns to show in table output (comma-separated)
    #[arg(long, value_delimiter = ',')]
    columns: Option<Vec<String>>,

    /// maximum columns in table output (default: 6)
    #[arg(long, default_value = "6")]
    max_columns: usize,

    /// print the request and skip write operations
    #[arg(long)]
    dry_run: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum ConfigAction {
    /// show the resolved configuration for a profile
    Show,
    /// list all available profiles
    List,
    /// validate a profile configuration
    Validate,
    /// show the config file path
    Path,
}

#[derive(Subcommand)]
enum Commands {
    /// manage configuration profiles
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// list resources by group (or all resources)
    Resources {
        /// resource group name (dcim, ipam, circuits, cloud, tenancy, extras, users, virtualization, wireless)
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
    /// fetch current user config
    UsersConfig,
    /// fetch Nautobot status
    Status,
    /// fetch OpenAPI schema
    Schema {
        /// schema format (json, yaml)
        #[arg(long)]
        format: Option<String>,
    },
    /// run a read-only graphql query
    Graphql {
        #[command(flatten)]
        input: GraphqlInput,
    },
    /// find a device connected to a peer device/interface
    ConnectedDevice {
        /// peer device name
        #[arg(long)]
        peer_device: String,
        /// peer interface name
        #[arg(long)]
        peer_interface: String,
    },
    /// fetch Prometheus metrics
    Metrics,
    /// make a raw API request (covers all endpoints)
    Raw {
        /// HTTP method (GET, POST, PATCH, PUT, DELETE)
        #[arg(long)]
        method: String,
        /// API path, e.g. "dcim/devices/"
        #[arg(long)]
        path: String,
        /// query string parameters (repeatable key=value)
        #[arg(long = "query")]
        query: Vec<String>,
        #[command(flatten)]
        input: JsonInputOptional,
    },
}

#[derive(Subcommand)]
enum ResourceAction {
    /// list resources
    List {
        /// query string parameters (repeatable key=value)
        #[arg(long = "query")]
        query: Vec<String>,
    },
    /// get a resource by id (UUID string)
    Get { id: String },
    /// create a resource
    Create {
        #[command(flatten)]
        input: JsonInput,
    },
    /// update a resource (PUT)
    Update {
        id: String,
        #[command(flatten)]
        input: JsonInput,
    },
    /// patch a resource
    Patch {
        id: String,
        #[command(flatten)]
        input: JsonInput,
    },
    /// delete a resource
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

    let config_file = match load_config() {
        Ok(cf) => cf,
        Err(e) => {
            eprintln!("warning: {e}");
            None
        }
    };

    // handle config commands first (no API access needed)
    if let Commands::Config { action } = &cli.command {
        return handle_config_command(action, &cli.profile, config_file.as_ref());
    }

    let mut profile = Profile::default();
    if let Some(ref cf) = config_file {
        if let Some(p) = cf.get_profile(&cli.profile) {
            profile = p.clone();
        } else if cli.profile != "default" {
            return Err(format!("profile '{}' not found in config file", cli.profile).into());
        }
    }

    // CLI args override config
    if cli.url.is_some() {
        profile.url = cli.url.clone();
    }
    if cli.token.is_some() {
        profile.token = cli.token.clone();
    }
    if cli.output.is_some() {
        profile.output = cli.output.map(|o| format!("{:?}", o).to_lowercase());
    }

    let url = profile
        .url
        .clone()
        .ok_or("url not specified (use --url, NAUTOBOT_URL, or config file)")?;
    let token = profile.resolve_token()?.ok_or(
        "token not specified (use --token, NAUTOBOT_TOKEN, token_env, or token_command in config)",
    )?;

    let mut client_config = ClientConfig::new(&url, &token);
    if let Some(timeout) = profile.timeout {
        client_config = client_config.with_timeout(Duration::from_secs(timeout));
    }
    if let Some(retries) = profile.retries {
        client_config = client_config.with_max_retries(retries);
    }
    if let Some(ssl_verify) = profile.ssl_verify {
        client_config = client_config.with_ssl_verification(ssl_verify);
    }

    let client = Client::new(client_config)?;
    let api = NautobotApiClient { inner: client };

    let output_format = cli.output.unwrap_or_else(|| {
        profile
            .output
            .as_deref()
            .and_then(|s| match s {
                "json" => Some(OutputFormat::Json),
                "yaml" => Some(OutputFormat::Yaml),
                "table" => Some(OutputFormat::Table),
                _ => None,
            })
            .unwrap_or(OutputFormat::Json)
    });

    let output = OutputConfig {
        format: output_format,
        select: cli.select.clone(),
        columns: cli.columns.clone(),
        max_columns: cli.max_columns,
        dry_run: cli.dry_run,
    };

    match cli.command {
        Commands::Config { .. } => unreachable!(), // handled above
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

#[cfg(test)]
mod tests {
    use super::*;
    use cli::{
        DCIM_RESOURCES, RequestError, dry_run_payload, find_resource_path, format_output,
        format_table, handle_resource_action, load_json, parse_query_pairs, resource_path_with_id,
        select_value,
    };
    use serde_json::json;
    use std::env;
    use std::error::Error;
    use std::fs;
    use std::sync::{Arc, Mutex};

    fn parse_args(args: &[&str]) -> Cli {
        Cli::parse_from(args)
    }

    fn base_args() -> Vec<&'static str> {
        vec![
            "nautobot-cli",
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
            columns: None,
            max_columns: 6,
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
        path.push("nautobot-cli-test.json");
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
        path.push("nautobot-cli-graphql.graphql");
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
        let table = format_table(&value, None, 6);
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
    fn format_nautobot_error_includes_status_path_and_request_id() {
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
        let table = format_table(&value, None, 6);
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
        let action = ResourceAction::Get {
            id: "42".to_string(),
        };
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
        let action = ResourceAction::Update {
            id: "7".to_string(),
            input,
        };
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
        let action = ResourceAction::Patch {
            id: "7".to_string(),
            input,
        };
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
        let action = ResourceAction::Delete {
            id: "7".to_string(),
        };
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
                columns: None,
                max_columns: 6,
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
            select: Some("nautobot-version".to_string()),
            columns: None,
            max_columns: 6,
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

    #[test]
    fn parse_config_path_command() {
        let args = vec!["nautobot-cli", "config", "path"];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Config {
                action: ConfigAction::Path,
            } => {}
            _ => panic!("expected config path command"),
        }
    }

    #[test]
    fn parse_config_list_command() {
        let args = vec!["nautobot-cli", "config", "list"];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Config {
                action: ConfigAction::List,
            } => {}
            _ => panic!("expected config list command"),
        }
    }

    #[test]
    fn parse_config_show_command() {
        let args = vec!["nautobot-cli", "--profile", "prod", "config", "show"];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.profile, "prod");
        match cli.command {
            Commands::Config {
                action: ConfigAction::Show,
            } => {}
            _ => panic!("expected config show command"),
        }
    }

    #[test]
    fn parse_config_validate_command() {
        let args = vec!["nautobot-cli", "config", "validate"];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Config {
                action: ConfigAction::Validate,
            } => {}
            _ => panic!("expected config validate command"),
        }
    }

    #[test]
    fn parse_columns_flag() {
        let mut args = base_args();
        args.extend(["--columns", "id,name,status", "dcim", "devices", "list"]);
        let cli = Cli::parse_from(args);
        assert_eq!(
            cli.columns,
            Some(vec![
                "id".to_string(),
                "name".to_string(),
                "status".to_string()
            ])
        );
    }

    #[test]
    fn parse_max_columns_flag() {
        let mut args = base_args();
        args.extend(["--max-columns", "10", "dcim", "devices", "list"]);
        let cli = Cli::parse_from(args);
        assert_eq!(cli.max_columns, 10);
    }

    #[test]
    fn parse_max_columns_default() {
        let mut args = base_args();
        args.extend(["dcim", "devices", "list"]);
        let cli = Cli::parse_from(args);
        assert_eq!(cli.max_columns, 6);
    }

    #[test]
    fn parse_profile_flag() {
        let mut args = base_args();
        args.extend(["--profile", "prod", "status"]);
        let cli = Cli::parse_from(args);
        assert_eq!(cli.profile, "prod");
    }

    #[test]
    fn parse_profile_default() {
        let mut args = base_args();
        args.extend(["status"]);
        let cli = Cli::parse_from(args);
        assert_eq!(cli.profile, "default");
    }

    #[test]
    fn format_table_respects_explicit_columns() {
        let value = json!({
            "results": [
                {"id": 1, "name": "alpha", "status": "active", "extra": "foo"},
                {"id": 2, "name": "beta", "status": "inactive", "extra": "bar"}
            ]
        });
        let columns = vec!["name".to_string(), "extra".to_string()];
        let table = format_table(&value, Some(&columns), 6);
        assert!(table.contains("name"));
        assert!(table.contains("extra"));
        assert!(table.contains("alpha"));
        assert!(table.contains("foo"));
        // id should not be shown since explicit columns were provided
        assert!(!table.contains("| id"));
    }

    #[test]
    fn format_table_respects_max_columns() {
        let value = json!({
            "results": [
                {"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}
            ]
        });
        let table = format_table(&value, None, 2);
        let header_line = table.lines().nth(1).unwrap_or("");
        let column_count = header_line.matches('|').count() - 1;
        assert!(
            column_count <= 2,
            "expected at most 2 columns, got {}",
            column_count
        );
    }
}
