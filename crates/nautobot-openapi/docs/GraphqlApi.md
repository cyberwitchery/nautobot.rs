# \GraphqlApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**graphql_create**](GraphqlApi.md#graphql_create) | **POST** /graphql/ | 



## graphql_create

> crate::models::GraphqlCreate200Response graphql_create(graph_qlapi_request, format)


Query the database using a GraphQL query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**graph_qlapi_request** | [**GraphQlapiRequest**](GraphQlapiRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**crate::models::GraphqlCreate200Response**](graphql_create_200_response.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

