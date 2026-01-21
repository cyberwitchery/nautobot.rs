# Tenant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**circuit_count** | Option<**i32**> |  | [optional][readonly]
**device_count** | Option<**i32**> |  | [optional][readonly]
**ipaddress_count** | Option<**i32**> |  | [optional][readonly]
**prefix_count** | Option<**i32**> |  | [optional][readonly]
**rack_count** | Option<**i32**> |  | [optional][readonly]
**virtualmachine_count** | Option<**i32**> |  | [optional][readonly]
**vlan_count** | Option<**i32**> |  | [optional][readonly]
**vrf_count** | Option<**i32**> |  | [optional][readonly]
**cluster_count** | Option<**i32**> |  | [optional][readonly]
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**tenant_group** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


