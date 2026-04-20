# BulkWritableApprovalWorkflowDefinitionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**model_content_type** | **String** |  | 
**name** | **String** |  | 
**model_constraints** | Option<[**serde_json::Value**](.md)> | Constraints to filter the objects that can be approved using this workflow. | [optional]
**weight** | Option<**i32**> | Determines workflow relevance when multiple apply. Higher weight wins. | [optional][default to 0]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


