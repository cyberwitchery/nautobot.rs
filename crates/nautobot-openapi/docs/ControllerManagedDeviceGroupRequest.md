# ControllerManagedDeviceGroupRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**capabilities** | Option<[**Vec<crate::models::CapabilitiesEnum>**](CapabilitiesEnum.md)> |  | [optional]
**name** | **String** | Name of the controller device group | 
**description** | Option<**String**> |  | [optional]
**weight** | Option<**i32**> | Weight of the controller device group, used to sort the groups within its parent group | [optional]
**parent** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**controller** | [**crate::models::BulkWritableControllerManagedDeviceGroupRequestController**](BulkWritableControllerManagedDeviceGroupRequest_controller.md) |  | 
**tenant** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


