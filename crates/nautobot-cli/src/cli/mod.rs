mod handlers;
mod output;
mod resources;
mod util;

pub use handlers::{handle_config_command, handle_resource_group};
pub use output::{OutputConfig, OutputFormat, print_dry_run, print_output};
pub use resources::{
    CIRCUITS_RESOURCES, CLOUD_RESOURCES, DCIM_RESOURCES, EXTRAS_RESOURCES, IPAM_RESOURCES,
    TENANCY_RESOURCES, USERS_RESOURCES, VIRTUALIZATION_RESOURCES, WIRELESS_RESOURCES,
    print_resources,
};
pub use util::{
    append_query, build_schema_path, load_graphql_query, load_graphql_vars, load_json_optional,
    normalize_api_path, request_raw_with_context, wrap_request_error,
};

#[cfg(test)]
pub use handlers::handle_resource_action;
#[cfg(test)]
pub use output::{dry_run_payload, format_output, format_table, select_value};
#[cfg(test)]
pub use resources::{find_resource_path, resource_path_with_id};
#[cfg(test)]
pub use util::parse_query_pairs;
#[cfg(test)]
pub use util::{RequestError, load_json};
