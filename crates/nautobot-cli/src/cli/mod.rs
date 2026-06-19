mod handlers;
mod output;
mod resources;
mod util;

pub use handlers::{handle_config_command, handle_resource_group};
pub use output::{OutputConfig, OutputFormat, print_dry_run, print_output};
pub use resources::{
    CIRCUITS_RESOURCES, CLOUD_RESOURCES, DCIM_RESOURCES, EXTRAS_RESOURCES, IPAM_RESOURCES,
    ResourceEntry, TENANCY_RESOURCES, USERS_RESOURCES, VIRTUALIZATION_RESOURCES,
    WIRELESS_RESOURCES, find_resource_path, list_resource_group, print_resources,
    resource_path_with_id,
};
pub use util::{
    RequestError, append_query, build_schema_path, load_graphql_query, load_graphql_vars,
    load_json, load_json_optional, normalize_api_path, request_raw_with_context,
    wrap_request_error,
};

#[cfg(test)]
pub use handlers::handle_resource_action;
#[cfg(test)]
pub use output::{dry_run_payload, format_output, format_table, select_value};
#[cfg(test)]
pub use util::parse_query_pairs;
