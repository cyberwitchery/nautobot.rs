# ApprovalWorkflowStageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**state** | Option<[**crate::models::ApprovalWorkflowStateChoices**](ApprovalWorkflowStateChoices.md)> |  | [optional]
**approval_workflow** | [**crate::models::ApprovalWorkflowStageApprovalWorkflow**](ApprovalWorkflowStage_approval_workflow.md) |  | 
**approval_workflow_stage_definition** | Option<[**crate::models::ApprovalWorkflowStageApprovalWorkflowStageDefinition**](ApprovalWorkflowStage_approval_workflow_stage_definition.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


