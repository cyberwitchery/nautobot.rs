# \UiApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ui_core_render_jinja_template_create**](UiApi.md#ui_core_render_jinja_template_create) | **POST** /ui/core/render-jinja-template/ | 



## ui_core_render_jinja_template_create

> crate::models::RenderJinja ui_core_render_jinja_template_create(render_jinja_request, format)


View to render a Jinja template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**render_jinja_request** | [**RenderJinjaRequest**](RenderJinjaRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**crate::models::RenderJinja**](RenderJinja.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

