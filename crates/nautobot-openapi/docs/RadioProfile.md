# RadioProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**channel_width** | Option<[**Vec<crate::models::RadioProfileChannelWidthInner>**](RadioProfile_channel_width_inner.md)> |  | [optional]
**allowed_channel_list** | Option<**Vec<i32>**> |  | [optional]
**name** | **String** |  | 
**frequency** | Option<[**crate::models::BulkWritableRadioProfileRequestFrequency**](BulkWritableRadioProfileRequest_frequency.md)> |  | [optional]
**tx_power_min** | Option<**i32**> |  | [optional]
**tx_power_max** | Option<**i32**> |  | [optional]
**regulatory_domain** | [**crate::models::RegulatoryDomainEnum**](RegulatoryDomainEnum.md) |  | 
**rx_power_min** | Option<**i32**> |  | [optional]
**supported_data_rates** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


