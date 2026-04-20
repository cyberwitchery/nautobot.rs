# BulkWritableVpnProfileRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**keepalive_enabled** | Option<**bool**> |  | [optional]
**keepalive_interval** | Option<**i32**> |  | [optional]
**keepalive_retries** | Option<**i32**> |  | [optional]
**nat_traversal** | Option<**bool**> |  | [optional]
**extra_options** | Option<[**serde_json::Value**](.md)> | Additional options specific to the VPN technology and/or implementation | [optional]
**role** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**secrets_group** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**tenant** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage>**](ApprovalWorkflowStageResponse_approval_workflow_stage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


