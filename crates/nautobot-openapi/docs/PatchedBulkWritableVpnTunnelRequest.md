# PatchedBulkWritableVpnTunnelRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**tunnel_id** | Option<**String**> |  | [optional]
**encapsulation** | Option<[**crate::models::BulkWritableVpnTunnelRequestEncapsulation**](BulkWritableVPNTunnelRequest_encapsulation.md)> |  | [optional]
**vpn_profile** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**vpn** | Option<[**crate::models::BulkWritableVpnTunnelRequestVpn**](BulkWritableVPNTunnelRequest_vpn.md)> |  | [optional]
**role** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**status** | Option<[**crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage**](ApprovalWorkflowStageResponse_approval_workflow_stage.md)> |  | [optional]
**secrets_group** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**endpoint_a** | Option<[**crate::models::BulkWritableVpnTunnelRequestEndpointA**](BulkWritableVPNTunnelRequest_endpoint_a.md)> |  | [optional]
**endpoint_z** | Option<[**crate::models::BulkWritableVpnTunnelRequestEndpointZ**](BulkWritableVPNTunnelRequest_endpoint_z.md)> |  | [optional]
**tenant** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage>**](ApprovalWorkflowStageResponse_approval_workflow_stage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


