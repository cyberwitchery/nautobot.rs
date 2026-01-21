# BulkWritableRackRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**r#type** | Option<[**crate::models::RackTypeChoices**](RackTypeChoices.md)> |  | [optional]
**width** | Option<**i32**> |  | [optional]
**outer_unit** | Option<[**crate::models::OuterUnitEnum**](OuterUnitEnum.md)> |  | [optional]
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
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


