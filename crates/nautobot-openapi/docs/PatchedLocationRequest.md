# PatchedLocationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**time_zone** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**facility** | Option<**String**> | Local facility ID or description | [optional]
**asn** | Option<**i64**> | 32-bit autonomous system number | [optional]
**physical_address** | Option<**String**> |  | [optional]
**shipping_address** | Option<**String**> |  | [optional]
**latitude** | Option<**String**> | GPS coordinate (latitude) | [optional]
**longitude** | Option<**String**> | GPS coordinate (longitude) | [optional]
**contact_name** | Option<**String**> |  | [optional]
**contact_phone** | Option<**String**> |  | [optional]
**contact_email** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**parent** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**location_type** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**status** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**tenant** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


