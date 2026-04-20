# BulkWritableCableRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**termination_a_type** | **String** |  | 
**termination_b_type** | **String** |  | 
**length_unit** | Option<[**crate::models::LengthUnitEnum**](LengthUnitEnum.md)> |  | [optional]
**r#type** | Option<[**crate::models::CableTypeChoices**](CableTypeChoices.md)> |  | [optional]
**termination_a_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**termination_b_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**label** | Option<**String**> |  | [optional]
**color** | Option<**String**> | RGB color in hexadecimal (e.g. 00ff00) | [optional]
**length** | Option<**i32**> |  | [optional]
**status** | [**crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage**](ApprovalWorkflowStageResponse_approval_workflow_stage.md) |  | 
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage>**](ApprovalWorkflowStageResponse_approval_workflow_stage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


