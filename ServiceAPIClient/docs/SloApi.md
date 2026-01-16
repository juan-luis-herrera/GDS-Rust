# \SloApi

All URIs are relative to *https://localhost:80/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**slo_discover**](SloApi.md#slo_discover) | **GET** /slos | Discover, describe, and gather values of SLOs.
[**slo_get**](SloApi.md#slo_get) | **GET** /slos/{sloId} | Describe and gather value of an SLO.



## slo_discover

> Vec<models::Slo> slo_discover()
Discover, describe, and gather values of SLOs.

Returns the list of declared SLOs of the service with their definitions and current values

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Slo>**](SLO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## slo_get

> models::Slo slo_get(slo_id)
Describe and gather value of an SLO.

Returns the description and current value of the requested SLO.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slo_id** | **String** | ID of SLO to describe | [required] |

### Return type

[**models::Slo**](SLO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

