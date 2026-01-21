# Cable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**termination_a_type** | **String** |  | 
**termination_b_type** | **String** |  | 
**termination_a** | Option<[**crate::models::CableTermination**](CableTermination.md)> |  | [optional][readonly]
**termination_b** | Option<[**crate::models::CableTermination**](CableTermination.md)> |  | [optional][readonly]
**length_unit** | Option<[**crate::models::CableLengthUnit**](Cable_length_unit.md)> |  | [optional]
**r#type** | Option<[**crate::models::CableType**](Cable_type.md)> |  | [optional]
**termination_a_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**termination_b_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**label** | Option<**String**> |  | [optional]
**color** | Option<**String**> | RGB color in hexadecimal (e.g. 00ff00) | [optional]
**length** | Option<**i32**> |  | [optional]
**status** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


