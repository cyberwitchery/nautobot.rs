# \UsersApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_config_retrieve**](UsersApi.md#users_config_retrieve) | **GET** /users/config/ | 
[**users_groups_bulk_destroy**](UsersApi.md#users_groups_bulk_destroy) | **DELETE** /users/groups/ | 
[**users_groups_bulk_partial_update**](UsersApi.md#users_groups_bulk_partial_update) | **PATCH** /users/groups/ | 
[**users_groups_bulk_update**](UsersApi.md#users_groups_bulk_update) | **PUT** /users/groups/ | 
[**users_groups_create**](UsersApi.md#users_groups_create) | **POST** /users/groups/ | 
[**users_groups_destroy**](UsersApi.md#users_groups_destroy) | **DELETE** /users/groups/{id}/ | 
[**users_groups_list**](UsersApi.md#users_groups_list) | **GET** /users/groups/ | 
[**users_groups_partial_update**](UsersApi.md#users_groups_partial_update) | **PATCH** /users/groups/{id}/ | 
[**users_groups_retrieve**](UsersApi.md#users_groups_retrieve) | **GET** /users/groups/{id}/ | 
[**users_groups_update**](UsersApi.md#users_groups_update) | **PUT** /users/groups/{id}/ | 
[**users_permissions_bulk_destroy**](UsersApi.md#users_permissions_bulk_destroy) | **DELETE** /users/permissions/ | 
[**users_permissions_bulk_partial_update**](UsersApi.md#users_permissions_bulk_partial_update) | **PATCH** /users/permissions/ | 
[**users_permissions_bulk_update**](UsersApi.md#users_permissions_bulk_update) | **PUT** /users/permissions/ | 
[**users_permissions_create**](UsersApi.md#users_permissions_create) | **POST** /users/permissions/ | 
[**users_permissions_destroy**](UsersApi.md#users_permissions_destroy) | **DELETE** /users/permissions/{id}/ | 
[**users_permissions_list**](UsersApi.md#users_permissions_list) | **GET** /users/permissions/ | 
[**users_permissions_partial_update**](UsersApi.md#users_permissions_partial_update) | **PATCH** /users/permissions/{id}/ | 
[**users_permissions_retrieve**](UsersApi.md#users_permissions_retrieve) | **GET** /users/permissions/{id}/ | 
[**users_permissions_update**](UsersApi.md#users_permissions_update) | **PUT** /users/permissions/{id}/ | 
[**users_tokens_bulk_destroy**](UsersApi.md#users_tokens_bulk_destroy) | **DELETE** /users/tokens/ | 
[**users_tokens_bulk_partial_update**](UsersApi.md#users_tokens_bulk_partial_update) | **PATCH** /users/tokens/ | 
[**users_tokens_bulk_update**](UsersApi.md#users_tokens_bulk_update) | **PUT** /users/tokens/ | 
[**users_tokens_create**](UsersApi.md#users_tokens_create) | **POST** /users/tokens/ | 
[**users_tokens_destroy**](UsersApi.md#users_tokens_destroy) | **DELETE** /users/tokens/{id}/ | 
[**users_tokens_list**](UsersApi.md#users_tokens_list) | **GET** /users/tokens/ | 
[**users_tokens_partial_update**](UsersApi.md#users_tokens_partial_update) | **PATCH** /users/tokens/{id}/ | 
[**users_tokens_retrieve**](UsersApi.md#users_tokens_retrieve) | **GET** /users/tokens/{id}/ | 
[**users_tokens_update**](UsersApi.md#users_tokens_update) | **PUT** /users/tokens/{id}/ | 
[**users_users_bulk_destroy**](UsersApi.md#users_users_bulk_destroy) | **DELETE** /users/users/ | 
[**users_users_bulk_partial_update**](UsersApi.md#users_users_bulk_partial_update) | **PATCH** /users/users/ | 
[**users_users_bulk_update**](UsersApi.md#users_users_bulk_update) | **PUT** /users/users/ | 
[**users_users_create**](UsersApi.md#users_users_create) | **POST** /users/users/ | 
[**users_users_destroy**](UsersApi.md#users_users_destroy) | **DELETE** /users/users/{id}/ | 
[**users_users_list**](UsersApi.md#users_users_list) | **GET** /users/users/ | 
[**users_users_partial_update**](UsersApi.md#users_users_partial_update) | **PATCH** /users/users/{id}/ | 
[**users_users_retrieve**](UsersApi.md#users_users_retrieve) | **GET** /users/users/{id}/ | 
[**users_users_update**](UsersApi.md#users_users_update) | **PUT** /users/users/{id}/ | 



## users_config_retrieve

> ::std::collections::HashMap<String, serde_json::Value> users_config_retrieve(format, depth, exclude_m2m)


Return the config_data for the currently authenticated User.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> |  |  |
**depth** | Option<**i32**> | Serializer Depth |  |[default to 1]
**exclude_m2m** | Option<**bool**> | Exclude many-to-many fields from the response |  |[default to false]

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_bulk_destroy

> users_groups_bulk_destroy(bulk_operation_integer_id_request, format)


Destroy a list of group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_operation_integer_id_request** | [**Vec<crate::models::BulkOperationIntegerIdRequest>**](BulkOperationIntegerIDRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_bulk_partial_update

> Vec<crate::models::Group> users_groups_bulk_partial_update(patched_bulk_writable_group_request, format)


Partial update a list of group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patched_bulk_writable_group_request** | [**Vec<crate::models::PatchedBulkWritableGroupRequest>**](PatchedBulkWritableGroupRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_bulk_update

> Vec<crate::models::Group> users_groups_bulk_update(bulk_writable_group_request, format)


Update a list of group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_writable_group_request** | [**Vec<crate::models::BulkWritableGroupRequest>**](BulkWritableGroupRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_create

> crate::models::Group users_groups_create(group_request, format)


Create one or more group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_request** | [**GroupRequest**](GroupRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_destroy

> users_groups_destroy(id, format)


Destroy a group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this group. | [required] |
**format** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_list

> crate::models::PaginatedGroupList users_groups_list(format, id, id__gt, id__gte, id__lt, id__lte, id__n, limit, name, name__ic, name__ie, name__iew, name__ire, name__isw, name__n, name__nic, name__nie, name__niew, name__nire, name__nisw, name__nre, name__re, offset, q, sort, depth, exclude_m2m)


Retrieve a list of group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> |  |  |
**id** | Option<[**Vec<i32>**](i32.md)> | Unique object identifier, either a UUID primary key or a composite key. |  |
**id__gt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__gte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lt** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__lte** | Option<[**Vec<i32>**](i32.md)> |  |  |
**id__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**name** | Option<[**Vec<String>**](String.md)> |  |  |
**name__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**name__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**name__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**name__ire** | Option<[**Vec<String>**](String.md)> |  |  |
**name__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**name__n** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**name__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nire** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nre** | Option<[**Vec<String>**](String.md)> |  |  |
**name__re** | Option<[**Vec<String>**](String.md)> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**q** | Option<**String**> | Search |  |
**sort** | Option<**String**> | Which field to use when ordering the results. |  |
**depth** | Option<**i32**> | Serializer Depth |  |[default to 1]
**exclude_m2m** | Option<**bool**> | Exclude many-to-many fields from the response |  |[default to false]

### Return type

[**crate::models::PaginatedGroupList**](PaginatedGroupList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_partial_update

> crate::models::Group users_groups_partial_update(id, format, patched_group_request)


Partial update a group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this group. | [required] |
**format** | Option<**String**> |  |  |
**patched_group_request** | Option<[**PatchedGroupRequest**](PatchedGroupRequest.md)> |  |  |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_retrieve

> crate::models::Group users_groups_retrieve(id, format, depth, exclude_m2m)


Retrieve a group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this group. | [required] |
**format** | Option<**String**> |  |  |
**depth** | Option<**i32**> | Serializer Depth |  |[default to 1]
**exclude_m2m** | Option<**bool**> | Exclude many-to-many fields from the response |  |[default to false]

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_groups_update

> crate::models::Group users_groups_update(id, group_request, format)


Update a group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this group. | [required] |
**group_request** | [**GroupRequest**](GroupRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_bulk_destroy

> users_permissions_bulk_destroy(bulk_operation_request, format)


Destroy a list of permission objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_operation_request** | [**Vec<crate::models::BulkOperationRequest>**](BulkOperationRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_bulk_partial_update

> Vec<crate::models::ObjectPermission> users_permissions_bulk_partial_update(patched_bulk_writable_object_permission_request, format)


Partial update a list of permission objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patched_bulk_writable_object_permission_request** | [**Vec<crate::models::PatchedBulkWritableObjectPermissionRequest>**](PatchedBulkWritableObjectPermissionRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ObjectPermission>**](ObjectPermission.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_bulk_update

> Vec<crate::models::ObjectPermission> users_permissions_bulk_update(bulk_writable_object_permission_request, format)


Update a list of permission objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_writable_object_permission_request** | [**Vec<crate::models::BulkWritableObjectPermissionRequest>**](BulkWritableObjectPermissionRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::ObjectPermission>**](ObjectPermission.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_create

> crate::models::ObjectPermission users_permissions_create(object_permission_request, format)


Create one or more permission objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_permission_request** | [**ObjectPermissionRequest**](ObjectPermissionRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**crate::models::ObjectPermission**](ObjectPermission.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_destroy

> users_permissions_destroy(id, format)


Destroy a permission object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this permission. | [required] |
**format** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_list

> crate::models::PaginatedObjectPermissionList users_permissions_list(description, description__ic, description__ie, description__iew, description__ire, description__isw, description__n, description__nic, description__nie, description__niew, description__nire, description__nisw, description__nre, description__re, enabled, format, groups, groups__n, groups_id, groups_id__n, id, id__n, limit, name, name__ic, name__ie, name__iew, name__ire, name__isw, name__n, name__nic, name__nie, name__niew, name__nire, name__nisw, name__nre, name__re, object_types, object_types__n, offset, q, sort, users, users__n, depth, exclude_m2m)


Retrieve a list of permission objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**description** | Option<[**Vec<String>**](String.md)> |  |  |
**description__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**description__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**description__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**description__ire** | Option<[**Vec<String>**](String.md)> |  |  |
**description__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**description__n** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**description__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nire** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nre** | Option<[**Vec<String>**](String.md)> |  |  |
**description__re** | Option<[**Vec<String>**](String.md)> |  |  |
**enabled** | Option<**bool**> |  |  |
**format** | Option<**String**> |  |  |
**groups** | Option<[**Vec<String>**](String.md)> | Group (name) |  |
**groups__n** | Option<[**Vec<String>**](String.md)> | Exclude Group (name) |  |
**groups_id** | Option<[**Vec<i32>**](i32.md)> | Group (ID) |  |
**groups_id__n** | Option<[**Vec<i32>**](i32.md)> | Exclude Group (ID) |  |
**id** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Unique object identifier, either a UUID primary key or a composite key. |  |
**id__n** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**name** | Option<[**Vec<String>**](String.md)> |  |  |
**name__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**name__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**name__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**name__ire** | Option<[**Vec<String>**](String.md)> |  |  |
**name__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**name__n** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**name__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nire** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**name__nre** | Option<[**Vec<String>**](String.md)> |  |  |
**name__re** | Option<[**Vec<String>**](String.md)> |  |  |
**object_types** | Option<[**Vec<i32>**](i32.md)> |  |  |
**object_types__n** | Option<[**Vec<i32>**](i32.md)> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**q** | Option<**String**> | Search |  |
**sort** | Option<**String**> | Which field to use when ordering the results. |  |
**users** | Option<[**Vec<String>**](String.md)> |  |  |
**users__n** | Option<[**Vec<String>**](String.md)> |  |  |
**depth** | Option<**i32**> | Serializer Depth |  |[default to 1]
**exclude_m2m** | Option<**bool**> | Exclude many-to-many fields from the response |  |[default to false]

### Return type

[**crate::models::PaginatedObjectPermissionList**](PaginatedObjectPermissionList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_partial_update

> crate::models::ObjectPermission users_permissions_partial_update(id, format, patched_object_permission_request)


Partial update a permission object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this permission. | [required] |
**format** | Option<**String**> |  |  |
**patched_object_permission_request** | Option<[**PatchedObjectPermissionRequest**](PatchedObjectPermissionRequest.md)> |  |  |

### Return type

[**crate::models::ObjectPermission**](ObjectPermission.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_retrieve

> crate::models::ObjectPermission users_permissions_retrieve(id, format, depth, exclude_m2m)


Retrieve a permission object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this permission. | [required] |
**format** | Option<**String**> |  |  |
**depth** | Option<**i32**> | Serializer Depth |  |[default to 1]
**exclude_m2m** | Option<**bool**> | Exclude many-to-many fields from the response |  |[default to false]

### Return type

[**crate::models::ObjectPermission**](ObjectPermission.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_permissions_update

> crate::models::ObjectPermission users_permissions_update(id, object_permission_request, format)


Update a permission object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this permission. | [required] |
**object_permission_request** | [**ObjectPermissionRequest**](ObjectPermissionRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**crate::models::ObjectPermission**](ObjectPermission.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_bulk_destroy

> users_tokens_bulk_destroy(bulk_operation_request, format)


Destroy a list of token objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_operation_request** | [**Vec<crate::models::BulkOperationRequest>**](BulkOperationRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_bulk_partial_update

> Vec<crate::models::Token> users_tokens_bulk_partial_update(patched_bulk_writable_token_request, format)


Partial update a list of token objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patched_bulk_writable_token_request** | [**Vec<crate::models::PatchedBulkWritableTokenRequest>**](PatchedBulkWritableTokenRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Token>**](Token.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_bulk_update

> Vec<crate::models::Token> users_tokens_bulk_update(bulk_writable_token_request, format)


Update a list of token objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_writable_token_request** | [**Vec<crate::models::BulkWritableTokenRequest>**](BulkWritableTokenRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Token>**](Token.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_create

> crate::models::Token users_tokens_create(format, token_request)


Create one or more token objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> |  |  |
**token_request** | Option<[**TokenRequest**](TokenRequest.md)> |  |  |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_destroy

> users_tokens_destroy(id, format)


Destroy a token object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this token. | [required] |
**format** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_list

> crate::models::PaginatedTokenList users_tokens_list(created, created__gt, created__gte, created__lt, created__lte, created__n, description, description__ic, description__ie, description__iew, description__ire, description__isw, description__n, description__nic, description__nie, description__niew, description__nire, description__nisw, description__nre, description__re, expires, expires__gt, expires__gte, expires__isnull, expires__lt, expires__lte, expires__n, format, id, id__n, key, key__ic, key__ie, key__iew, key__ire, key__isw, key__n, key__nic, key__nie, key__niew, key__nire, key__nisw, key__nre, key__re, limit, offset, q, sort, write_enabled, depth, exclude_m2m)


Retrieve a list of token objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created** | Option<[**Vec<String>**](String.md)> |  |  |
**created__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**created__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**created__lt** | Option<[**Vec<String>**](String.md)> |  |  |
**created__lte** | Option<[**Vec<String>**](String.md)> |  |  |
**created__n** | Option<[**Vec<String>**](String.md)> |  |  |
**description** | Option<[**Vec<String>**](String.md)> |  |  |
**description__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**description__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**description__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**description__ire** | Option<[**Vec<String>**](String.md)> |  |  |
**description__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**description__n** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**description__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nire** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**description__nre** | Option<[**Vec<String>**](String.md)> |  |  |
**description__re** | Option<[**Vec<String>**](String.md)> |  |  |
**expires** | Option<[**Vec<String>**](String.md)> |  |  |
**expires__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**expires__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**expires__isnull** | Option<**bool**> |  |  |
**expires__lt** | Option<[**Vec<String>**](String.md)> |  |  |
**expires__lte** | Option<[**Vec<String>**](String.md)> |  |  |
**expires__n** | Option<[**Vec<String>**](String.md)> |  |  |
**format** | Option<**String**> |  |  |
**id** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Unique object identifier, either a UUID primary key or a composite key. |  |
**id__n** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**key** | Option<[**Vec<String>**](String.md)> |  |  |
**key__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**key__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**key__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**key__ire** | Option<[**Vec<String>**](String.md)> |  |  |
**key__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**key__n** | Option<[**Vec<String>**](String.md)> |  |  |
**key__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**key__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**key__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**key__nire** | Option<[**Vec<String>**](String.md)> |  |  |
**key__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**key__nre** | Option<[**Vec<String>**](String.md)> |  |  |
**key__re** | Option<[**Vec<String>**](String.md)> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**q** | Option<**String**> | Search |  |
**sort** | Option<**String**> | Which field to use when ordering the results. |  |
**write_enabled** | Option<**bool**> |  |  |
**depth** | Option<**i32**> | Serializer Depth |  |[default to 1]
**exclude_m2m** | Option<**bool**> | Exclude many-to-many fields from the response |  |[default to false]

### Return type

[**crate::models::PaginatedTokenList**](PaginatedTokenList.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_partial_update

> crate::models::Token users_tokens_partial_update(id, format, patched_token_request)


Partial update a token object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this token. | [required] |
**format** | Option<**String**> |  |  |
**patched_token_request** | Option<[**PatchedTokenRequest**](PatchedTokenRequest.md)> |  |  |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_retrieve

> crate::models::Token users_tokens_retrieve(id, format, depth, exclude_m2m)


Retrieve a token object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this token. | [required] |
**format** | Option<**String**> |  |  |
**depth** | Option<**i32**> | Serializer Depth |  |[default to 1]
**exclude_m2m** | Option<**bool**> | Exclude many-to-many fields from the response |  |[default to false]

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_tokens_update

> crate::models::Token users_tokens_update(id, format, token_request)


Update a token object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this token. | [required] |
**format** | Option<**String**> |  |  |
**token_request** | Option<[**TokenRequest**](TokenRequest.md)> |  |  |

### Return type

[**crate::models::Token**](Token.md)

### Authorization

[basicAuth](../README.md#basicAuth), [cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_bulk_destroy

> users_users_bulk_destroy(bulk_operation_request, format)


Destroy a list of user objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_operation_request** | [**Vec<crate::models::BulkOperationRequest>**](BulkOperationRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_bulk_partial_update

> Vec<crate::models::User> users_users_bulk_partial_update(patched_bulk_writable_user_request, format)


Partial update a list of user objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patched_bulk_writable_user_request** | [**Vec<crate::models::PatchedBulkWritableUserRequest>**](PatchedBulkWritableUserRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_bulk_update

> Vec<crate::models::User> users_users_bulk_update(bulk_writable_user_request, format)


Update a list of user objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_writable_user_request** | [**Vec<crate::models::BulkWritableUserRequest>**](BulkWritableUserRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_create

> crate::models::User users_users_create(user_request, format)


Create one or more user objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_request** | [**UserRequest**](UserRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_destroy

> users_users_destroy(id, format)


Destroy a user object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this user. | [required] |
**format** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_list

> crate::models::PaginatedUserList users_users_list(email, email__ic, email__ie, email__iew, email__ire, email__isw, email__n, email__nic, email__nie, email__niew, email__nire, email__nisw, email__nre, email__re, first_name, first_name__ic, first_name__ie, first_name__iew, first_name__ire, first_name__isw, first_name__n, first_name__nic, first_name__nie, first_name__niew, first_name__nire, first_name__nisw, first_name__nre, first_name__re, format, groups, groups__n, groups_id, groups_id__n, has_object_changes, has_object_permissions, has_rack_reservations, id, id__n, is_active, is_staff, last_name, last_name__ic, last_name__ie, last_name__iew, last_name__ire, last_name__isw, last_name__n, last_name__nic, last_name__nie, last_name__niew, last_name__nire, last_name__nisw, last_name__nre, last_name__re, limit, object_changes, object_changes__isnull, object_changes__n, object_permissions, object_permissions__isnull, object_permissions__n, offset, q, rack_reservations_id, rack_reservations_id__isnull, rack_reservations_id__n, sort, username, username__ic, username__ie, username__iew, username__ire, username__isw, username__n, username__nic, username__nie, username__niew, username__nire, username__nisw, username__nre, username__re, depth, exclude_m2m)


Retrieve a list of user objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | Option<[**Vec<String>**](String.md)> |  |  |
**email__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**email__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**email__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**email__ire** | Option<[**Vec<String>**](String.md)> |  |  |
**email__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**email__n** | Option<[**Vec<String>**](String.md)> |  |  |
**email__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**email__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**email__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**email__nire** | Option<[**Vec<String>**](String.md)> |  |  |
**email__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**email__nre** | Option<[**Vec<String>**](String.md)> |  |  |
**email__re** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__ire** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__n** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__nire** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__nre** | Option<[**Vec<String>**](String.md)> |  |  |
**first_name__re** | Option<[**Vec<String>**](String.md)> |  |  |
**format** | Option<**String**> |  |  |
**groups** | Option<[**Vec<String>**](String.md)> | Group (name) |  |
**groups__n** | Option<[**Vec<String>**](String.md)> | Exclude Group (name) |  |
**groups_id** | Option<[**Vec<i32>**](i32.md)> | Group (ID) |  |
**groups_id__n** | Option<[**Vec<i32>**](i32.md)> | Exclude Group (ID) |  |
**has_object_changes** | Option<**bool**> | Has Changes |  |
**has_object_permissions** | Option<**bool**> | Has object permissions |  |
**has_rack_reservations** | Option<**bool**> | Has Rack Reservations |  |
**id** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Unique object identifier, either a UUID primary key or a composite key. |  |
**id__n** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**is_active** | Option<**bool**> |  |  |
**is_staff** | Option<**bool**> |  |  |
**last_name** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__ire** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__n** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__nire** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__nre** | Option<[**Vec<String>**](String.md)> |  |  |
**last_name__re** | Option<[**Vec<String>**](String.md)> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**object_changes** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Object Changes (ID) |  |
**object_changes__isnull** | Option<**bool**> | Object Changes (ID) is null |  |
**object_changes__n** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Exclude Object Changes (ID) |  |
**object_permissions** | Option<[**Vec<String>**](String.md)> |  |  |
**object_permissions__isnull** | Option<**bool**> | Object Permission (ID or name) is null |  |
**object_permissions__n** | Option<[**Vec<String>**](String.md)> |  |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**q** | Option<**String**> | Search |  |
**rack_reservations_id** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Rack Reservation (ID) |  |
**rack_reservations_id__isnull** | Option<**bool**> | Rack Reservation (ID) is null |  |
**rack_reservations_id__n** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Exclude Rack Reservation (ID) |  |
**sort** | Option<**String**> | Which field to use when ordering the results. |  |
**username** | Option<[**Vec<String>**](String.md)> |  |  |
**username__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**username__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**username__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**username__ire** | Option<[**Vec<String>**](String.md)> |  |  |
**username__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**username__n** | Option<[**Vec<String>**](String.md)> |  |  |
**username__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**username__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**username__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**username__nire** | Option<[**Vec<String>**](String.md)> |  |  |
**username__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**username__nre** | Option<[**Vec<String>**](String.md)> |  |  |
**username__re** | Option<[**Vec<String>**](String.md)> |  |  |
**depth** | Option<**i32**> | Serializer Depth |  |[default to 1]
**exclude_m2m** | Option<**bool**> | Exclude many-to-many fields from the response |  |[default to false]

### Return type

[**crate::models::PaginatedUserList**](PaginatedUserList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_partial_update

> crate::models::User users_users_partial_update(id, format, patched_user_request)


Partial update a user object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this user. | [required] |
**format** | Option<**String**> |  |  |
**patched_user_request** | Option<[**PatchedUserRequest**](PatchedUserRequest.md)> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_retrieve

> crate::models::User users_users_retrieve(id, format, depth, exclude_m2m)


Retrieve a user object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this user. | [required] |
**format** | Option<**String**> |  |  |
**depth** | Option<**i32**> | Serializer Depth |  |[default to 1]
**exclude_m2m** | Option<**bool**> | Exclude many-to-many fields from the response |  |[default to false]

### Return type

[**crate::models::User**](User.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_users_update

> crate::models::User users_users_update(id, user_request, format)


Update a user object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this user. | [required] |
**user_request** | [**UserRequest**](UserRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**crate::models::User**](User.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

