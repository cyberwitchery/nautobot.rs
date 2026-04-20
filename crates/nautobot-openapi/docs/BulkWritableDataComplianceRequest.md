# BulkWritableDataComplianceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**compliance_class_name** | **String** |  | 
**object_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**validated_object_str** | Option<**String**> |  | [optional]
**validated_attribute** | Option<**String**> |  | [optional][default to ]
**validated_attribute_value** | Option<**String**> |  | [optional]
**valid** | **bool** |  | 
**message** | Option<**String**> |  | [optional]
**content_type** | [**crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage**](ApprovalWorkflowStageResponse_approval_workflow_stage.md) |  | 
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


