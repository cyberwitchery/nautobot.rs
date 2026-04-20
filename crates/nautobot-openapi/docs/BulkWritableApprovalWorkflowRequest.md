# BulkWritableApprovalWorkflowRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**object_under_review_content_type** | **String** |  | 
**object_under_review_object_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**current_state** | Option<[**crate::models::ApprovalWorkflowStateChoices**](ApprovalWorkflowStateChoices.md)> |  | [optional]
**approval_workflow_definition** | Option<[**crate::models::ApprovalWorkflowApprovalWorkflowDefinition**](ApprovalWorkflow_approval_workflow_definition.md)> |  | [optional]
**user** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


