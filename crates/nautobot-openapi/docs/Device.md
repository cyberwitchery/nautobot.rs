# Device

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**face** | Option<[**crate::models::DeviceFace**](Device_face.md)> |  | [optional]
**local_config_context_data** | Option<[**serde_json::Value**](.md)> |  | [optional]
**local_config_context_data_owner_object_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**serial** | Option<**String**> |  | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this device | [optional]
**position** | Option<**i32**> | The lowest-numbered unit occupied by the device | [optional]
**device_redundancy_group_priority** | Option<**i32**> | The priority the device has in the device redundancy group. | [optional]
**vc_position** | Option<**i32**> |  | [optional]
**vc_priority** | Option<**i32**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**local_config_context_schema** | Option<[**crate::models::BulkWritableConfigContextRequestConfigContextSchema**](BulkWritableConfigContextRequest_config_context_schema.md)> |  | [optional]
**local_config_context_data_owner_content_type** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**device_type** | [**crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage**](ApprovalWorkflowStageResponse_approval_workflow_stage.md) |  | 
**status** | [**crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage**](ApprovalWorkflowStageResponse_approval_workflow_stage.md) |  | 
**role** | [**crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage**](ApprovalWorkflowStageResponse_approval_workflow_stage.md) |  | 
**tenant** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**platform** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**location** | [**crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage**](ApprovalWorkflowStageResponse_approval_workflow_stage.md) |  | 
**rack** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**primary_ip4** | Option<[**crate::models::PrimaryIpv4**](Primary_IPv4.md)> |  | [optional]
**primary_ip6** | Option<[**crate::models::PrimaryIpv6**](Primary_IPv6.md)> |  | [optional]
**virtual_chassis** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**device_redundancy_group** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**software_version** | Option<[**crate::models::BulkWritableDeviceRequestSoftwareVersion**](BulkWritableDeviceRequest_software_version.md)> |  | [optional]
**secrets_group** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**controller_managed_device_group** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage>**](ApprovalWorkflowStageResponse_approval_workflow_stage.md)> |  | [optional]
**parent_bay** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


