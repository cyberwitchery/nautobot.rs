# ApprovalWorkflowStageDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**approver_group** | **String** | The group that will be assigned to approve this stage. | 
**sequence** | **i32** | The sequence dictates the order in which this stage will need to be approved. The lower the number, the earlier it will be. | 
**name** | **String** |  | 
**min_approvers** | **i32** | Minimum number of approvers required to approve this stage. | 
**denial_message** | Option<**String**> | Message to show when the stage is denied. | [optional]
**approval_workflow_definition** | [**crate::models::ApprovalWorkflowStageDefinitionApprovalWorkflowDefinition**](ApprovalWorkflowStageDefinition_approval_workflow_definition.md) |  | 
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


