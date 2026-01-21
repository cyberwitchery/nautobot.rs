# JobResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**status** | Option<[**crate::models::JobResultStatus**](JobResult_status.md)> |  | [optional]
**name** | **String** |  | 
**task_name** | Option<**String**> | Registered name of the Celery task for this job. Internal use only. | [optional]
**date_created** | Option<**String**> |  | [optional][readonly]
**date_started** | Option<**String**> |  | [optional]
**date_done** | Option<**String**> |  | [optional]
**result** | Option<[**serde_json::Value**](.md)> | The data returned by the task | [optional][readonly]
**worker** | Option<**String**> |  | [optional]
**task_args** | Option<[**serde_json::Value**](.md)> |  | [optional]
**task_kwargs** | Option<[**serde_json::Value**](.md)> |  | [optional]
**celery_kwargs** | Option<[**serde_json::Value**](.md)> |  | [optional]
**traceback** | Option<**String**> |  | [optional]
**meta** | Option<[**serde_json::Value**](.md)> |  | [optional][readonly]
**job_model** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**user** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**scheduled_job** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**computed_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional][readonly]
**files** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


