# ConsoleServerPort

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<**String**> |  | [optional][readonly]
**display** | Option<**String**> | Human friendly display value | [optional][readonly]
**url** | Option<**String**> |  | [optional][readonly]
**natural_slug** | Option<**String**> |  | [optional][readonly]
**cable_peer_type** | Option<**String**> |  | [optional][readonly]
**cable_peer** | Option<[**crate::models::CableTermination**](CableTermination.md)> |  | [optional][readonly]
**connected_endpoint_type** | Option<**String**> |  | [optional][readonly]
**connected_endpoint** | Option<[**crate::models::PathEndpoint**](PathEndpoint.md)> |  | [optional][readonly]
**connected_endpoint_reachable** | Option<**bool**> |  | [optional][readonly]
**r#type** | Option<[**crate::models::ConsolePortType**](ConsolePort_type.md)> |  | [optional]
**name** | **String** |  | 
**label** | Option<**String**> | Physical label | [optional]
**description** | Option<**String**> |  | [optional]
**device** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**module** | Option<[**crate::models::BulkWritableCircuitRequestTenant**](BulkWritableCircuitRequest_tenant.md)> |  | [optional]
**cable** | Option<[**crate::models::CircuitCircuitTerminationA**](Circuit_circuit_termination_a.md)> |  | [optional]
**created** | Option<**String**> |  | [optional][readonly]
**last_updated** | Option<**String**> |  | [optional][readonly]
**notes_url** | Option<**String**> |  | [optional][readonly]
**custom_fields** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::BulkWritableCableRequestStatus>**](BulkWritableCableRequest_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


