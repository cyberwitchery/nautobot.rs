# PatchedWebhookRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**content_types** | Option<**Vec<String>**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**type_create** | Option<**bool**> | Call this webhook when a matching object is created. | [optional]
**type_update** | Option<**bool**> | Call this webhook when a matching object is updated. | [optional]
**type_delete** | Option<**bool**> | Call this webhook when a matching object is deleted. | [optional]
**payload_url** | Option<**String**> | A POST will be sent to this URL when the webhook is called. | [optional]
**enabled** | Option<**bool**> |  | [optional]
**http_method** | Option<[**crate::models::HttpMethodEnum**](HttpMethodEnum.md)> |  | [optional]
**http_content_type** | Option<**String**> | The complete list of official content types is available here (<>). | [optional]
**additional_headers** | Option<**String**> | User-supplied HTTP headers to be sent with the request in addition to the HTTP content type. Headers should be defined in the format <code>Name: Value</code>. Jinja2 template processing is supported with the same context as the request body (below). | [optional]
**body_template** | Option<**String**> | Jinja2 template for a custom request body. If blank, a JSON object representing the change will be included. Available context data includes: <code>event</code>, <code>model</code>, <code>timestamp</code>, <code>username</code>, <code>request_id</code>, and <code>data</code>. | [optional]
**secret** | Option<**String**> | When provided, the request will include a 'X-Hook-Signature' header containing a HMAC hex digest of the payload body using the secret as the key. The secret is not transmitted in the request. | [optional]
**ssl_verification** | Option<**bool**> | Enable SSL certificate verification. Disable with caution! | [optional]
**ca_file_path** | Option<**String**> | The specific CA certificate file to use for SSL verification. Leave blank to use the system defaults. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


