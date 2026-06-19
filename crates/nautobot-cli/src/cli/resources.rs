#[derive(Clone, Copy)]
pub struct ResourceEntry {
    pub name: &'static str,
    pub path: &'static str,
}

pub const DCIM_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "cables",
        path: "dcim/cables/",
    },
    ResourceEntry {
        name: "console-connections",
        path: "dcim/console-connections/",
    },
    ResourceEntry {
        name: "console-port-templates",
        path: "dcim/console-port-templates/",
    },
    ResourceEntry {
        name: "console-ports",
        path: "dcim/console-ports/",
    },
    ResourceEntry {
        name: "console-server-port-templates",
        path: "dcim/console-server-port-templates/",
    },
    ResourceEntry {
        name: "console-server-ports",
        path: "dcim/console-server-ports/",
    },
    ResourceEntry {
        name: "controller-managed-device-groups",
        path: "dcim/controller-managed-device-groups/",
    },
    ResourceEntry {
        name: "controllers",
        path: "dcim/controllers/",
    },
    ResourceEntry {
        name: "device-bay-templates",
        path: "dcim/device-bay-templates/",
    },
    ResourceEntry {
        name: "device-bays",
        path: "dcim/device-bays/",
    },
    ResourceEntry {
        name: "device-families",
        path: "dcim/device-families/",
    },
    ResourceEntry {
        name: "device-redundancy-groups",
        path: "dcim/device-redundancy-groups/",
    },
    ResourceEntry {
        name: "device-types-to-software-image-files",
        path: "dcim/device-types-to-software-image-files/",
    },
    ResourceEntry {
        name: "device-types",
        path: "dcim/device-types/",
    },
    ResourceEntry {
        name: "devices",
        path: "dcim/devices/",
    },
    ResourceEntry {
        name: "front-port-templates",
        path: "dcim/front-port-templates/",
    },
    ResourceEntry {
        name: "front-ports",
        path: "dcim/front-ports/",
    },
    ResourceEntry {
        name: "interface-connections",
        path: "dcim/interface-connections/",
    },
    ResourceEntry {
        name: "interface-redundancy-group-associations",
        path: "dcim/interface-redundancy-group-associations/",
    },
    ResourceEntry {
        name: "interface-redundancy-groups",
        path: "dcim/interface-redundancy-groups/",
    },
    ResourceEntry {
        name: "interface-templates",
        path: "dcim/interface-templates/",
    },
    ResourceEntry {
        name: "interface-vdc-assignments",
        path: "dcim/interface-vdc-assignments/",
    },
    ResourceEntry {
        name: "interfaces",
        path: "dcim/interfaces/",
    },
    ResourceEntry {
        name: "inventory-items",
        path: "dcim/inventory-items/",
    },
    ResourceEntry {
        name: "location-types",
        path: "dcim/location-types/",
    },
    ResourceEntry {
        name: "locations",
        path: "dcim/locations/",
    },
    ResourceEntry {
        name: "manufacturers",
        path: "dcim/manufacturers/",
    },
    ResourceEntry {
        name: "module-bay-templates",
        path: "dcim/module-bay-templates/",
    },
    ResourceEntry {
        name: "module-bays",
        path: "dcim/module-bays/",
    },
    ResourceEntry {
        name: "module-families",
        path: "dcim/module-families/",
    },
    ResourceEntry {
        name: "module-types",
        path: "dcim/module-types/",
    },
    ResourceEntry {
        name: "modules",
        path: "dcim/modules/",
    },
    ResourceEntry {
        name: "platforms",
        path: "dcim/platforms/",
    },
    ResourceEntry {
        name: "power-connections",
        path: "dcim/power-connections/",
    },
    ResourceEntry {
        name: "power-feeds",
        path: "dcim/power-feeds/",
    },
    ResourceEntry {
        name: "power-outlet-templates",
        path: "dcim/power-outlet-templates/",
    },
    ResourceEntry {
        name: "power-outlets",
        path: "dcim/power-outlets/",
    },
    ResourceEntry {
        name: "power-panels",
        path: "dcim/power-panels/",
    },
    ResourceEntry {
        name: "power-port-templates",
        path: "dcim/power-port-templates/",
    },
    ResourceEntry {
        name: "power-ports",
        path: "dcim/power-ports/",
    },
    ResourceEntry {
        name: "rack-groups",
        path: "dcim/rack-groups/",
    },
    ResourceEntry {
        name: "rack-reservations",
        path: "dcim/rack-reservations/",
    },
    ResourceEntry {
        name: "racks",
        path: "dcim/racks/",
    },
    ResourceEntry {
        name: "rear-port-templates",
        path: "dcim/rear-port-templates/",
    },
    ResourceEntry {
        name: "rear-ports",
        path: "dcim/rear-ports/",
    },
    ResourceEntry {
        name: "software-image-files",
        path: "dcim/software-image-files/",
    },
    ResourceEntry {
        name: "software-versions",
        path: "dcim/software-versions/",
    },
    ResourceEntry {
        name: "virtual-chassis",
        path: "dcim/virtual-chassis/",
    },
    ResourceEntry {
        name: "virtual-device-contexts",
        path: "dcim/virtual-device-contexts/",
    },
];

pub const IPAM_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "ip-address-to-interface",
        path: "ipam/ip-address-to-interface/",
    },
    ResourceEntry {
        name: "ip-addresses",
        path: "ipam/ip-addresses/",
    },
    ResourceEntry {
        name: "namespaces",
        path: "ipam/namespaces/",
    },
    ResourceEntry {
        name: "prefix-location-assignments",
        path: "ipam/prefix-location-assignments/",
    },
    ResourceEntry {
        name: "prefixes",
        path: "ipam/prefixes/",
    },
    ResourceEntry {
        name: "rirs",
        path: "ipam/rirs/",
    },
    ResourceEntry {
        name: "route-targets",
        path: "ipam/route-targets/",
    },
    ResourceEntry {
        name: "services",
        path: "ipam/services/",
    },
    ResourceEntry {
        name: "vlan-groups",
        path: "ipam/vlan-groups/",
    },
    ResourceEntry {
        name: "vlan-location-assignments",
        path: "ipam/vlan-location-assignments/",
    },
    ResourceEntry {
        name: "vlans",
        path: "ipam/vlans/",
    },
    ResourceEntry {
        name: "vrf-device-assignments",
        path: "ipam/vrf-device-assignments/",
    },
    ResourceEntry {
        name: "vrf-prefix-assignments",
        path: "ipam/vrf-prefix-assignments/",
    },
    ResourceEntry {
        name: "vrfs",
        path: "ipam/vrfs/",
    },
];

pub const CIRCUITS_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "circuit-terminations",
        path: "circuits/circuit-terminations/",
    },
    ResourceEntry {
        name: "circuit-types",
        path: "circuits/circuit-types/",
    },
    ResourceEntry {
        name: "circuits",
        path: "circuits/circuits/",
    },
    ResourceEntry {
        name: "provider-networks",
        path: "circuits/provider-networks/",
    },
    ResourceEntry {
        name: "providers",
        path: "circuits/providers/",
    },
];

pub const CLOUD_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "cloud-accounts",
        path: "cloud/cloud-accounts/",
    },
    ResourceEntry {
        name: "cloud-network-prefix-assignments",
        path: "cloud/cloud-network-prefix-assignments/",
    },
    ResourceEntry {
        name: "cloud-networks",
        path: "cloud/cloud-networks/",
    },
    ResourceEntry {
        name: "cloud-resource-types",
        path: "cloud/cloud-resource-types/",
    },
    ResourceEntry {
        name: "cloud-service-network-assignments",
        path: "cloud/cloud-service-network-assignments/",
    },
    ResourceEntry {
        name: "cloud-services",
        path: "cloud/cloud-services/",
    },
];

pub const TENANCY_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "tenant-groups",
        path: "tenancy/tenant-groups/",
    },
    ResourceEntry {
        name: "tenants",
        path: "tenancy/tenants/",
    },
];

pub const EXTRAS_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "computed-fields",
        path: "extras/computed-fields/",
    },
    ResourceEntry {
        name: "config-context-schemas",
        path: "extras/config-context-schemas/",
    },
    ResourceEntry {
        name: "config-contexts",
        path: "extras/config-contexts/",
    },
    ResourceEntry {
        name: "contact-associations",
        path: "extras/contact-associations/",
    },
    ResourceEntry {
        name: "contacts",
        path: "extras/contacts/",
    },
    ResourceEntry {
        name: "content-types",
        path: "extras/content-types/",
    },
    ResourceEntry {
        name: "custom-field-choices",
        path: "extras/custom-field-choices/",
    },
    ResourceEntry {
        name: "custom-fields",
        path: "extras/custom-fields/",
    },
    ResourceEntry {
        name: "custom-links",
        path: "extras/custom-links/",
    },
    ResourceEntry {
        name: "dynamic-group-memberships",
        path: "extras/dynamic-group-memberships/",
    },
    ResourceEntry {
        name: "dynamic-groups",
        path: "extras/dynamic-groups/",
    },
    ResourceEntry {
        name: "export-templates",
        path: "extras/export-templates/",
    },
    ResourceEntry {
        name: "external-integrations",
        path: "extras/external-integrations/",
    },
    ResourceEntry {
        name: "file-proxies",
        path: "extras/file-proxies/",
    },
    ResourceEntry {
        name: "git-repositories",
        path: "extras/git-repositories/",
    },
    ResourceEntry {
        name: "graphql-queries",
        path: "extras/graphql-queries/",
    },
    ResourceEntry {
        name: "image-attachments",
        path: "extras/image-attachments/",
    },
    ResourceEntry {
        name: "job-buttons",
        path: "extras/job-buttons/",
    },
    ResourceEntry {
        name: "job-hooks",
        path: "extras/job-hooks/",
    },
    ResourceEntry {
        name: "job-logs",
        path: "extras/job-logs/",
    },
    ResourceEntry {
        name: "job-queue-assignments",
        path: "extras/job-queue-assignments/",
    },
    ResourceEntry {
        name: "job-queues",
        path: "extras/job-queues/",
    },
    ResourceEntry {
        name: "job-results",
        path: "extras/job-results/",
    },
    ResourceEntry {
        name: "jobs",
        path: "extras/jobs/",
    },
    ResourceEntry {
        name: "metadata-choices",
        path: "extras/metadata-choices/",
    },
    ResourceEntry {
        name: "metadata-types",
        path: "extras/metadata-types/",
    },
    ResourceEntry {
        name: "notes",
        path: "extras/notes/",
    },
    ResourceEntry {
        name: "object-changes",
        path: "extras/object-changes/",
    },
    ResourceEntry {
        name: "object-metadata",
        path: "extras/object-metadata/",
    },
    ResourceEntry {
        name: "relationship-associations",
        path: "extras/relationship-associations/",
    },
    ResourceEntry {
        name: "relationships",
        path: "extras/relationships/",
    },
    ResourceEntry {
        name: "roles",
        path: "extras/roles/",
    },
    ResourceEntry {
        name: "saved-views",
        path: "extras/saved-views/",
    },
    ResourceEntry {
        name: "scheduled-jobs",
        path: "extras/scheduled-jobs/",
    },
    ResourceEntry {
        name: "secrets-groups-associations",
        path: "extras/secrets-groups-associations/",
    },
    ResourceEntry {
        name: "secrets-groups",
        path: "extras/secrets-groups/",
    },
    ResourceEntry {
        name: "secrets",
        path: "extras/secrets/",
    },
    ResourceEntry {
        name: "static-group-associations",
        path: "extras/static-group-associations/",
    },
    ResourceEntry {
        name: "statuses",
        path: "extras/statuses/",
    },
    ResourceEntry {
        name: "tags",
        path: "extras/tags/",
    },
    ResourceEntry {
        name: "teams",
        path: "extras/teams/",
    },
    ResourceEntry {
        name: "user-saved-view-associations",
        path: "extras/user-saved-view-associations/",
    },
    ResourceEntry {
        name: "webhooks",
        path: "extras/webhooks/",
    },
];

pub const USERS_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "groups",
        path: "users/groups/",
    },
    ResourceEntry {
        name: "permissions",
        path: "users/permissions/",
    },
    ResourceEntry {
        name: "tokens",
        path: "users/tokens/",
    },
    ResourceEntry {
        name: "users",
        path: "users/users/",
    },
];

pub const VIRTUALIZATION_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "cluster-groups",
        path: "virtualization/cluster-groups/",
    },
    ResourceEntry {
        name: "cluster-types",
        path: "virtualization/cluster-types/",
    },
    ResourceEntry {
        name: "clusters",
        path: "virtualization/clusters/",
    },
    ResourceEntry {
        name: "interfaces",
        path: "virtualization/interfaces/",
    },
    ResourceEntry {
        name: "virtual-machines",
        path: "virtualization/virtual-machines/",
    },
];

pub const WIRELESS_RESOURCES: &[ResourceEntry] = &[
    ResourceEntry {
        name: "controller-managed-device-group-radio-profile-assignments",
        path: "wireless/controller-managed-device-group-radio-profile-assignments/",
    },
    ResourceEntry {
        name: "controller-managed-device-group-wireless-network-assignments",
        path: "wireless/controller-managed-device-group-wireless-network-assignments/",
    },
    ResourceEntry {
        name: "radio-profiles",
        path: "wireless/radio-profiles/",
    },
    ResourceEntry {
        name: "supported-data-rates",
        path: "wireless/supported-data-rates/",
    },
    ResourceEntry {
        name: "wireless-networks",
        path: "wireless/wireless-networks/",
    },
];

pub fn print_resources(group: Option<&str>) {
    match group {
        None => {
            println!("dcim");
            list_resource_group(DCIM_RESOURCES);
            println!("ipam");
            list_resource_group(IPAM_RESOURCES);
            println!("circuits");
            list_resource_group(CIRCUITS_RESOURCES);
            println!("cloud");
            list_resource_group(CLOUD_RESOURCES);
            println!("tenancy");
            list_resource_group(TENANCY_RESOURCES);
            println!("extras");
            list_resource_group(EXTRAS_RESOURCES);
            println!("users");
            list_resource_group(USERS_RESOURCES);
            println!("virtualization");
            list_resource_group(VIRTUALIZATION_RESOURCES);
            println!("wireless");
            list_resource_group(WIRELESS_RESOURCES);
        }
        Some("dcim") => list_resource_group(DCIM_RESOURCES),
        Some("ipam") => list_resource_group(IPAM_RESOURCES),
        Some("circuits") => list_resource_group(CIRCUITS_RESOURCES),
        Some("cloud") => list_resource_group(CLOUD_RESOURCES),
        Some("tenancy") => list_resource_group(TENANCY_RESOURCES),
        Some("extras") => list_resource_group(EXTRAS_RESOURCES),
        Some("users") => list_resource_group(USERS_RESOURCES),
        Some("virtualization") => list_resource_group(VIRTUALIZATION_RESOURCES),
        Some("wireless") => list_resource_group(WIRELESS_RESOURCES),
        Some(other) => {
            println!("unknown group '{}'", other);
        }
    }
}

pub fn list_resource_group(resources: &[ResourceEntry]) {
    for entry in resources {
        println!("  {}", entry.name);
    }
}

pub fn find_resource_path(resources: &[ResourceEntry], name: &str) -> Option<&'static str> {
    resources
        .iter()
        .find(|entry| entry.name == name)
        .map(|entry| entry.path)
}

pub fn resource_path_with_id(path: &str, id: &str) -> String {
    format!("{}/{}/", path.trim_end_matches('/'), id)
}
