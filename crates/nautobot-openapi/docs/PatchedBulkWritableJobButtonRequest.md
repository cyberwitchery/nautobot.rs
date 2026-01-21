# PatchedBulkWritableJobButtonRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**content_types** | Option<**Vec<String>**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**text** | Option<**String**> | Jinja2 template code for button text. Reference the object as <code>{{ obj }}</code> such as <code>{{ obj.platform.name }}</code>. Buttons which render as empty text will not be displayed. | [optional]
**weight** | Option<**i32**> |  | [optional]
**group_name** | Option<**String**> | Buttons with the same group will appear as a dropdown menu. Group dropdown buttons will inherit the button class from the button with the lowest weight in the group. | [optional]
**button_class** | Option<[**crate::models::ButtonClassEnum**](ButtonClassEnum.md)> |  | [optional]
**confirmation** | Option<**bool**> | Enable confirmation pop-up box. <span class='text-danger'>WARNING: unselecting this option will allow the Job to run (and commit changes) with a single click!</span> | [optional]
**job** | Option<[**crate::models::BulkWritableJobButtonRequestJob**](BulkWritableJobButtonRequest_job.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


