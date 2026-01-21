# PatchedBulkWritableCustomLinkRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**content_type** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**text** | Option<**String**> | Jinja2 template code for link text. Reference the object as <code>{{ obj }}</code> such as <code>{{ obj.platform.name }}</code>. Links which render as empty text will not be displayed. | [optional]
**target_url** | Option<**String**> | Jinja2 template code for link URL. Reference the object as <code>{{ obj }}</code> such as <code>{{ obj.platform.name }}</code>. | [optional]
**weight** | Option<**i32**> |  | [optional]
**group_name** | Option<**String**> | Links with the same group will appear as a dropdown menu | [optional]
**button_class** | Option<[**crate::models::ButtonClassEnum**](ButtonClassEnum.md)> |  | [optional]
**new_window** | Option<**bool**> | Force link to open in a new window | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


