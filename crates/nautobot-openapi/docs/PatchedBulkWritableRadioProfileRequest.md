# PatchedBulkWritableRadioProfileRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**channel_width** | Option<**Vec<i32>**> |  | [optional]
**allowed_channel_list** | Option<**Vec<i32>**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**frequency** | Option<[**crate::models::BulkWritableRadioProfileRequestFrequency**](BulkWritableRadioProfileRequest_frequency.md)> |  | [optional]
**tx_power_min** | Option<**i32**> |  | [optional]
**tx_power_max** | Option<**i32**> |  | [optional]
**regulatory_domain** | Option<[**crate::models::RegulatoryDomainEnum**](RegulatoryDomainEnum.md)> |  | [optional]
**rx_power_min** | Option<**i32**> |  | [optional]
**supported_data_rates** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


