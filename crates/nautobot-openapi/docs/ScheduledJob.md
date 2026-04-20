# ScheduledJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**queue** | Option<**String**> |  | [optional][readonly]
**time_zone** | Option<**String**> |  | [optional]
**state** | Option<[**crate::models::ScheduledJobState**](ScheduledJob_state.md)> |  | [optional]
**name** | **String** | Human-readable description of this scheduled task | 
**task** | **String** | The name of the Celery task that should be run. (Example: \"proj.tasks.import_contacts\") | 
**interval** | [**crate::models::JobExecutionTypeIntervalChoices**](JobExecutionTypeIntervalChoices.md) |  | 
**crontab** | Option<**String**> | Cronjob syntax string for custom scheduling | [optional]
**args** | Option<[**serde_json::Value**](.md)> |  | [optional]
**kwargs** | Option<[**serde_json::Value**](.md)> |  | [optional]
**one_off** | Option<**bool**> | If True, the schedule will only run the task a single time | [optional]
**start_time** | **String** | Datetime when the schedule should begin triggering the task to run | 
**enabled** | Option<**bool**> | Set to False to disable the schedule | [optional]
**last_run_at** | Option<**String**> | Datetime that the schedule last triggered the task to run. Reset to None if enabled is set to False. | [optional][readonly]
**total_run_count** | Option<**i32**> | Running count of how many times the schedule has triggered the task | [optional][readonly]
**date_changed** | Option<**String**> | Datetime that this scheduled job was last modified | [optional][readonly]
**description** | Option<**String**> | Detailed description about the details of this scheduled job | [optional]
**celery_kwargs** | Option<[**serde_json::Value**](.md)> |  | [optional]
**decision_date** | Option<**String**> | Datetime that the schedule was approved or denied | [optional][readonly]
**job_model** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**job_queue** | Option<[**crate::models::JobQueueOverride**](Job_Queue_Override.md)> |  | [optional]
**user** | Option<[**crate::models::ScheduledJobUser**](ScheduledJob_user.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


