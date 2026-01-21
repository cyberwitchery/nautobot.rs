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
    /// Fetch Nautobot status
    Status,
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

#[derive(Args, Debug)]
struct JsonInputOptional {
    /// JSON payload string
    #[arg(long)]
    json: Option<String>,
    /// JSON payload file path
    #[arg(long)]
    file: Option<PathBuf>,
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
        Commands::Status => {
            let response = request_raw_with_context(&api, Method::GET, "status/", None).await?;
            print_output(&response, &output)?;
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
        Some(Value::Array(items)) => format!(
            "[{}]
",
            items.len()
        ),
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
