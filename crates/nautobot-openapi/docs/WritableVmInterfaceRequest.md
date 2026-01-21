# WritableVmInterfaceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**mac_address** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**mtu** | Option<**i32**> |  | [optional]
**mode** | Option<[**crate::models::PatchedWritableInterfaceRequestMode**](PatchedWritableInterfaceRequest_mode.md)> |  | [optional]
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
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


