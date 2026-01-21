# Prefix

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**prefix** | **String** |  | 
**r#type** | Option<[**crate::models::PrefixType**](Prefix_type.md)> |  | [optional]
**network** | Option<**String**> | IPv4 or IPv6 network address | [optional][readonly]
**broadcast** | Option<**String**> | IPv4 or IPv6 broadcast address | [optional][readonly]
**prefix_length** | Option<**i32**> | Length of the Network prefix, in bits. | [optional][readonly]
**ip_version** | Option<**i32**> |  | [optional][readonly]
**date_allocated** | Option<**String**> | Date this prefix was allocated to an RIR, reserved in IPAM, etc. | [optional]
**description** | Option<**String**> |  | [optional]
**status** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**role** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**parent** | Option<[**crate::models::BulkWritablePrefixRequestParent**](BulkWritablePrefixRequest_parent.md)> |  | [optional]
**namespace** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**tenant** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**vlan** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**rir** | Option<[**crate::models::BulkWritablePrefixRequestRir**](BulkWritablePrefixRequest_rir.md)> |  | [optional]
**locations** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional][readonly]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**vrfs** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


