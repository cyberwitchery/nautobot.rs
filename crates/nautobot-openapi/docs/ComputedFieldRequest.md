# ComputedFieldRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**content_type** | **String** |  | 
**key** | Option<**String**> | Internal field name. Please use underscores rather than dashes in this key. | [optional]
**grouping** | Option<**String**> | Human-readable grouping that this computed field belongs to. | [optional]
**label** | **String** | Name of the field as displayed to users | 
**description** | Option<**String**> |  | [optional]
**template** | **String** | Jinja2 template code for field value | 
**fallback_value** | Option<**String**> | Fallback value (if any) to be output for the field in the case of a template rendering error. | [optional]
**weight** | Option<**i32**> |  | [optional]
**advanced_ui** | Option<**bool**> | Hide this field from the object's primary information tab. It will appear in the \"Advanced\" tab instead. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


