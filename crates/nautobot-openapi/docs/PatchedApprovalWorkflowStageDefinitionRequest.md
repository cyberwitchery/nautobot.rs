# PatchedApprovalWorkflowStageDefinitionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**approver_group** | Option<**String**> | The group that will be assigned to approve this stage. | [optional]
**sequence** | Option<**i32**> | The sequence dictates the order in which this stage will need to be approved. The lower the number, the earlier it will be. | [optional]
**name** | Option<**String**> |  | [optional]
**min_approvers** | Option<**i32**> | Minimum number of approvers required to approve this stage. | [optional]
**denial_message** | Option<**String**> | Message to show when the stage is denied. | [optional]
**approval_workflow_definition** | Option<[**crate::models::ApprovalWorkflowStageDefinitionApprovalWorkflowDefinition**](ApprovalWorkflowStageDefinition_approval_workflow_definition.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


