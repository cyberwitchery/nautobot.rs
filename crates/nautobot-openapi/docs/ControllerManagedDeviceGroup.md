# ControllerManagedDeviceGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**capabilities** | Option<[**Vec<crate::models::ControllerCapabilitiesInner>**](Controller_capabilities_inner.md)> |  | [optional]
**name** | **String** | Name of the controller device group | 
**description** | Option<**String**> |  | [optional]
**weight** | Option<**i32**> | Weight of the controller device group, used to sort the groups within its parent group | [optional]
**parent** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**controller** | [**crate::models::BulkWritableControllerManagedDeviceGroupRequestController**](BulkWritableControllerManagedDeviceGroupRequest_controller.md) |  | 
**tenant** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**radio_profiles** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional][readonly]
**wireless_networks** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional][readonly]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


