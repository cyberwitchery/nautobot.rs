# WritableServiceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**ports** | **Vec<i32>** |  | 
**name** | **String** |  | 
**protocol** | [**crate::models::ServiceProtocolChoices**](ServiceProtocolChoices.md) |  | 
**description** | Option<**String**> |  | [optional]
**device** | Option<[**crate::models::BulkWritableServiceRequestDevice**](BulkWritableServiceRequest_device.md)> |  | [optional]
**virtual_machine** | Option<[**crate::models::BulkWritableServiceRequestVirtualMachine**](BulkWritableServiceRequest_virtual_machine.md)> |  | [optional]
**ip_addresses** | Option<[**Vec<crate::models::IpAddresses>**](IP_addresses.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


