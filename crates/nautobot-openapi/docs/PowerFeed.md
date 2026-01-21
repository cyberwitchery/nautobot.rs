# PowerFeed

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**cable_peer_type** | Option<**String**> |  | [optional][readonly]
**cable_peer** | Option<[**crate::models::CableTermination**](CableTermination.md)> |  | [optional][readonly]
**connected_endpoint_type** | Option<**String**> |  | [optional][readonly]
**connected_endpoint** | Option<[**crate::models::PathEndpoint**](PathEndpoint.md)> |  | [optional][readonly]
**connected_endpoint_reachable** | Option<**bool**> |  | [optional][readonly]
**r#type** | Option<[**crate::models::PowerFeedType**](PowerFeed_type.md)> |  | [optional]
**power_path** | Option<[**crate::models::PowerFeedPowerPath**](PowerFeed_power_path.md)> |  | [optional]
**supply** | Option<[**crate::models::PowerFeedSupply**](PowerFeed_supply.md)> |  | [optional]
**phase** | Option<[**crate::models::PowerFeedPhase**](PowerFeed_phase.md)> |  | [optional]
**breaker_pole_count** | Option<[**crate::models::PowerFeedBreakerPoleCount**](PowerFeed_breaker_pole_count.md)> |  | [optional]
**name** | **String** |  | 
**voltage** | Option<**i32**> |  | [optional]
**amperage** | Option<**i32**> |  | [optional]
**max_utilization** | Option<**i32**> | Maximum permissible draw (percentage) | [optional]
**breaker_position** | Option<**i32**> | Starting circuit breaker position in panel | [optional]
**available_power** | Option<**i32**> |  | [optional][readonly]
**comments** | Option<**String**> |  | [optional]
**cable** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**power_panel** | [**crate::models::BulkWritablePowerFeedRequestPowerPanel**](BulkWritablePowerFeedRequest_power_panel.md) |  | 
**destination_panel** | Option<[**crate::models::BulkWritablePowerFeedRequestDestinationPanel**](BulkWritablePowerFeedRequest_destination_panel.md)> |  | [optional]
**rack** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**status** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


