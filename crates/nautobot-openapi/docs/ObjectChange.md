# ObjectChange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**action** | Option<[**crate::models::ObjectChangeAction**](ObjectChange_action.md)> |  | [optional]
**changed_object_type** | Option<**String**> |  | [optional][readonly]
**related_object_type** | Option<**String**> |  | [optional][readonly]
**changed_object** | Option<[**crate::models::ObjectChangeChangedObject**](ObjectChangeChangedObject.md)> |  | [optional][readonly]
**time** | Option<**String**> |  | [optional][readonly]
**user_name** | Option<**String**> |  | [optional][readonly]
**request_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional][readonly]
**changed_object_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**change_context** | Option<**String**> |  | [optional][readonly]
**change_context_detail** | Option<**String**> |  | [optional][readonly]
**related_object_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_repr** | Option<**String**> |  | [optional][readonly]
**object_data** | Option<[**serde_json::Value**](.md)> |  | [optional][readonly]
**object_data_v2** | Option<[**serde_json::Value**](.md)> |  | [optional][readonly]
**user** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


