# PatchedBulkWritableCloudNetworkRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**extra_config** | Option<[**serde_json::Value**](.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**cloud_resource_type** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**cloud_account** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**parent** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


