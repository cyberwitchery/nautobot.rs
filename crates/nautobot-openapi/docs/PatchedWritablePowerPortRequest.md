# PatchedWritablePowerPortRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**label** | Option<**String**> | Physical label | [optional]
**description** | Option<**String**> |  | [optional]
**r#type** | Option<[**crate::models::PatchedWritablePowerPortRequestType**](PatchedWritablePowerPortRequest_type.md)> |  | [optional]
**maximum_draw** | Option<**i32**> | Maximum power draw (watts) | [optional]
**allocated_draw** | Option<**i32**> | Allocated power draw (watts) | [optional]
**power_factor** | Option<**String**> | Power factor (0.01-1.00) for converting between watts (W) and volt-amps (VA). Defaults to 0.95. | [optional]
**device** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**module** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


