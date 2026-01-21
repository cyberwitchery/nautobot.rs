# PatchedBulkWritableSavedViewRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | Option<**String**> | The name of this view | [optional]
**view** | Option<**String**> | The name of the list view that the saved view is derived from, e.g. dcim:device_list | [optional]
**config** | Option<[**serde_json::Value**](.md)> | Saved Configuration on this view | [optional]
**is_global_default** | Option<**bool**> |  | [optional]
**is_shared** | Option<**bool**> |  | [optional]
**owner** | Option<[**crate::models::BulkWritableSavedViewRequestOwner**](BulkWritableSavedViewRequest_owner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


