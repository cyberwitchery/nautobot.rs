# VmInterface

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**mode** | Option<[**crate::models::InterfaceMode**](Interface_mode.md)> |  | [optional]
**mac_address** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**mtu** | Option<**i32**> |  | [optional]
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**status** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**role** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**parent_interface** | Option<[**crate::models::BulkWritableInterfaceRequestParentInterface**](BulkWritableInterfaceRequest_parent_interface.md)> |  | [optional]
**bridge** | Option<[**crate::models::BridgeInterface**](Bridge_interface.md)> |  | [optional]
**virtual_machine** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
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


