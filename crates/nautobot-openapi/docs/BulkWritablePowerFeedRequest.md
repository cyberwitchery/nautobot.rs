# BulkWritablePowerFeedRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**r#type** | Option<[**crate::models::PowerFeedTypeChoices**](PowerFeedTypeChoices.md)> |  | [optional][default to Primary]
**power_path** | Option<[**crate::models::PowerPathEnum**](PowerPathEnum.md)> |  | [optional]
**supply** | Option<[**crate::models::SupplyEnum**](SupplyEnum.md)> |  | [optional][default to Ac]
**phase** | Option<[**crate::models::PhaseEnum**](PhaseEnum.md)> |  | [optional][default to SinglePhase]
**breaker_pole_count** | Option<**i32**> |  | [optional]
**name** | **String** |  | 
**voltage** | Option<**i32**> |  | [optional]
**amperage** | Option<**i32**> |  | [optional]
**max_utilization** | Option<**i32**> | Maximum permissible draw (percentage) | [optional]
**breaker_position** | Option<**i32**> | Starting circuit breaker position in panel | [optional]
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


