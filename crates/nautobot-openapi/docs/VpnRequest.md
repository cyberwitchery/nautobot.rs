# VpnRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**vpn_id** | Option<**String**> |  | [optional]
**service_type** | Option<[**crate::models::BulkWritableVpnRequestServiceType**](BulkWritableVPNRequest_service_type.md)> |  | [optional]
**extra_attributes** | Option<[**serde_json::Value**](.md)> | Free-form scalar service metadata only; not for references to real Nautobot objects. | [optional]
**vpn_profile** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**role** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**tenant** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**status** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage>**](ApprovalWorkflowStageResponse_approval_workflow_stage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


