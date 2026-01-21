# PatchedWritableDeviceTypeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**front_image** | Option<[**std::path::PathBuf**](std::path::PathBuf.md)> |  | [optional]
**rear_image** | Option<[**std::path::PathBuf**](std::path::PathBuf.md)> |  | [optional]
**model** | Option<**String**> |  | [optional]
**part_number** | Option<**String**> | Discrete part number (optional) | [optional]
**u_height** | Option<**i32**> |  | [optional]
**is_full_depth** | Option<**bool**> | Device consumes both front and rear rack faces | [optional]
**subdevice_role** | Option<[**crate::models::ParentChildStatus**](Parent_child_status.md)> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**manufacturer** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**device_family** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


