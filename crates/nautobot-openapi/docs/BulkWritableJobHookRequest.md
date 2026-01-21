# BulkWritableJobHookRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**content_types** | **Vec<String>** |  | 
**enabled** | Option<**bool**> |  | [optional]
**name** | **String** |  | 
**type_create** | Option<**bool**> | Call this job hook when a matching object is created. | [optional]
**type_delete** | Option<**bool**> | Call this job hook when a matching object is deleted. | [optional]
**type_update** | Option<**bool**> | Call this job hook when a matching object is updated. | [optional]
**job** | [**crate::models::BulkWritableJobHookRequestJob**](BulkWritableJobHookRequest_job.md) |  | 
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


