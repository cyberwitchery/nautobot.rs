use clap::ValueEnum;
use comfy_table::{Cell, ContentArrangement, Table};
use reqwest::Method;
use serde_json::{Value, to_string_pretty};
use terminal_size::{Width, terminal_size};

use super::util::append_query;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OutputFormat {
    Json,
    Yaml,
    Table,
}

#[derive(Clone, Debug)]
pub struct OutputConfig {
    pub format: OutputFormat,
    pub select: Option<String>,
    pub columns: Option<Vec<String>>,
    pub max_columns: usize,
    pub dry_run: bool,
}

pub fn print_output(value: &Value, output: &OutputConfig) -> Result<(), Box<dyn std::error::Error>> {
    let formatted = format_output(value, output)?;
    println!("{formatted}");
    Ok(())
}

pub fn format_output(
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
        OutputFormat::Table => Ok(format_table(
            &selected,
            output.columns.as_deref(),
            output.max_columns,
        )),
    }
}

pub fn print_dry_run(
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

pub fn dry_run_payload(method: Method, path: &str, body: Option<&Value>) -> Value {
    serde_json::json!({
        "method": method.as_str(),
        "path": path,
        "body": body,
    })
}

pub fn select_value(value: &Value, path: &str) -> Value {
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

pub fn format_table(value: &Value, columns: Option<&[String]>, max_columns: usize) -> String {
    let width = terminal_width().unwrap_or(120).min(u16::MAX as usize) as u16;
    if let Value::Object(map) = value
        && let Some(Value::Array(items)) = map.get("results")
    {
        let summary = format_table_summary(map);
        let table = table_from_items(items, width, columns, max_columns);
        return if summary.is_empty() {
            table
        } else {
            format!("{summary}\n{table}")
        };
    }

    match value {
        Value::Array(items) => table_from_items(items, width, columns, max_columns),
        Value::Object(map) => {
            let mut table = base_table(width);
            let headers: Vec<String> = if let Some(cols) = columns {
                cols.to_vec()
            } else {
                map.keys().take(max_columns).cloned().collect()
            };
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

fn table_from_items(
    items: &[Value],
    width: u16,
    columns: Option<&[String]>,
    max_columns: usize,
) -> String {
    let mut table = base_table(width);
    if let Some(Value::Object(first)) = items.first() {
        let headers = if let Some(cols) = columns {
            cols.to_vec()
        } else {
            infer_columns(items, first, max_columns)
        };
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
    } else if let Some(cols) = columns {
        // empty result set with explicit columns: render the headers anyway.
        table.set_header(cols.iter().map(Cell::new));
    } else {
        table.set_header(vec![Cell::new("value")]);
        for item in items {
            table.add_row(vec![Cell::new(value_to_cell(Some(item)))]);
        }
    }
    table.to_string()
}

fn infer_columns(
    items: &[Value],
    first: &serde_json::Map<String, Value>,
    max_columns: usize,
) -> Vec<String> {
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
        if first.contains_key(key) && columns.len() < max_columns {
            columns.push(key.to_string());
        }
    }

    if columns.is_empty() {
        columns = first.keys().take(max_columns).cloned().collect();
    }

    if columns.len() < max_columns {
        let mut additional = first
            .keys()
            .filter(|key| !columns.iter().any(|col| col == *key))
            .take(max_columns - columns.len())
            .cloned()
            .collect::<Vec<_>>();
        columns.append(&mut additional);
    }

    if columns.len() > max_columns {
        columns.truncate(max_columns);
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
        let end = raw.floor_char_boundary(117);
        format!("{}...", &raw[..end])
    } else {
        raw
    }
}
