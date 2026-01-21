# GraphQlQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**variables** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional][readonly]
**owner_content_type** | Option<**String**> |  | [optional]
**owner** | Option<[**crate::models::GraphQlQueryOwner**](GraphQLQueryOwner.md)> |  | [optional][readonly]
**owner_object_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | **String** |  | 
**query** | **String** |  | 
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


