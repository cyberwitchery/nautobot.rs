# PatchedBulkWritableMinMaxValidationRuleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**content_type** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**field** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**error_message** | Option<**String**> | Optional error message to display when validation fails. | [optional]
**min** | Option<**f64**> | When set, apply a minimum value contraint to the value of the model field. | [optional]
**max** | Option<**f64**> | When set, apply a maximum value contraint to the value of the model field. | [optional]
**tags** | Option<[**Vec<crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage>**](ApprovalWorkflowStageResponse_approval_workflow_stage.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


