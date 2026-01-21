# Rack

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**r#type** | Option<[**crate::models::RackType**](Rack_type.md)> |  | [optional]
**width** | Option<[**crate::models::RackWidth**](Rack_width.md)> |  | [optional]
**outer_unit** | Option<[**crate::models::RackOuterUnit**](Rack_outer_unit.md)> |  | [optional]
**device_count** | Option<**i32**> |  | [optional][readonly]
**power_feed_count** | Option<**i32**> |  | [optional][readonly]
**name** | **String** |  | 
**facility_id** | Option<**String**> | Locally-assigned identifier | [optional]
**serial** | Option<**String**> |  | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this rack | [optional]
**u_height** | Option<**i32**> | Height in rack units | [optional]
**desc_units** | Option<**bool**> | Units are numbered top-to-bottom | [optional]
**outer_width** | Option<**i32**> | Outer dimension of rack (width) | [optional]
**outer_depth** | Option<**i32**> | Outer dimension of rack (depth) | [optional]
**comments** | Option<**String**> |  | [optional]
**status** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**role** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**location** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**rack_group** | Option<[**crate::models::BulkWritableRackRequestRackGroup**](BulkWritableRackRequest_rack_group.md)> |  | [optional]
**tenant** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


