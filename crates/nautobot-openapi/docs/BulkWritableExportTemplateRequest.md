# BulkWritableExportTemplateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**content_type** | **String** |  | 
**owner_content_type** | Option<**String**> |  | [optional]
**owner_object_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**template_code** | **String** | The list of objects being exported is passed as a context variable named <code>queryset</code>. | 
**mime_type** | Option<**String**> | Defaults to <code>text/plain</code> | [optional]
**file_extension** | Option<**String**> | Extension to append to the rendered filename | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


