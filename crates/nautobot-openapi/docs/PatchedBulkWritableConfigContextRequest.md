# PatchedBulkWritableConfigContextRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**owner_content_type** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**owner_object_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**weight** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**is_active** | Option<**bool**> |  | [optional]
**data** | Option<[**serde_json::Value**](.md)> |  | [optional]
**config_context_schema** | Option<[**crate::models::BulkWritableConfigContextRequestConfigContextSchema**](BulkWritableConfigContextRequest_config_context_schema.md)> |  | [optional]
**locations** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**roles** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**device_types** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**device_redundancy_groups** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**platforms** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**cluster_groups** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**clusters** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**tenant_groups** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**tenants** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


