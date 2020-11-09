#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBareMetalInstancesListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AzureBareMetalInstance>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBareMetalInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureBareMetalInstanceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBareMetalInstanceProperties {
    #[serde(rename = "hardwareProfile", skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,
    #[serde(rename = "storageProfile", skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[serde(rename = "osProfile", skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[serde(rename = "networkProfile", skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    #[serde(rename = "azureBareMetalInstanceId", skip_serializing)]
    pub azure_bare_metal_instance_id: Option<String>,
    #[serde(rename = "powerState", skip_serializing)]
    pub power_state: Option<azure_bare_metal_instance_properties::PowerState>,
    #[serde(rename = "proximityPlacementGroup", skip_serializing)]
    pub proximity_placement_group: Option<String>,
    #[serde(rename = "hwRevision", skip_serializing)]
    pub hw_revision: Option<String>,
    #[serde(rename = "partnerNodeId", skip_serializing_if = "Option::is_none")]
    pub partner_node_id: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<azure_bare_metal_instance_properties::ProvisioningState>,
}
pub mod azure_bare_metal_instance_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PowerState {
        #[serde(rename = "starting")]
        Starting,
        #[serde(rename = "started")]
        Started,
        #[serde(rename = "stopping")]
        Stopping,
        #[serde(rename = "stopped")]
        Stopped,
        #[serde(rename = "restarting")]
        Restarting,
        #[serde(rename = "unknown")]
        Unknown,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Accepted,
        Creating,
        Updating,
        Failed,
        Succeeded,
        Deleting,
        Migrating,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HardwareProfile {
    #[serde(rename = "hardwareType", skip_serializing)]
    pub hardware_type: Option<hardware_profile::HardwareType>,
    #[serde(rename = "azureBareMetalInstanceSize", skip_serializing)]
    pub azure_bare_metal_instance_size: Option<hardware_profile::AzureBareMetalInstanceSize>,
}
pub mod hardware_profile {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum HardwareType {
        #[serde(rename = "Cisco_UCS")]
        CiscoUcs,
        #[serde(rename = "HPE")]
        Hpe,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AzureBareMetalInstanceSize {
        S72m,
        S144m,
        S72,
        S144,
        S192,
        S192m,
        S192xm,
        S96,
        S112,
        S224,
        S224m,
        S224om,
        S224oo,
        S224oom,
        S224ooo,
        S384,
        S384m,
        S384xm,
        S384xxm,
        S448,
        S448m,
        S448om,
        S448oo,
        S448oom,
        S448ooo,
        S576m,
        S576xm,
        S672,
        S672m,
        S672om,
        S672oo,
        S672oom,
        S672ooo,
        S768,
        S768m,
        S768xm,
        S896,
        S896m,
        S896om,
        S896oo,
        S896oom,
        S896ooo,
        S960m,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Disk {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "diskSizeGB", skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[serde(skip_serializing)]
    pub lun: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageProfile {
    #[serde(rename = "nfsIpAddress", skip_serializing)]
    pub nfs_ip_address: Option<String>,
    #[serde(rename = "osDisks", skip_serializing_if = "Vec::is_empty")]
    pub os_disks: Vec<Disk>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsProfile {
    #[serde(rename = "computerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[serde(rename = "osType", skip_serializing)]
    pub os_type: Option<String>,
    #[serde(skip_serializing)]
    pub version: Option<String>,
    #[serde(rename = "sshPublicKey", skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkProfile {
    #[serde(rename = "networkInterfaces", skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<IpAddress>,
    #[serde(rename = "circuitId", skip_serializing)]
    pub circuit_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpAddress {
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<Display>,
    #[serde(rename = "isDataAction", skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Result {
    #[serde(rename = "sampleProperty", skip_serializing_if = "Option::is_none")]
    pub sample_property: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Display {
    #[serde(skip_serializing)]
    pub provider: Option<String>,
    #[serde(skip_serializing)]
    pub resource: Option<String>,
    #[serde(skip_serializing)]
    pub operation: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(skip_serializing)]
    pub origin: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ErrorDefinition>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tags {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}