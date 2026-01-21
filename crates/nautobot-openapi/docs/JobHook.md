# JobHook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**content_types** | **Vec<String>** |  | 
**enabled** | Option<**bool**> |  | [optional]
**name** | **String** |  | 
**type_create** | Option<**bool**> | Call this job hook when a matching object is created. | [optional]
**type_delete** | Option<**bool**> | Call this job hook when a matching object is deleted. | [optional]
**type_update** | Option<**bool**> | Call this job hook when a matching object is updated. | [optional]
**job** | [**crate::models::BulkWritableJobHookRequestJob**](BulkWritableJobHookRequest_job.md) |  | 
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


