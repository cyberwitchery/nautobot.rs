# CustomLinkRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**content_type** | **String** |  | 
**name** | **String** |  | 
**text** | **String** | Jinja2 template code for link text. Reference the object as <code>{{ obj }}</code> such as <code>{{ obj.platform.name }}</code>. Links which render as empty text will not be displayed. | 
**target_url** | **String** | Jinja2 template code for link URL. Reference the object as <code>{{ obj }}</code> such as <code>{{ obj.platform.name }}</code>. | 
**weight** | Option<**i32**> |  | [optional]
**group_name** | Option<**String**> | Links with the same group will appear as a dropdown menu | [optional]
**button_class** | Option<[**crate::models::ButtonClassEnum**](ButtonClassEnum.md)> |  | [optional]
**new_window** | **bool** | Force link to open in a new window | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


