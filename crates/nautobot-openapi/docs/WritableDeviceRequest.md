# WritableDeviceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**local_config_context_data** | Option<[**serde_json::Value**](.md)> |  | [optional]
**local_config_context_data_owner_object_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**name** | Option<**String**> |  | [optional]
**serial** | Option<**String**> |  | [optional]
**asset_tag** | Option<**String**> | A unique tag used to identify this device | [optional]
**position** | Option<**i32**> | The lowest-numbered unit occupied by the device | [optional]
**face** | Option<[**crate::models::RackFace**](Rack_face.md)> |  | [optional]
**device_redundancy_group_priority** | Option<**i32**> | The priority the device has in the device redundancy group. | [optional]
**vc_position** | Option<**i32**> |  | [optional]
**vc_priority** | Option<**i32**> |  | [optional]
**comments** | Option<**String**> |  | [optional]
**local_config_context_schema** | Option<[**crate::models::BulkWritableConfigContextRequestConfigContextSchema**](BulkWritableConfigContextRequest_config_context_schema.md)> |  | [optional]
**local_config_context_data_owner_content_type** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**device_type** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**status** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**role** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**tenant** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**platform** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**location** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**rack** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**primary_ip4** | Option<[**crate::models::PrimaryIpv4**](Primary_IPv4.md)> |  | [optional]
**primary_ip6** | Option<[**crate::models::PrimaryIpv6**](Primary_IPv6.md)> |  | [optional]
**cluster** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**virtual_chassis** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**device_redundancy_group** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**software_version** | Option<[**crate::models::BulkWritableDeviceRequestSoftwareVersion**](BulkWritableDeviceRequest_software_version.md)> |  | [optional]
**secrets_group** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**controller_managed_device_group** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**software_image_files** | Option<[**Vec<crate::models::SoftwareImageFiles>**](Software_Image_Files.md)> | Override the software image files associated with the software version for this device | [optional]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**relationships** | Option<[**::std::collections::HashMap<String, crate::models::BulkWritableCableRequestRelationshipsValue>**](BulkWritableCableRequest_relationships_value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]
**parent_bay** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


