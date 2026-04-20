# SoftwareImageFileRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**image_file_name** | **String** |  | 
**image_file_checksum** | Option<**String**> |  | [optional]
**hashing_algorithm** | Option<[**crate::models::BulkWritableSoftwareImageFileRequestHashingAlgorithm**](BulkWritableSoftwareImageFileRequest_hashing_algorithm.md)> |  | [optional]
**image_file_size** | Option<**i64**> | Image file size in bytes | [optional]
**download_url** | Option<**String**> |  | [optional]
**default_image** | Option<**bool**> | Is the default image for this software version | [optional]
**software_version** | [**crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage**](ApprovalWorkflowStageResponse_approval_workflow_stage.md) |  | 
**external_integration** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**status** | [**crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage**](ApprovalWorkflowStageResponse_approval_workflow_stage.md) |  | 
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage>**](ApprovalWorkflowStageResponse_approval_workflow_stage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


