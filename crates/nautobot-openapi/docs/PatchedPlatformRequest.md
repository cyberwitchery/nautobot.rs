# PatchedPlatformRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**network_driver** | Option<**String**> | The normalized network driver to use when interacting with devices, e.g. cisco_ios, arista_eos, etc. Library-specific driver names will be derived from this setting as appropriate | [optional]
**napalm_driver** | Option<**String**> | The name of the NAPALM driver to use when Nautobot internals interact with devices | [optional]
**napalm_args** | Option<[**serde_json::Value**](.md)> | Additional arguments to pass when initiating the NAPALM driver (JSON format) | [optional]
**description** | Option<**String**> |  | [optional]
**manufacturer** | Option<[**crate::models::BulkWritablePlatformRequestManufacturer**](BulkWritablePlatformRequest_manufacturer.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


