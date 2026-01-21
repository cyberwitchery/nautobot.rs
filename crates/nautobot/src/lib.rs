#![doc = include_str!("../docs/client.md")]
#![warn(missing_docs)]
#![warn(clippy::all)]

mod client;
mod config;
mod error;
mod pagination;
mod query;
mod resource;

// api endpoint modules
/// circuits and provider resources.
pub mod circuits;
/// cloud endpoints.
pub mod cloud;
/// core endpoints and system resources.
pub mod core;
/// dcim endpoints.
pub mod dcim;
/// extras endpoints (tags, webhooks, scripts, custom fields).
pub mod extras;
/// graphql query helper.
pub mod graphql;
/// ipam endpoints.
pub mod ipam;
/// metrics endpoint.
pub mod metrics;
/// status endpoint.
pub mod status;
/// tenancy endpoints.
pub mod tenancy;
/// ui endpoints.
pub mod ui;
/// users and auth endpoints.
pub mod users;
/// virtualization endpoints.
pub mod virtualization;
/// wireless endpoints.
pub mod wireless;

pub use client::Client;
pub use config::ClientConfig;
pub use error::{Error, Result};
pub use pagination::{Page, Paginator};
pub use query::QueryBuilder;
pub use resource::{BulkDelete, BulkUpdate, Resource};

/// generated openapi types and api functions.
pub mod openapi {
    pub use nautobot_openapi::apis;
    pub use nautobot_openapi::models;
}

// re-export the generated models for convenience
pub use nautobot_openapi::models;
