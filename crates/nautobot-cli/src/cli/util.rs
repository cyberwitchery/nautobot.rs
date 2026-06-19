use reqwest::Method;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt;
use std::fs;
use std::path::PathBuf;

use crate::{ApiClient, GraphqlInput, JsonInput, JsonInputOptional};

#[derive(Debug)]
pub struct RequestError {
    method: Method,
    path: String,
    source: Box<dyn std::error::Error + 'static>,
}

impl RequestError {
    pub fn new(
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

pub async fn request_raw_with_context(
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

pub fn wrap_request_error(
    method: Method,
    path: &str,
    err: Box<dyn std::error::Error + 'static>,
) -> Box<dyn std::error::Error> {
    Box::new(RequestError::new(method, path, err))
}

pub fn normalize_api_path(path: &str) -> String {
    let trimmed = path.trim_start_matches('/');
    match trimmed.strip_prefix("api/") {
        Some(stripped) => stripped.to_string(),
        None => trimmed.to_string(),
    }
}

pub fn load_json<T>(input: &JsonInput) -> Result<T, Box<dyn std::error::Error>>
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

pub fn load_json_optional<T>(
    input: &JsonInputOptional,
) -> Result<Option<T>, Box<dyn std::error::Error>>
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

pub fn load_graphql_query(input: &GraphqlInput) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(query) = &input.query {
        return Ok(query.clone());
    }
    if let Some(path) = &input.query_file {
        return Ok(fs::read_to_string(path)?);
    }
    Err("Provide --query or --query-file".into())
}

pub fn load_graphql_vars(input: &GraphqlInput) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match &input.vars {
        Some(vars) => Ok(Some(serde_json::from_str(vars)?)),
        None => Ok(None),
    }
}

pub fn append_query(path: &str, query: &[String]) -> Result<String, Box<dyn std::error::Error>> {
    let pairs = parse_query_pairs(query)?;
    if pairs.is_empty() {
        return Ok(path.to_string());
    }

    let query_string = serde_urlencoded::to_string(pairs)?;
    let separator = if path.contains('?') { "&" } else { "?" };
    Ok(format!("{}{}{}", path, separator, query_string))
}

pub fn parse_query_pairs(
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

pub fn build_schema_path(format: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    match format.unwrap_or("json") {
        "json" => Ok("swagger.json".to_string()),
        "yaml" => Ok("swagger.yaml".to_string()),
        other => Err(format!("unsupported schema format: {other}").into()),
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
