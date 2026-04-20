# PatchedBulkWritableVpnPhase1PolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**encryption_algorithm** | Option<[**Vec<crate::models::EncryptionAlgorithmEnum>**](EncryptionAlgorithmEnum.md)> |  | [optional]
**integrity_algorithm** | Option<[**Vec<crate::models::IntegrityAlgorithmEnum>**](IntegrityAlgorithmEnum.md)> |  | [optional]
**dh_group** | Option<[**Vec<crate::models::VpnPhase2PolicyChoices>**](VPNPhase2PolicyChoices.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**ike_version** | Option<[**crate::models::BulkWritableVpnPhase1PolicyRequestIkeVersion**](BulkWritableVPNPhase1PolicyRequest_ike_version.md)> |  | [optional]
**aggressive_mode** | Option<**bool**> | Only applicable to IKEv1 | [optional]
**lifetime_seconds** | Option<**i32**> |  | [optional]
**lifetime_kb** | Option<**i32**> |  | [optional]
**authentication_method** | Option<[**crate::models::BulkWritableVpnPhase1PolicyRequestAuthenticationMethod**](BulkWritableVPNPhase1PolicyRequest_authentication_method.md)> |  | [optional]
**tenant** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage>**](ApprovalWorkflowStageResponse_approval_workflow_stage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


