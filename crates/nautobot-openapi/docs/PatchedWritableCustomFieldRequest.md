# PatchedWritableCustomFieldRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**content_types** | Option<**Vec<String>**> |  | [optional]
**label** | Option<**String**> |  | [optional]
**grouping** | Option<**String**> | Human-readable grouping that this custom field belongs to. | [optional]
**r#type** | Option<[**crate::models::CustomFieldTypeChoices**](CustomFieldTypeChoices.md)> |  | [optional]
**key** | Option<**String**> | Internal field name. Please use underscores rather than dashes in this key. | [optional]
**description** | Option<**String**> | A helpful description for this field. | [optional]
**required** | Option<**bool**> | If true, this field is required when creating new objects or editing an existing object. | [optional]
**filter_logic** | Option<[**crate::models::FilterLogicEnum**](FilterLogicEnum.md)> |  | [optional]
**default** | Option<[**serde_json::Value**](.md)> | Default value for the field (must be a JSON value). Encapsulate strings with double quotes (e.g. \"Foo\"). | [optional]
**weight** | Option<**i32**> | Fields with higher weights appear lower in a form. | [optional]
**validation_minimum** | Option<**i64**> | Minimum allowed value (for numeric fields) or length (for text fields). | [optional]
**validation_maximum** | Option<**i64**> | Maximum allowed value (for numeric fields) or length (for text fields). | [optional]
**validation_regex** | Option<**String**> | Regular expression to enforce on text field values. Use ^ and $ to force matching of entire string. For example, <code>^[A-Z]{3}$</code> will limit values to exactly three uppercase letters. Regular expression on select and multi-select will be applied at <code>Custom Field Choices</code> definition. | [optional]
**advanced_ui** | Option<**bool**> | Hide this field from the object's primary information tab. It will appear in the \"Advanced\" tab instead. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


