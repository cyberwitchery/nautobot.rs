# DataCompliance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**compliance_class_name** | **String** |  | 
**last_validation_date** | Option<**String**> |  | [optional][readonly]
**object_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**validated_object_str** | Option<**String**> |  | [optional]
**validated_attribute** | Option<**String**> |  | [optional][default to ]
**validated_attribute_value** | Option<**String**> |  | [optional]
**valid** | **bool** |  | 
**message** | Option<**String**> |  | [optional]
**content_type** | [**crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage**](ApprovalWorkflowStageResponse_approval_workflow_stage.md) |  | 
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


