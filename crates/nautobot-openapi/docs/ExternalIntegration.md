# ExternalIntegration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**name** | **String** |  | 
**remote_url** | **String** |  | 
**verify_ssl** | Option<**bool**> | Verify SSL certificates when connecting to the remote system | [optional]
**timeout** | Option<**i32**> | Number of seconds to wait for a response | [optional]
**extra_config** | Option<[**serde_json::Value**](.md)> | Optional user-defined JSON data for this integration | [optional]
**http_method** | Option<[**crate::models::BulkWritableExternalIntegrationRequestHttpMethod**](BulkWritableExternalIntegrationRequest_http_method.md)> |  | [optional]
**headers** | Option<[**serde_json::Value**](.md)> | Headers for the HTTP request | [optional]
**ca_file_path** | Option<**String**> |  | [optional]
**secrets_group** | Option<[**crate::models::BulkWritableExternalIntegrationRequestSecretsGroup**](BulkWritableExternalIntegrationRequest_secrets_group.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


