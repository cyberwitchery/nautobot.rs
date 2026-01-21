# Job

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**task_queues** | Option<[**serde_json::Value**](.md)> |  | [optional][readonly]
**task_queues_override** | Option<**bool**> |  | [optional][readonly]
**module_name** | Option<**String**> | Dotted name of the Python module providing this job | [optional][readonly]
**job_class_name** | Option<**String**> | Name of the Python class providing this job | [optional][readonly]
**grouping** | **String** | Human-readable grouping that this job belongs to | 
**name** | **String** | Human-readable name of this job | 
**description** | Option<**String**> | Markdown formatting and a limited subset of HTML are supported | [optional]
**installed** | Option<**bool**> | Whether the Python module and class providing this job are presently installed and loadable | [optional][readonly]
**enabled** | Option<**bool**> | Whether this job can be executed by users | [optional]
**is_job_hook_receiver** | Option<**bool**> | Whether this job is a job hook receiver | [optional][readonly]
**is_job_button_receiver** | Option<**bool**> | Whether this job is a job button receiver | [optional][readonly]
**has_sensitive_variables** | Option<**bool**> | Whether this job contains sensitive variables | [optional]
**is_singleton** | Option<**bool**> | Whether this job should fail to run if another instance of this job is already running | [optional]
**approval_required** | Option<**bool**> | Whether the job requires approval from another user before running | [optional]
**hidden** | Option<**bool**> | Whether the job defaults to not being shown in the UI | [optional]
**dryrun_default** | Option<**bool**> | Whether the job defaults to running with dryrun argument set to true | [optional]
**read_only** | Option<**bool**> | Set to true if the job does not make any changes to the environment | [optional][readonly]
**soft_time_limit** | Option<**f64**> | Maximum runtime in seconds before the job will receive a <code>SoftTimeLimitExceeded</code> exception.<br>Set to 0 to use Nautobot system default | [optional]
**time_limit** | Option<**f64**> | Maximum runtime in seconds before the job will be forcibly terminated.<br>Set to 0 to use Nautobot system default | [optional]
**supports_dryrun** | Option<**bool**> | If supported, allows the job to bypass approval when running with dryrun argument set to true | [optional][readonly]
**grouping_override** | Option<**bool**> | If set, the configured grouping will remain even if the underlying Job source code changes | [optional]
**name_override** | Option<**bool**> | If set, the configured name will remain even if the underlying Job source code changes | [optional]
**description_override** | Option<**bool**> | If set, the configured description will remain even if the underlying Job source code changes | [optional]
**approval_required_override** | Option<**bool**> | If set, the configured value will remain even if the underlying Job source code changes | [optional]
**dryrun_default_override** | Option<**bool**> | If set, the configured value will remain even if the underlying Job source code changes | [optional]
**hidden_override** | Option<**bool**> | If set, the configured value will remain even if the underlying Job source code changes | [optional]
**soft_time_limit_override** | Option<**bool**> | If set, the configured value will remain even if the underlying Job source code changes | [optional]
**time_limit_override** | Option<**bool**> | If set, the configured value will remain even if the underlying Job source code changes | [optional]
**has_sensitive_variables_override** | Option<**bool**> | If set, the configured value will remain even if the underlying Job source code changes | [optional]
**job_queues_override** | Option<**bool**> | If set, the configured value will remain even if the underlying Job source code changes | [optional]
**default_job_queue_override** | Option<**bool**> | If set, the configured value will remain even if the underlying Job source code changes | [optional]
**is_singleton_override** | Option<**bool**> | If set, the configured value will remain even if the underlying Job source code changes | [optional]
**default_job_queue** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**job_queues** | Option<[**Vec<crate::models::JobQueues>**](Job_Queues.md)> | The job queues that this job can be run on | [optional][readonly]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


