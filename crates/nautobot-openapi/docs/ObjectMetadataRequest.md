# ObjectMetadataRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**assigned_object_type** | **String** |  | 
**value** | Option<[**serde_json::Value**](.md)> |  | [optional]
**scoped_fields** | Option<[**serde_json::Value**](.md)> | List of scoped fields, only direct fields on the model | [optional]
**assigned_object_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**metadata_type** | [**crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage**](ApprovalWorkflowStageResponse_approval_workflow_stage.md) |  | 
**contact** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**team** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


