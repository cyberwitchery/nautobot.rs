# WritablePowerPanelRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | **String** |  | 
**panel_type** | Option<[**crate::models::PatchedWritablePowerPanelRequestPanelType**](PatchedWritablePowerPanelRequest_panel_type.md)> |  | [optional]
**breaker_position_count** | Option<**i32**> | Total number of breaker positions in the panel (e.g., 42) | [optional]
**power_path** | Option<[**crate::models::PatchedWritablePowerFeedRequestPowerPath**](PatchedWritablePowerFeedRequest_power_path.md)> |  | [optional]
**location** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**rack_group** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


