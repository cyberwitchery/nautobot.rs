# CablePath

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional][readonly]
**origin_type** | Option<**String**> |  | [optional][readonly]
**origin** | Option<[**crate::models::PathEndpoint**](PathEndpoint.md)> |  | [optional][readonly]
**destination_type** | Option<**String**> |  | [optional][readonly]
**destination** | Option<[**crate::models::PathEndpoint**](PathEndpoint.md)> |  | [optional][readonly]
**path** | Option<[**Vec<crate::models::CableTermination>**](CableTermination.md)> |  | [optional][readonly]
**origin_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**destination_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**is_active** | Option<**bool**> |  | [optional]
**is_split** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


