# BulkWritablePrefixRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**prefix** | **String** |  | 
**r#type** | Option<[**crate::models::PrefixTypeChoices**](PrefixTypeChoices.md)> |  | [optional][default to Network]
**location** | Option<[**crate::models::BulkWritablePrefixRequestLocation**](BulkWritablePrefixRequest_location.md)> |  | [optional]
**date_allocated** | Option<**String**> | Date this prefix was allocated to an RIR, reserved in IPAM, etc. | [optional]
**description** | Option<**String**> |  | [optional]
**status** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**role** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**parent** | Option<[**crate::models::BulkWritablePrefixRequestParent**](BulkWritablePrefixRequest_parent.md)> |  | [optional]
**namespace** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**tenant** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**vlan** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**rir** | Option<[**crate::models::BulkWritablePrefixRequestRir**](BulkWritablePrefixRequest_rir.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


