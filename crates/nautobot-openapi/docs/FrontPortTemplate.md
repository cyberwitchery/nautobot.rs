# FrontPortTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**r#type** | [**crate::models::FrontPortType**](FrontPort_type.md) |  | 
**name** | **String** |  | 
**label** | Option<**String**> | Physical label | [optional]
**description** | Option<**String**> |  | [optional]
**rear_port_position** | Option<**i32**> |  | [optional]
**device_type** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**module_type** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**rear_port_template** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


