# PatchedBulkWritableModuleBayRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**requires_first_party_modules** | Option<**bool**> | This bay will only accept modules from the same manufacturer as the parent device or module | [optional]
**name** | Option<**String**> |  | [optional]
**position** | Option<**String**> | The position of the module bay within the parent device/module | [optional]
**label** | Option<**String**> | Physical label | [optional]
**description** | Option<**String**> |  | [optional]
**parent_device** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**parent_module** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**module_family** | Option<[**crate::models::BulkWritableModuleBayRequestModuleFamily**](BulkWritableModuleBayRequest_module_family.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


