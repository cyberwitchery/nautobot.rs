# PatchedWritableInterfaceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**mac_address** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**label** | Option<**String**> | Physical label | [optional]
**description** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**mtu** | Option<**i32**> |  | [optional]
**mode** | Option<[**crate::models::PatchedWritableInterfaceRequestMode**](PatchedWritableInterfaceRequest_mode.md)> |  | [optional]
**r#type** | Option<[**crate::models::InterfaceTypeChoices**](InterfaceTypeChoices.md)> |  | [optional]
**mgmt_only** | Option<**bool**> | This interface is used only for out-of-band management | [optional]
**device** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**module** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**status** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**role** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**parent_interface** | Option<[**crate::models::BulkWritableInterfaceRequestParentInterface**](BulkWritableInterfaceRequest_parent_interface.md)> |  | [optional]
**bridge** | Option<[**crate::models::BridgeInterface**](Bridge_interface.md)> |  | [optional]
**lag** | Option<[**crate::models::ParentLag**](Parent_LAG.md)> |  | [optional]
**untagged_vlan** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**vrf** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**tagged_vlans** | Option<[**Vec<crate::models::TaggedVlans>**](Tagged_VLANs.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


