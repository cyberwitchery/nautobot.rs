# PatchedBulkWritableWirelessNetworkRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**ssid** | Option<**String**> |  | [optional]
**mode** | Option<[**crate::models::WirelessNetworkModeChoices**](WirelessNetworkModeChoices.md)> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**authentication** | Option<[**crate::models::AuthenticationEnum**](AuthenticationEnum.md)> |  | [optional]
**hidden** | Option<**bool**> |  | [optional]
**secrets_group** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**tenant** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


