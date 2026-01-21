# PatchedSoftwareImageFileRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**image_file_name** | Option<**String**> |  | [optional]
**image_file_checksum** | Option<**String**> |  | [optional]
**hashing_algorithm** | Option<[**crate::models::BulkWritableSoftwareImageFileRequestHashingAlgorithm**](BulkWritableSoftwareImageFileRequest_hashing_algorithm.md)> |  | [optional]
**image_file_size** | Option<**i64**> | Image file size in bytes | [optional]
**download_url** | Option<**String**> |  | [optional]
**default_image** | Option<**bool**> | Is the default image for this software version | [optional]
**software_version** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**external_integration** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**status** | Option<[**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


