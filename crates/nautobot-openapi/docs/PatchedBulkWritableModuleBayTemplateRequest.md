# PatchedBulkWritableModuleBayTemplateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | Option<**String**> |  | [optional]
**position** | Option<**String**> | The position of the module bay within the device or module | [optional]
**label** | Option<**String**> | Physical label | [optional]
**description** | Option<**String**> |  | [optional]
**requires_first_party_modules** | Option<**bool**> | This bay will only accept modules from the same manufacturer as the parent device or module | [optional]
**device_type** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**module_type** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**module_family** | Option<[**crate::models::BulkWritableModuleBayTemplateRequestModuleFamily**](BulkWritableModuleBayTemplateRequest_module_family.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


