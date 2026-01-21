# ConfigContextSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**owner_content_type** | Option<**String**> |  | [optional]
**owner** | Option<[**crate::models::ConfigContextSchemaOwner**](ConfigContextSchemaOwner.md)> |  | [optional][readonly]
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**data_schema** | Option<[**serde_json::Value**](.md)> | A JSON Schema document which is used to validate a config context object. | 
**owner_object_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


