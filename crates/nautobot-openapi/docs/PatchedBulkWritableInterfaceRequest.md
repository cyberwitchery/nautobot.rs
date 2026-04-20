# PatchedBulkWritableInterfaceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**r#type** | Option<[**crate::models::InterfaceTypeChoices**](InterfaceTypeChoices.md)> |  | [optional]
**port_type** | Option<[**crate::models::PortTypeChoices**](PortTypeChoices.md)> |  | [optional]
**mode** | Option<[**crate::models::InterfaceModeChoices**](InterfaceModeChoices.md)> |  | [optional]
**mac_address** | Option<**String**> |  | [optional]
**speed** | Option<**i32**> |  | [optional]
**duplex** | Option<[**crate::models::DuplexEnum**](DuplexEnum.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**label** | Option<**String**> | Physical label | [optional]
**description** | Option<**String**> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**mtu** | Option<**i32**> |  | [optional]
**mgmt_only** | Option<**bool**> | This interface is used only for out-of-band management | [optional]
**device** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**module** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**status** | Option<[**crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage**](ApprovalWorkflowStageResponse_approval_workflow_stage.md)> |  | [optional]
**role** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**parent_interface** | Option<[**crate::models::BulkWritableInterfaceRequestParentInterface**](BulkWritableInterfaceRequest_parent_interface.md)> |  | [optional]
**bridge** | Option<[**crate::models::BridgeInterface**](Bridge_interface.md)> |  | [optional]
**lag** | Option<[**crate::models::ParentLag**](Parent_LAG.md)> |  | [optional]
**untagged_vlan** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**vrf** | Option<[**crate::models::ApprovalWorkflowUser**](ApprovalWorkflow_user.md)> |  | [optional]
**tagged_vlans** | Option<[**Vec<crate::models::TaggedVlans>**](Tagged_VLANs.md)> |  | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::ApprovalWorkflowDefinitionRequestRelationshipsValue>**](ApprovalWorkflowDefinitionRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::ApprovalWorkflowStageResponseApprovalWorkflowStage>**](ApprovalWorkflowStageResponse_approval_workflow_stage.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


