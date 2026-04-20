# VpnPhase1Policy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**encryption_algorithm** | Option<[**Vec<crate::models::EncryptionAlgorithmEnum>**](EncryptionAlgorithmEnum.md)> |  | [optional]
**integrity_algorithm** | Option<[**Vec<crate::models::IntegrityAlgorithmEnum>**](IntegrityAlgorithmEnum.md)> |  | [optional]
**dh_group** | Option<[**Vec<crate::models::VpnPhase2PolicyChoices>**](VPNPhase2PolicyChoices.md)> |  | [optional]
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**ike_version** | Option<[**crate::models::BulkWritableVpnPhase1PolicyRequestIkeVersion**](BulkWritableVPNPhase1PolicyRequest_ike_version.md)> |  | [optional]
**aggressive_mode** | Option<**bool**> | Only applicable to IKEv1 | [optional]
**lifetime_seconds** | Option<**i32**> |  | [optional]
**lifetime_kb** | Option<**i32**> |  | [optional]
**authentication_method** | Option<[**crate::models::BulkWritableVpnPhase1PolicyRequestAuthenticationMethod**](BulkWritableVPNPhase1PolicyRequest_authentication_method.md)> |  | [optional]
**tenant** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage>**](ApprovalWorkflowStageResponse_approval_workflow_stage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


