# JobLogEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**log_level** | Option<[**crate::models::LogLevelEnum**](LogLevelEnum.md)> |  | [optional]
**grouping** | Option<**String**> |  | [optional]
**message** | Option<**String**> |  | [optional]
**log_object** | Option<**String**> |  | [optional]
**absolute_url** | Option<**String**> |  | [optional]
**job_result** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**created** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


