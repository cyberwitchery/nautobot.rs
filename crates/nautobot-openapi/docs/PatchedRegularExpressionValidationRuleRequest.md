# PatchedRegularExpressionValidationRuleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**content_type** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**field** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**error_message** | Option<**String**> | Optional error message to display when validation fails. | [optional]
**regular_expression** | Option<**String**> |  | [optional]
**context_processing** | Option<**bool**> | When enabled, the regular expression value is first processed as a Jinja2 template with access to the context of the data being validated in a variable named <code>object</code>. | [optional]
**tags** | Option<[**Vec<crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage>**](ApprovalWorkflowStageResponse_approval_workflow_stage.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


