# Service

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**protocol** | Option<[**crate::models::ServiceProtocol**](Service_protocol.md)> |  | [optional]
**ports** | **Vec<i32>** |  | 
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**device** | Option<[**crate::models::BulkWritableServiceRequestDevice**](BulkWritableServiceRequest_device.md)> |  | [optional]
**virtual_machine** | Option<[**crate::models::BulkWritableServiceRequestVirtualMachine**](BulkWritableServiceRequest_virtual_machine.md)> |  | [optional]
**ip_addresses** | Option<[**Vec<crate::models::IpAddresses>**](IP_addresses.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


