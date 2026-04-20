# PatchedDataComplianceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**compliance_class_name** | Option<**String**> |  | [optional]
**object_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**validated_object_str** | Option<**String**> |  | [optional]
**validated_attribute** | Option<**String**> |  | [optional][default to ]
**validated_attribute_value** | Option<**String**> |  | [optional]
**valid** | Option<**bool**> |  | [optional]
**message** | Option<**String**> |  | [optional]
**content_type** | Option<[**crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage**](ApprovalWorkflowStageResponse_approval_workflow_stage.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


