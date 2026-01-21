# PatchedModuleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**serial** | Option<**String**> |  | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this module | [optional]
**module_type** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**parent_module_bay** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**status** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**role** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**tenant** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**location** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


