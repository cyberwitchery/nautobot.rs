# Location

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**tree_depth** | Option<**i32**> |  | [optional][readonly]
**time_zone** | Option<**String**> |  | [optional]
**circuit_count** | Option<**i32**> |  | [optional][readonly]
**device_count** | Option<**i32**> |  | [optional][readonly]
**prefix_count** | Option<**i32**> |  | [optional][readonly]
**rack_count** | Option<**i32**> |  | [optional][readonly]
**virtual_machine_count** | Option<**i32**> |  | [optional][readonly]
**vlan_count** | Option<**i32**> |  | [optional][readonly]
**name** | **String** |  | 
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
**location_type** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**status** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**tenant** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


