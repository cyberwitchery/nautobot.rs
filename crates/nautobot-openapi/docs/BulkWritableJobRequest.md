# BulkWritableJobRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**grouping** | **String** | Human-readable grouping that this job belongs to | 
**name** | **String** | Human-readable name of this job | 
**description** | Option<**String**> | Markdown formatting and a limited subset of HTML are supported | [optional]
**enabled** | Option<**bool**> | Whether this job can be executed by users | [optional]
**has_sensitive_variables** | Option<**bool**> | Whether this job contains sensitive variables | [optional]
**is_singleton** | Option<**bool**> | Whether this job should fail to run if another instance of this job is already running | [optional]
**console_log_default** | Option<**bool**> | Whether the job defaults to running with console log argument set to true | [optional]
**hidden** | Option<**bool**> | Whether the job defaults to not being shown in the UI | [optional]
**dryrun_default** | Option<**bool**> | Whether the job defaults to running with dryrun argument set to true | [optional]
**soft_time_limit** | Option<**f64**> | Maximum runtime in seconds before the job will receive a <code>SoftTimeLimitExceeded</code> exception.<br>Set to 0 to use Nautobot system default | [optional]
**time_limit** | Option<**f64**> | Maximum runtime in seconds before the job will be forcibly terminated.<br>Set to 0 to use Nautobot system default | [optional]
**grouping_override** | Option<**bool**> | If set, the configured grouping will remain even if the underlying Job source code changes | [optional]
**name_override** | Option<**bool**> | If set, the configured name will remain even if the underlying Job source code changes | [optional]
**console_log_default_override** | Option<**bool**> | If set, the configured console log default will remain even if the underlying Job source code changes | [optional]
**description_override** | Option<**bool**> | If set, the configured description will remain even if the underlying Job source code changes | [optional]
**dryrun_default_override** | Option<**bool**> | If set, the configured value will remain even if the underlying Job source code changes | [optional]
**hidden_override** | Option<**bool**> | If set, the configured value will remain even if the underlying Job source code changes | [optional]
**soft_time_limit_override** | Option<**bool**> | If set, the configured value will remain even if the underlying Job source code changes | [optional]
**time_limit_override** | Option<**bool**> | If set, the configured value will remain even if the underlying Job source code changes | [optional]
**has_sensitive_variables_override** | Option<**bool**> | If set, the configured value will remain even if the underlying Job source code changes | [optional]
**job_queues_override** | Option<**bool**> | If set, the configured value will remain even if the underlying Job source code changes | [optional]
**default_job_queue_override** | Option<**bool**> | If set, the configured value will remain even if the underlying Job source code changes | [optional]
**is_singleton_override** | Option<**bool**> | If set, the configured value will remain even if the underlying Job source code changes | [optional]
**default_job_queue** | [**crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage**](ApprovalWorkflowStageResponse_approval_workflow_stage.md) |  | 
**tags** | Option<[**Vec<crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage>**](ApprovalWorkflowStageResponse_approval_workflow_stage.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


