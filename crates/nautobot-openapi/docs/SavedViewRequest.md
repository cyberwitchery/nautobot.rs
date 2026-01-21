# SavedViewRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | **String** | The name of this view | 
**view** | **String** | The name of the list view that the saved view is derived from, e.g. dcim:device_list | 
**config** | Option<[**serde_json::Value**](.md)> | Saved Configuration on this view | [optional]
**is_global_default** | Option<**bool**> |  | [optional]
**is_shared** | Option<**bool**> |  | [optional]
**owner** | [**crate::models::BulkWritableSavedViewRequestOwner**](BulkWritableSavedViewRequest_owner.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


