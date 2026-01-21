# \StatusApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**status_retrieve**](StatusApi.md#status_retrieve) | **GET** /status/ | 



## status_retrieve

> crate::models::StatusRetrieve200Response status_retrieve(format, depth, exclude_m2m)


A lightweight read-only endpoint for conveying the current operational status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> |  |  |
**depth** | Option<**i32**> | Serializer Depth |  |[default to 1]
**exclude_m2m** | Option<**bool**> | Exclude many-to-many fields from the response |  |[default to false]

### Return type

[**crate::models::StatusRetrieve200Response**](status_retrieve_200_response.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

