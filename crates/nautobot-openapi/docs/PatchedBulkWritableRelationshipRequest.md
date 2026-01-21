# PatchedBulkWritableRelationshipRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**source_type** | Option<**String**> |  | [optional]
**destination_type** | Option<**String**> |  | [optional]
**label** | Option<**String**> | Label of the relationship as displayed to users | [optional]
**key** | Option<**String**> | Internal relationship key. Please use underscores rather than dashes in this key. | [optional]
**description** | Option<**String**> |  | [optional]
**r#type** | Option<[**crate::models::RelationshipTypeChoices**](RelationshipTypeChoices.md)> |  | [optional]
**required_on** | Option<[**crate::models::BulkWritableRelationshipRequestRequiredOn**](BulkWritableRelationshipRequest_required_on.md)> |  | [optional]
**source_label** | Option<**String**> | Label for related destination objects, as displayed on the source object. | [optional]
**source_hidden** | Option<**bool**> | Hide this relationship on the source object. | [optional]
**source_filter** | Option<[**serde_json::Value**](.md)> | Filterset filter matching the applicable source objects of the selected type | [optional]
**destination_label** | Option<**String**> | Label for related source objects, as displayed on the destination object. | [optional]
**destination_hidden** | Option<**bool**> | Hide this relationship on the destination object. | [optional]
**destination_filter** | Option<[**serde_json::Value**](.md)> | Filterset filter matching the applicable destination objects of the selected type | [optional]
**advanced_ui** | Option<**bool**> | Hide this field from the object's primary information tab. It will appear in the \"Advanced\" tab instead. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


