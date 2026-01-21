# Interface

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**cable_peer_type** | Option<**String**> |  | [optional][readonly]
**cable_peer** | Option<[**crate::models::CableTermination**](CableTermination.md)> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**connected_endpoint_type** | Option<**String**> |  | [optional][readonly]
**connected_endpoint** | Option<[**crate::models::PathEndpoint**](PathEndpoint.md)> |  | [optional][readonly]
**connected_endpoint_reachable** | Option<**bool**> |  | [optional][readonly]
**r#type** | [**crate::models::InterfaceType**](Interface_type.md) |  | 
**mode** | Option<[**crate::models::InterfaceMode**](Interface_mode.md)> |  | [optional]
**mac_address** | Option<**String**> |  | [optional]
**ip_address_count** | Option<**i32**> |  | [optional][readonly]
**name** | **String** |  | 
**label** | Option<**String**> | Physical label | [optional]
**description** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**mtu** | Option<**i32**> |  | [optional]
**mgmt_only** | Option<**bool**> | This interface is used only for out-of-band management | [optional]
**device** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**module** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**cable** | Option<[**crate::models::CircuitCircuitTerminationA**](Circuit_circuit_termination_a.md)> |  | [optional]
**status** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**role** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**parent_interface** | Option<[**crate::models::BulkWritableInterfaceRequestParentInterface**](BulkWritableInterfaceRequest_parent_interface.md)> |  | [optional]
**bridge** | Option<[**crate::models::BridgeInterface**](Bridge_interface.md)> |  | [optional]
**lag** | Option<[**crate::models::ParentLag**](Parent_LAG.md)> |  | [optional]
**untagged_vlan** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**vrf** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**tagged_vlans** | Option<[**Vec<crate::models::TaggedVlans>**](Tagged_VLANs.md)> |  | [optional]
**ip_addresses** | Option<[**Vec<crate::models::IpAddresses>**](IP_Addresses.md)> |  | [optional][readonly]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


