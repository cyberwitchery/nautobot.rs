# SecretsGroupAssociation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**access_type** | [**crate::models::AccessTypeEnum**](AccessTypeEnum.md) |  | 
**secret_type** | [**crate::models::SecretTypeEnum**](SecretTypeEnum.md) |  | 
**secrets_group** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**secret** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


