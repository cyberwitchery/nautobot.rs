# PatchedWritableRackRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**facility_id** | Option<**String**> | Locally-assigned identifier | [optional]
**serial** | Option<**String**> |  | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this rack | [optional]
**r#type** | Option<[**crate::models::PatchedWritableRackRequestType**](PatchedWritableRackRequest_type.md)> |  | [optional]
**width** | Option<**i32**> |  | [optional]
**u_height** | Option<**i32**> | Height in rack units | [optional]
**desc_units** | Option<**bool**> | Units are numbered top-to-bottom | [optional]
**outer_width** | Option<**i32**> | Outer dimension of rack (width) | [optional]
**outer_depth** | Option<**i32**> | Outer dimension of rack (depth) | [optional]
**outer_unit** | Option<[**crate::models::PatchedWritableRackRequestOuterUnit**](PatchedWritableRackRequest_outer_unit.md)> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**status** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**role** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**location** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**rack_group** | Option<[**crate::models::BulkWritableRackRequestRackGroup**](BulkWritableRackRequest_rack_group.md)> |  | [optional]
**tenant** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


