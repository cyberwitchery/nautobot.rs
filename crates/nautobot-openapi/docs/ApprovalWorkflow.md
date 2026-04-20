# ApprovalWorkflow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**object_under_review_content_type** | **String** |  | 
**decision_date** | Option<**String**> |  | [optional][readonly]
**object_under_review_object_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**current_state** | Option<[**crate::models::ApprovalWorkflowStateChoices**](ApprovalWorkflowStateChoices.md)> |  | [optional]
**user_name** | Option<**String**> |  | [optional][readonly]
**approval_workflow_definition** | Option<[**crate::models::ApprovalWorkflowApprovalWorkflowDefinition**](ApprovalWorkflow_approval_workflow_definition.md)> |  | [optional]
**user** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


