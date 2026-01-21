# WritablePowerFeedRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | **String** |  | 
**r#type** | Option<[**crate::models::PowerFeedTypeChoices**](PowerFeedTypeChoices.md)> |  | [optional]
**power_path** | Option<[**crate::models::PatchedWritablePowerFeedRequestPowerPath**](PatchedWritablePowerFeedRequest_power_path.md)> |  | [optional]
**supply** | Option<[**crate::models::SupplyEnum**](SupplyEnum.md)> |  | [optional]
**phase** | Option<[**crate::models::PhaseEnum**](PhaseEnum.md)> |  | [optional]
**voltage** | Option<**i32**> |  | [optional]
**amperage** | Option<**i32**> |  | [optional]
**max_utilization** | Option<**i32**> | Maximum permissible draw (percentage) | [optional]
**breaker_position** | Option<**i32**> | Starting circuit breaker position in panel | [optional]
**breaker_pole_count** | Option<[**crate::models::PatchedWritablePowerFeedRequestBreakerPoleCount**](PatchedWritablePowerFeedRequest_breaker_pole_count.md)> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**cable** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**power_panel** | [**crate::models::BulkWritablePowerFeedRequestPowerPanel**](BulkWritablePowerFeedRequest_power_panel.md) |  | 
**destination_panel** | Option<[**crate::models::BulkWritablePowerFeedRequestDestinationPanel**](BulkWritablePowerFeedRequest_destination_panel.md)> |  | [optional]
**rack** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**status** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


