# JobCreationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**start_time** | Option<**String**> |  | [optional]
**interval** | [**crate::models::JobExecutionTypeIntervalChoices**](JobExecutionTypeIntervalChoices.md) |  | 
**crontab** | Option<**String**> | Cronjob syntax string for custom scheduling | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


