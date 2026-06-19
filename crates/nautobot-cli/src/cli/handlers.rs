use reqwest::Method;
use serde_json::Value;

use crate::config::{ConfigFile, Profile, config_path, validate_profile};
use crate::{ApiClient, ConfigAction, JsonInput, ResourceAction};

use super::output::{OutputConfig, print_dry_run, print_output};
use super::resources::{ResourceEntry, find_resource_path, resource_path_with_id};
use super::util::{
    append_query, load_json, normalize_api_path, request_raw_with_context, wrap_request_error,
};

pub fn handle_config_command(
    action: &ConfigAction,
    profile_name: &str,
    config_file: Option<&ConfigFile>,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ConfigAction::Path => match config_path() {
            Some(path) => println!("{}", path.display()),
            None => println!("(could not determine config directory)"),
        },
        ConfigAction::List => match config_file {
            Some(cf) => {
                let mut names: Vec<_> = cf.profile_names();
                names.sort();
                for name in names {
                    if name == profile_name {
                        println!("{} (active)", name);
                    } else {
                        println!("{}", name);
                    }
                }
            }
            None => {
                println!("(no config file found)");
                if let Some(path) = config_path() {
                    println!("expected at: {}", path.display());
                }
            }
        },
        ConfigAction::Show => match config_file {
            Some(cf) => {
                if let Some(profile) = cf.get_profile(profile_name) {
                    let toml = toml::to_string_pretty(profile)?;
                    println!("[{}]", profile_name);
                    print!("{}", toml);
                } else {
                    return Err(format!("profile '{}' not found", profile_name).into());
                }
            }
            None => {
                return Err("no config file found".into());
            }
        },
        ConfigAction::Validate => match config_file {
            Some(cf) => {
                if let Some(profile) = cf.get_profile(profile_name) {
                    match validate_profile(profile) {
                        Ok(()) => {
                            println!("profile '{}' is valid", profile_name);
                            match profile.resolve_token() {
                                Ok(Some(_)) => println!("  token: ok"),
                                Ok(None) => {
                                    println!(
                                        "  token: (not set, will need --token or NAUTOBOT_TOKEN)"
                                    )
                                }
                                Err(e) => println!("  token: error - {}", e),
                            }
                        }
                        Err(e) => {
                            return Err(format!("profile '{}' invalid: {}", profile_name, e).into());
                        }
                    }
                } else {
                    return Err(format!("profile '{}' not found", profile_name).into());
                }
            }
            None => {
                return Err("no config file found".into());
            }
        },
    }
    Ok(())
}

pub async fn handle_resource_group(
    client: &impl ApiClient,
    output: &OutputConfig,
    group: &str,
    resources: &[ResourceEntry],
    resource: &str,
    action: ResourceAction,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = find_resource_path(resources, resource).ok_or_else(|| {
        format!(
            "unknown {} resource '{}'. use `nautobot-cli resources {}` to list options.",
            group, resource, group
        )
    })?;
    handle_resource_action(client, output, path, action).await
}

pub async fn handle_resource_action(
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
