# IpAddressToInterfaceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**is_source** | Option<**bool**> | Is source address on interface | [optional]
**is_destination** | Option<**bool**> | Is destination address on interface | [optional]
**is_default** | Option<**bool**> | Is default address on interface | [optional]
**is_preferred** | Option<**bool**> | Is preferred address on interface | [optional]
**is_primary** | Option<**bool**> | Is primary address on interface | [optional]
**is_secondary** | Option<**bool**> | Is secondary address on interface | [optional]
**is_standby** | Option<**bool**> | Is standby address on interface | [optional]
**ip_address** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**interface** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**vm_interface** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


