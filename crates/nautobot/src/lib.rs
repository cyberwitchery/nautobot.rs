#![doc = include_str!("../docs/client.md")]
#![warn(missing_docs)]
#![warn(clippy::all)]

mod client;
mod config;
mod error;
mod pagination;
mod query;
mod resource;

pub mod hooks;
pub use hooks::HttpHooks;

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
/// every model with the rest route that serves it.
pub mod object_types;
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
pub use object_types::{ObjectType, ObjectTypes};
pub use pagination::{Page, Paginator};
pub use query::QueryBuilder;
pub use resource::{BulkDelete, BulkUpdate, Resource};

/// generated openapi types and api functions.
pub mod openapi {
    pub use nautobot_openapi::apis;
    pub use nautobot_openapi::models;
}

pub use nautobot_openapi::models;
