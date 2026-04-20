# PatchedBulkWritableHealthCheckMonitorRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | Option<**String**> |  | [optional]
**interval** | Option<**i32**> |  | [optional]
**retry** | Option<**i32**> | Number of retries before marking as down | [optional]
**timeout** | Option<**i32**> |  | [optional]
**port** | Option<**i32**> |  | [optional]
**health_check_type** | Option<[**crate::models::BulkWritableHealthCheckMonitorRequestHealthCheckType**](BulkWritableHealthCheckMonitorRequest_health_check_type.md)> |  | [optional]
**tenant** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage>**](ApprovalWorkflowStageResponse_approval_workflow_stage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


