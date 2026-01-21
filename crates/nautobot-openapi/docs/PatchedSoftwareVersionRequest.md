# PatchedSoftwareVersionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**version** | Option<**String**> |  | [optional]
**alias** | Option<**String**> | Optional alternative label for this version | [optional]
**release_date** | Option<[**String**](string.md)> |  | [optional]
**end_of_support_date** | Option<[**String**](string.md)> |  | [optional]
**documentation_url** | Option<**String**> |  | [optional]
**long_term_support** | Option<**bool**> | Is a Long Term Support version | [optional]
**pre_release** | Option<**bool**> | Is a Pre-Release version | [optional]
**platform** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**status** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


