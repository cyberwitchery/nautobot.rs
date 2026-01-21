# BulkWritableObjectPermissionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**object_types** | **Vec<String>** |  | 
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**actions** | Option<[**serde_json::Value**](.md)> | The list of actions granted by this permission | 
**constraints** | Option<[**serde_json::Value**](.md)> | Queryset filter matching the applicable objects of the selected type(s) | [optional]
**groups** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**users** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


