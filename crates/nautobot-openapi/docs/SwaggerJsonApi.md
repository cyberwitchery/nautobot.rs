# \SwaggerJsonApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**swagger_period_json_retrieve**](SwaggerJsonApi.md#swagger_period_json_retrieve) | **GET** /swagger.json | 



## swagger_period_json_retrieve

> ::std::collections::HashMap<String, serde_json::Value> swagger_period_json_retrieve(lang, depth, exclude_m2m)


OpenApi3 schema for this API. Format can be selected via content negotiation.  - YAML: application/vnd.oai.openapi - JSON: application/vnd.oai.openapi+json

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lang** | Option<**String**> |  |  |
**depth** | Option<**i32**> | Serializer Depth |  |[default to 1]
**exclude_m2m** | Option<**bool**> | Exclude many-to-many fields from the response |  |[default to false]

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.oai.openapi+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

