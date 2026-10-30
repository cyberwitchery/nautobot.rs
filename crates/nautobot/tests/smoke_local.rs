//! local smoke tests against a live Nautobot instance.
//!
//! run manually with:
//! NAUTOBOT_TOKEN=... NAUTOBOT_URL=http://localhost:8000 cargo test -p nautobot --test smoke_local -- --ignored

use nautobot::models::interface_template::Duplex;
use nautobot::{Client, ClientConfig};
use reqwest::Method;
use serde_json::{Value, json};

fn env_var(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn build_client() -> Option<Client> {
    let token = env_var("NAUTOBOT_TOKEN")?;
    let url = env_var("NAUTOBOT_URL").unwrap_or_else(|| "http://localhost:8000".to_string());
    let config = ClientConfig::new(url, token).with_max_retries(0);
    Client::new(config).ok()
}

#[tokio::test]
#[ignore]
async fn smoke_local_status() {
    let Some(client) = build_client() else {
        eprintln!("NAUTOBOT_TOKEN is not set; skipping smoke test");
        return;
    };

    let status = client.status().status().await.expect("status response");
    assert!(!status.is_empty(), "expected status payload");
}

async fn create_raw(client: &Client, path: &str, body: Value) -> String {
    let created = client
        .request_raw(Method::POST, path, Some(&body))
        .await
        .unwrap_or_else(|err| panic!("create {path}: {err}"));
    created["id"].as_str().expect("created id").to_string()
}

#[tokio::test]
#[ignore]
async fn smoke_local_typed_interface_template() {
    let Some(client) = build_client() else {
        eprintln!("NAUTOBOT_TOKEN is not set; skipping smoke test");
        return;
    };

    let suffix = std::process::id();
    let manufacturer = create_raw(
        &client,
        "dcim/manufacturers/",
        json!({"name": format!("smoke-manufacturer-{suffix}")}),
    )
    .await;
    let device_type = create_raw(
        &client,
        "dcim/device-types/",
        json!({"manufacturer": manufacturer, "model": format!("smoke-device-type-{suffix}")}),
    )
    .await;
    let template = create_raw(
        &client,
        "dcim/interface-templates/",
        json!({"device_type": device_type, "name": "eth0", "type": "1000base-t", "duplex": "full"}),
    )
    .await;

    let dcim = client.dcim();
    let typed = dcim.interface_templates().get(&template).await;

    dcim.interface_templates()
        .delete(&template)
        .await
        .expect("delete interface template");
    dcim.device_types()
        .delete(&device_type)
        .await
        .expect("delete device type");
    dcim.manufacturers()
        .delete(&manufacturer)
        .await
        .expect("delete manufacturer");

    let typed = typed.expect("typed interface template");
    assert_eq!(typed.duplex, Some(Duplex::Full));
    let device_type_id = typed
        .device_type
        .flatten()
        .and_then(|reference| reference.id)
        .map(|id| id.to_string());
    assert_eq!(device_type_id, Some(device_type));
}
