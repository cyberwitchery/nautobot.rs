# ExportTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**content_type** | **String** |  | 
**owner_content_type** | Option<**String**> |  | [optional]
**owner** | Option<[**crate::models::ExportTemplateOwner**](ExportTemplateOwner.md)> |  | [optional][readonly]
**owner_object_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**template_code** | **String** | The list of objects being exported is passed as a context variable named <code>queryset</code>. | 
**mime_type** | Option<**String**> | Defaults to <code>text/plain</code> | [optional]
**file_extension** | Option<**String**> | Extension to append to the rendered filename | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


