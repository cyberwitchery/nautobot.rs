# BulkWritableCloudResourceTypeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**content_types** | **Vec<String>** |  | 
**name** | **String** | Type of cloud objects | 
**description** | Option<**String**> |  | [optional]
**config_schema** | Option<[**serde_json::Value**](.md)> |  | [optional]
**provider** | [**crate::models::BulkWritableCloudAccountRequestProvider**](BulkWritableCloudAccountRequest_provider.md) |  | 
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


