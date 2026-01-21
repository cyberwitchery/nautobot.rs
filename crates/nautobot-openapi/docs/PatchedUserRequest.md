# PatchedUserRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**password** | Option<**String**> |  | [optional]
**last_login** | Option<**String**> |  | [optional]
**is_superuser** | Option<**bool**> | Designates that this user has all permissions without explicitly assigning them. | [optional]
**username** | Option<**String**> | Required. 150 characters or fewer. Letters, digits and @/./+/-/_ only. | [optional]
**first_name** | Option<**String**> |  | [optional]
**last_name** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**is_staff** | Option<**bool**> | Designates whether the user can log into this admin site. | [optional]
**is_active** | Option<**bool**> | Designates whether this user should be treated as active. Unselect this instead of deleting accounts. | [optional]
**date_joined** | Option<**String**> |  | [optional]
**config_data** | Option<[**serde_json::Value**](.md)> |  | [optional]
**groups** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> | The groups this user belongs to. A user will get all permissions granted to each of their groups. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


