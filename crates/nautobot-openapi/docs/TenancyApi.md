# \TenancyApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tenancy_tenant_groups_bulk_destroy**](TenancyApi.md#tenancy_tenant_groups_bulk_destroy) | **DELETE** /tenancy/tenant-groups/ | 
[**tenancy_tenant_groups_bulk_partial_update**](TenancyApi.md#tenancy_tenant_groups_bulk_partial_update) | **PATCH** /tenancy/tenant-groups/ | 
[**tenancy_tenant_groups_bulk_update**](TenancyApi.md#tenancy_tenant_groups_bulk_update) | **PUT** /tenancy/tenant-groups/ | 
[**tenancy_tenant_groups_create**](TenancyApi.md#tenancy_tenant_groups_create) | **POST** /tenancy/tenant-groups/ | 
[**tenancy_tenant_groups_destroy**](TenancyApi.md#tenancy_tenant_groups_destroy) | **DELETE** /tenancy/tenant-groups/{id}/ | 
[**tenancy_tenant_groups_list**](TenancyApi.md#tenancy_tenant_groups_list) | **GET** /tenancy/tenant-groups/ | 
[**tenancy_tenant_groups_notes_create**](TenancyApi.md#tenancy_tenant_groups_notes_create) | **POST** /tenancy/tenant-groups/{id}/notes/ | 
[**tenancy_tenant_groups_notes_list**](TenancyApi.md#tenancy_tenant_groups_notes_list) | **GET** /tenancy/tenant-groups/{id}/notes/ | 
[**tenancy_tenant_groups_partial_update**](TenancyApi.md#tenancy_tenant_groups_partial_update) | **PATCH** /tenancy/tenant-groups/{id}/ | 
[**tenancy_tenant_groups_retrieve**](TenancyApi.md#tenancy_tenant_groups_retrieve) | **GET** /tenancy/tenant-groups/{id}/ | 
[**tenancy_tenant_groups_update**](TenancyApi.md#tenancy_tenant_groups_update) | **PUT** /tenancy/tenant-groups/{id}/ | 
[**tenancy_tenants_bulk_destroy**](TenancyApi.md#tenancy_tenants_bulk_destroy) | **DELETE** /tenancy/tenants/ | 
[**tenancy_tenants_bulk_partial_update**](TenancyApi.md#tenancy_tenants_bulk_partial_update) | **PATCH** /tenancy/tenants/ | 
[**tenancy_tenants_bulk_update**](TenancyApi.md#tenancy_tenants_bulk_update) | **PUT** /tenancy/tenants/ | 
[**tenancy_tenants_create**](TenancyApi.md#tenancy_tenants_create) | **POST** /tenancy/tenants/ | 
[**tenancy_tenants_destroy**](TenancyApi.md#tenancy_tenants_destroy) | **DELETE** /tenancy/tenants/{id}/ | 
[**tenancy_tenants_list**](TenancyApi.md#tenancy_tenants_list) | **GET** /tenancy/tenants/ | 
[**tenancy_tenants_notes_create**](TenancyApi.md#tenancy_tenants_notes_create) | **POST** /tenancy/tenants/{id}/notes/ | 
[**tenancy_tenants_notes_list**](TenancyApi.md#tenancy_tenants_notes_list) | **GET** /tenancy/tenants/{id}/notes/ | 
[**tenancy_tenants_partial_update**](TenancyApi.md#tenancy_tenants_partial_update) | **PATCH** /tenancy/tenants/{id}/ | 
[**tenancy_tenants_retrieve**](TenancyApi.md#tenancy_tenants_retrieve) | **GET** /tenancy/tenants/{id}/ | 
[**tenancy_tenants_update**](TenancyApi.md#tenancy_tenants_update) | **PUT** /tenancy/tenants/{id}/ | 



## tenancy_tenant_groups_bulk_destroy

> tenancy_tenant_groups_bulk_destroy(bulk_operation_request, format)


Destroy a list of tenant group objects.

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


## tenancy_tenant_groups_bulk_partial_update

> Vec<crate::models::TenantGroup> tenancy_tenant_groups_bulk_partial_update(patched_bulk_writable_tenant_group_request, format)


Partial update a list of tenant group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patched_bulk_writable_tenant_group_request** | [**Vec<crate::models::PatchedBulkWritableTenantGroupRequest>**](PatchedBulkWritableTenantGroupRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TenantGroup>**](TenantGroup.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenant_groups_bulk_update

> Vec<crate::models::TenantGroup> tenancy_tenant_groups_bulk_update(bulk_writable_tenant_group_request, format)


Update a list of tenant group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_writable_tenant_group_request** | [**Vec<crate::models::BulkWritableTenantGroupRequest>**](BulkWritableTenantGroupRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TenantGroup>**](TenantGroup.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenant_groups_create

> crate::models::TenantGroup tenancy_tenant_groups_create(tenant_group_request, format)


Create one or more tenant group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_group_request** | [**TenantGroupRequest**](TenantGroupRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**crate::models::TenantGroup**](TenantGroup.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenant_groups_destroy

> tenancy_tenant_groups_destroy(id, format)


Destroy a tenant group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this tenant group. | [required] |
**format** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenant_groups_list

> crate::models::PaginatedTenantGroupList tenancy_tenant_groups_list(children, children__isnull, children__n, contacts, contacts__isnull, contacts__n, created, created__gt, created__gte, created__isnull, created__lt, created__lte, created__n, description, description__ic, description__ie, description__iew, description__ire, description__isw, description__n, description__nic, description__nie, description__niew, description__nire, description__nisw, description__nre, description__re, dynamic_groups, dynamic_groups__n, format, has_children, has_tenants, id, id__n, last_updated, last_updated__gt, last_updated__gte, last_updated__isnull, last_updated__lt, last_updated__lte, last_updated__n, limit, name, name__ic, name__ie, name__iew, name__ire, name__isw, name__n, name__nic, name__nie, name__niew, name__nire, name__nisw, name__nre, name__re, offset, parent, parent__isnull, parent__n, q, sort, teams, teams__isnull, teams__n, tenants, tenants__isnull, tenants__n, depth, exclude_m2m)


Retrieve a list of tenant group objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**children** | Option<[**Vec<String>**](String.md)> |  |  |
**children__isnull** | Option<**bool**> | Children (name or ID) is null |  |
**children__n** | Option<[**Vec<String>**](String.md)> |  |  |
**contacts** | Option<[**Vec<String>**](String.md)> |  |  |
**contacts__isnull** | Option<**bool**> | Contacts (name or ID) is null |  |
**contacts__n** | Option<[**Vec<String>**](String.md)> |  |  |
**created** | Option<[**Vec<String>**](String.md)> |  |  |
**created__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**created__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**created__isnull** | Option<**bool**> |  |  |
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
**dynamic_groups** | Option<[**Vec<String>**](String.md)> |  |  |
**dynamic_groups__n** | Option<[**Vec<String>**](String.md)> |  |  |
**format** | Option<**String**> |  |  |
**has_children** | Option<**bool**> | Has children |  |
**has_tenants** | Option<**bool**> | Has tenants |  |
**id** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Unique object identifier, either a UUID primary key or a composite key. |  |
**id__n** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**last_updated** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__isnull** | Option<**bool**> |  |  |
**last_updated__lt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__lte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__n** | Option<[**Vec<String>**](String.md)> |  |  |
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
**parent** | Option<[**Vec<String>**](String.md)> |  |  |
**parent__isnull** | Option<**bool**> | Parent tenant group (name or ID) is null |  |
**parent__n** | Option<[**Vec<String>**](String.md)> |  |  |
**q** | Option<**String**> | Search |  |
**sort** | Option<**String**> | Which field to use when ordering the results. |  |
**teams** | Option<[**Vec<String>**](String.md)> |  |  |
**teams__isnull** | Option<**bool**> | Teams (name or ID) is null |  |
**teams__n** | Option<[**Vec<String>**](String.md)> |  |  |
**tenants** | Option<[**Vec<String>**](String.md)> |  |  |
**tenants__isnull** | Option<**bool**> | Tenants (name or ID) is null |  |
**tenants__n** | Option<[**Vec<String>**](String.md)> |  |  |
**depth** | Option<**i32**> | Serializer Depth |  |[default to 1]
**exclude_m2m** | Option<**bool**> | Exclude many-to-many fields from the response |  |[default to false]

### Return type

[**crate::models::PaginatedTenantGroupList**](PaginatedTenantGroupList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenant_groups_notes_create

> crate::models::Note tenancy_tenant_groups_notes_create(id, note_input_request, format)


API methods for returning or creating notes on an object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this tenant group. | [required] |
**note_input_request** | [**NoteInputRequest**](NoteInputRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**crate::models::Note**](Note.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenant_groups_notes_list

> crate::models::PaginatedNoteList tenancy_tenant_groups_notes_list(id, format, limit, offset, depth, exclude_m2m)


API methods for returning or creating notes on an object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this tenant group. | [required] |
**format** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**depth** | Option<**i32**> | Serializer Depth |  |[default to 1]
**exclude_m2m** | Option<**bool**> | Exclude many-to-many fields from the response |  |[default to false]

### Return type

[**crate::models::PaginatedNoteList**](PaginatedNoteList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenant_groups_partial_update

> crate::models::TenantGroup tenancy_tenant_groups_partial_update(id, format, patched_tenant_group_request)


Partial update a tenant group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this tenant group. | [required] |
**format** | Option<**String**> |  |  |
**patched_tenant_group_request** | Option<[**PatchedTenantGroupRequest**](PatchedTenantGroupRequest.md)> |  |  |

### Return type

[**crate::models::TenantGroup**](TenantGroup.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenant_groups_retrieve

> crate::models::TenantGroup tenancy_tenant_groups_retrieve(id, format, depth, exclude_m2m)


Retrieve a tenant group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this tenant group. | [required] |
**format** | Option<**String**> |  |  |
**depth** | Option<**i32**> | Serializer Depth |  |[default to 1]
**exclude_m2m** | Option<**bool**> | Exclude many-to-many fields from the response |  |[default to false]

### Return type

[**crate::models::TenantGroup**](TenantGroup.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenant_groups_update

> crate::models::TenantGroup tenancy_tenant_groups_update(id, tenant_group_request, format)


Update a tenant group object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this tenant group. | [required] |
**tenant_group_request** | [**TenantGroupRequest**](TenantGroupRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**crate::models::TenantGroup**](TenantGroup.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenants_bulk_destroy

> tenancy_tenants_bulk_destroy(bulk_operation_request, format)


Destroy a list of tenant objects.

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


## tenancy_tenants_bulk_partial_update

> Vec<crate::models::Tenant> tenancy_tenants_bulk_partial_update(patched_bulk_writable_tenant_request, format)


Partial update a list of tenant objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patched_bulk_writable_tenant_request** | [**Vec<crate::models::PatchedBulkWritableTenantRequest>**](PatchedBulkWritableTenantRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Tenant>**](Tenant.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenants_bulk_update

> Vec<crate::models::Tenant> tenancy_tenants_bulk_update(bulk_writable_tenant_request, format)


Update a list of tenant objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_writable_tenant_request** | [**Vec<crate::models::BulkWritableTenantRequest>**](BulkWritableTenantRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::Tenant>**](Tenant.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenants_create

> crate::models::Tenant tenancy_tenants_create(tenant_request, format)


Create one or more tenant objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_request** | [**TenantRequest**](TenantRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenants_destroy

> tenancy_tenants_destroy(id, format)


Destroy a tenant object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this tenant. | [required] |
**format** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenants_list

> crate::models::PaginatedTenantList tenancy_tenants_list(circuits, circuits__isnull, circuits__n, clusters, clusters__isnull, clusters__n, comments, comments__ic, comments__ie, comments__iew, comments__ire, comments__isw, comments__n, comments__nic, comments__nie, comments__niew, comments__nire, comments__nisw, comments__nre, comments__re, contacts, contacts__isnull, contacts__n, created, created__gt, created__gte, created__isnull, created__lt, created__lte, created__n, description, description__ic, description__ie, description__iew, description__ire, description__isw, description__n, description__nic, description__nie, description__niew, description__nire, description__nisw, description__nre, description__re, devices, devices__isnull, devices__n, dynamic_groups, dynamic_groups__n, format, has_circuits, has_clusters, has_devices, has_ip_addresses, has_locations, has_prefixes, has_rack_reservations, has_racks, has_route_targets, has_virtual_machines, has_vlans, has_vrfs, id, id__n, ip_addresses, ip_addresses__isnull, ip_addresses__n, last_updated, last_updated__gt, last_updated__gte, last_updated__isnull, last_updated__lt, last_updated__lte, last_updated__n, limit, locations, locations__isnull, locations__n, name, name__ic, name__ie, name__iew, name__ire, name__isw, name__n, name__nic, name__nie, name__niew, name__nire, name__nisw, name__nre, name__re, offset, prefixes, prefixes__isnull, prefixes__n, q, rack_reservations, rack_reservations__isnull, rack_reservations__n, racks, racks__isnull, racks__n, route_targets, route_targets__isnull, route_targets__n, sort, tags, tags__isnull, tags__n, teams, teams__isnull, teams__n, tenant_group, tenant_group__isnull, tenant_group__n, virtual_machines, virtual_machines__isnull, virtual_machines__n, vlans, vlans__isnull, vlans__n, vrfs, vrfs__isnull, vrfs__n, depth, exclude_m2m)


Retrieve a list of tenant objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**circuits** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Circuits (ID) |  |
**circuits__isnull** | Option<**bool**> | Circuits (ID) is null |  |
**circuits__n** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Exclude Circuits (ID) |  |
**clusters** | Option<[**Vec<String>**](String.md)> |  |  |
**clusters__isnull** | Option<**bool**> | Clusters (name or ID) is null |  |
**clusters__n** | Option<[**Vec<String>**](String.md)> |  |  |
**comments** | Option<[**Vec<String>**](String.md)> |  |  |
**comments__ic** | Option<[**Vec<String>**](String.md)> |  |  |
**comments__ie** | Option<[**Vec<String>**](String.md)> |  |  |
**comments__iew** | Option<[**Vec<String>**](String.md)> |  |  |
**comments__ire** | Option<[**Vec<String>**](String.md)> |  |  |
**comments__isw** | Option<[**Vec<String>**](String.md)> |  |  |
**comments__n** | Option<[**Vec<String>**](String.md)> |  |  |
**comments__nic** | Option<[**Vec<String>**](String.md)> |  |  |
**comments__nie** | Option<[**Vec<String>**](String.md)> |  |  |
**comments__niew** | Option<[**Vec<String>**](String.md)> |  |  |
**comments__nire** | Option<[**Vec<String>**](String.md)> |  |  |
**comments__nisw** | Option<[**Vec<String>**](String.md)> |  |  |
**comments__nre** | Option<[**Vec<String>**](String.md)> |  |  |
**comments__re** | Option<[**Vec<String>**](String.md)> |  |  |
**contacts** | Option<[**Vec<String>**](String.md)> |  |  |
**contacts__isnull** | Option<**bool**> | Contacts (name or ID) is null |  |
**contacts__n** | Option<[**Vec<String>**](String.md)> |  |  |
**created** | Option<[**Vec<String>**](String.md)> |  |  |
**created__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**created__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**created__isnull** | Option<**bool**> |  |  |
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
**devices** | Option<[**Vec<String>**](String.md)> |  |  |
**devices__isnull** | Option<**bool**> | Devices (name or ID) is null |  |
**devices__n** | Option<[**Vec<String>**](String.md)> |  |  |
**dynamic_groups** | Option<[**Vec<String>**](String.md)> |  |  |
**dynamic_groups__n** | Option<[**Vec<String>**](String.md)> |  |  |
**format** | Option<**String**> |  |  |
**has_circuits** | Option<**bool**> | Has circuits |  |
**has_clusters** | Option<**bool**> | Has clusters |  |
**has_devices** | Option<**bool**> | Has devices |  |
**has_ip_addresses** | Option<**bool**> | Has IP addresses |  |
**has_locations** | Option<**bool**> | Has locations |  |
**has_prefixes** | Option<**bool**> | Has prefixes |  |
**has_rack_reservations** | Option<**bool**> | Has rack reservations |  |
**has_racks** | Option<**bool**> | Has racks |  |
**has_route_targets** | Option<**bool**> | Has route targets |  |
**has_virtual_machines** | Option<**bool**> | Has virtual machines |  |
**has_vlans** | Option<**bool**> | Has VLANs |  |
**has_vrfs** | Option<**bool**> | Has VRFs |  |
**id** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Unique object identifier, either a UUID primary key or a composite key. |  |
**id__n** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  |  |
**ip_addresses** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | IP addresses (ID) |  |
**ip_addresses__isnull** | Option<**bool**> | IP addresses (ID) is null |  |
**ip_addresses__n** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Exclude IP addresses (ID) |  |
**last_updated** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__gt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__gte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__isnull** | Option<**bool**> |  |  |
**last_updated__lt** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__lte** | Option<[**Vec<String>**](String.md)> |  |  |
**last_updated__n** | Option<[**Vec<String>**](String.md)> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**locations** | Option<[**Vec<String>**](String.md)> |  |  |
**locations__isnull** | Option<**bool**> | Locations (names and/or IDs) is null |  |
**locations__n** | Option<[**Vec<String>**](String.md)> |  |  |
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
**prefixes** | Option<[**Vec<String>**](String.md)> |  |  |
**prefixes__isnull** | Option<**bool**> | Prefix (ID or prefix string) is null |  |
**prefixes__n** | Option<[**Vec<String>**](String.md)> |  |  |
**q** | Option<**String**> | Search |  |
**rack_reservations** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Rack reservations (ID) |  |
**rack_reservations__isnull** | Option<**bool**> | Rack reservations (ID) is null |  |
**rack_reservations__n** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Exclude Rack reservations (ID) |  |
**racks** | Option<[**Vec<String>**](String.md)> |  |  |
**racks__isnull** | Option<**bool**> | Racks (name or ID) is null |  |
**racks__n** | Option<[**Vec<String>**](String.md)> |  |  |
**route_targets** | Option<[**Vec<String>**](String.md)> |  |  |
**route_targets__isnull** | Option<**bool**> | Route targets (name or ID) is null |  |
**route_targets__n** | Option<[**Vec<String>**](String.md)> |  |  |
**sort** | Option<**String**> | Which field to use when ordering the results. |  |
**tags** | Option<[**Vec<String>**](String.md)> |  |  |
**tags__isnull** | Option<**bool**> |  |  |
**tags__n** | Option<[**Vec<String>**](String.md)> |  |  |
**teams** | Option<[**Vec<String>**](String.md)> |  |  |
**teams__isnull** | Option<**bool**> | Teams (name or ID) is null |  |
**teams__n** | Option<[**Vec<String>**](String.md)> |  |  |
**tenant_group** | Option<[**Vec<String>**](String.md)> |  |  |
**tenant_group__isnull** | Option<**bool**> | Tenant group (name or ID) is null |  |
**tenant_group__n** | Option<[**Vec<String>**](String.md)> |  |  |
**virtual_machines** | Option<[**Vec<String>**](String.md)> |  |  |
**virtual_machines__isnull** | Option<**bool**> | Virtual machines (name or ID) is null |  |
**virtual_machines__n** | Option<[**Vec<String>**](String.md)> |  |  |
**vlans** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | VLANs (ID) |  |
**vlans__isnull** | Option<**bool**> | VLANs (ID) is null |  |
**vlans__n** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Exclude VLANs (ID) |  |
**vrfs** | Option<[**Vec<String>**](String.md)> |  |  |
**vrfs__isnull** | Option<**bool**> | VRFs (name or ID) is null |  |
**vrfs__n** | Option<[**Vec<String>**](String.md)> |  |  |
**depth** | Option<**i32**> | Serializer Depth |  |[default to 1]
**exclude_m2m** | Option<**bool**> | Exclude many-to-many fields from the response |  |[default to false]

### Return type

[**crate::models::PaginatedTenantList**](PaginatedTenantList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenants_notes_create

> crate::models::Note tenancy_tenants_notes_create(id, note_input_request, format)


API methods for returning or creating notes on an object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this tenant. | [required] |
**note_input_request** | [**NoteInputRequest**](NoteInputRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**crate::models::Note**](Note.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenants_notes_list

> crate::models::PaginatedNoteList tenancy_tenants_notes_list(id, format, limit, offset, depth, exclude_m2m)


API methods for returning or creating notes on an object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this tenant. | [required] |
**format** | Option<**String**> |  |  |
**limit** | Option<**i32**> | Number of results to return per page. |  |
**offset** | Option<**i32**> | The initial index from which to return the results. |  |
**depth** | Option<**i32**> | Serializer Depth |  |[default to 1]
**exclude_m2m** | Option<**bool**> | Exclude many-to-many fields from the response |  |[default to false]

### Return type

[**crate::models::PaginatedNoteList**](PaginatedNoteList.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenants_partial_update

> crate::models::Tenant tenancy_tenants_partial_update(id, format, patched_tenant_request)


Partial update a tenant object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this tenant. | [required] |
**format** | Option<**String**> |  |  |
**patched_tenant_request** | Option<[**PatchedTenantRequest**](PatchedTenantRequest.md)> |  |  |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenants_retrieve

> crate::models::Tenant tenancy_tenants_retrieve(id, format, depth, exclude_m2m)


Retrieve a tenant object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this tenant. | [required] |
**format** | Option<**String**> |  |  |
**depth** | Option<**i32**> | Serializer Depth |  |[default to 1]
**exclude_m2m** | Option<**bool**> | Exclude many-to-many fields from the response |  |[default to false]

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenancy_tenants_update

> crate::models::Tenant tenancy_tenants_update(id, tenant_request, format)


Update a tenant object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID string identifying this tenant. | [required] |
**tenant_request** | [**TenantRequest**](TenantRequest.md) |  | [required] |
**format** | Option<**String**> |  |  |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[cookieAuth](../README.md#cookieAuth), [tokenAuth](../README.md#tokenAuth)

### HTTP request headers

- **Content-Type**: application/json, text/csv
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

