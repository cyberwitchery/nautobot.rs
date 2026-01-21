# GitRepository

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**provided_contents** | Option<[**Vec<crate::models::BulkWritableGitRepositoryRequestProvidedContentsInner>**](BulkWritableGitRepositoryRequest_provided_contents_inner.md)> |  | [optional]
**name** | **String** |  | 
**slug** | Option<**String**> | Internal field name. Please use underscores rather than dashes in this key. | [optional]
**remote_url** | **String** | Only HTTP and HTTPS URLs are presently supported | 
**branch** | Option<**String**> | Branch, tag, or commit | [optional]
**current_head** | Option<**String**> | Commit hash of the most recent fetch from the selected branch. Used for syncing between workers. | [optional]
**secrets_group** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


