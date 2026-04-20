# VpnPhase2PolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**encryption_algorithm** | Option<[**Vec<crate::models::EncryptionAlgorithmEnum>**](EncryptionAlgorithmEnum.md)> |  | [optional]
**integrity_algorithm** | Option<[**Vec<crate::models::IntegrityAlgorithmEnum>**](IntegrityAlgorithmEnum.md)> |  | [optional]
**pfs_group** | Option<[**Vec<crate::models::VpnPhase2PolicyChoices>**](VPNPhase2PolicyChoices.md)> |  | [optional]
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**lifetime** | Option<**i32**> |  | [optional]
**tenant** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage>**](ApprovalWorkflowStageResponse_approval_workflow_stage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


