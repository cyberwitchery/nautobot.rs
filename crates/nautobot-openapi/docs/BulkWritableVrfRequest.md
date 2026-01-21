# BulkWritableVrfRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | **String** |  | 
**rd** | Option<**String**> | Unique route distinguisher (as defined in RFC 4364) | [optional]
**description** | Option<**String**> |  | [optional]
**status** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**namespace** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**tenant** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**import_targets** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**export_targets** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


