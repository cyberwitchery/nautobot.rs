# DeviceType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**subdevice_role** | Option<[**crate::models::DeviceTypeSubdeviceRole**](DeviceType_subdevice_role.md)> |  | [optional]
**front_image** | Option<**String**> |  | [optional]
**rear_image** | Option<**String**> |  | [optional]
**device_count** | Option<**i32**> |  | [optional][readonly]
**model** | **String** |  | 
**part_number** | Option<**String**> | Discrete part number (optional) | [optional]
**u_height** | Option<**i32**> |  | [optional]
**is_full_depth** | Option<**bool**> | Device consumes both front and rear rack faces | [optional]
**comments** | Option<**String**> |  | [optional]
**manufacturer** | [**crate::models::BulkWritableCableRequestStatus**](BulkWritableCableRequest_status.md) |  | 
**device_family** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**software_image_files** | Option<[**Vec<crate::models::SoftwareImageFiles>**](Software_Image_Files.md)> |  | [optional][readonly]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


