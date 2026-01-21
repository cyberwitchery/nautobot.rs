# PatchedWritableCableRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**termination_a_type** | Option<**String**> |  | [optional]
**termination_b_type** | Option<**String**> |  | [optional]
**termination_a_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**termination_b_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**r#type** | Option<[**crate::models::PatchedWritableCableRequestType**](PatchedWritableCableRequest_type.md)> |  | [optional]
**label** | Option<**String**> |  | [optional]
**color** | Option<**String**> | RGB color in hexadecimal (e.g. 00ff00) | [optional]
**length** | Option<**i32**> |  | [optional]
**length_unit** | Option<[**crate::models::PatchedWritableCableRequestLengthUnit**](PatchedWritableCableRequest_length_unit.md)> |  | [optional]
**status** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


