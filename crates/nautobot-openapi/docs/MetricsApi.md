# \MetricsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**metrics_retrieve**](MetricsApi.md#metrics_retrieve) | **GET** /metrics/ | 



## metrics_retrieve

> metrics_retrieve()


Exports /metrics. This overwrites the default django_prometheus view to inject metrics from Nautobot apps. Note that we cannot use `prometheus_django.ExportToDjangoView`, as that is a simple function, and we need access to the `prometheus_registry` variable that is defined inside of it.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

