# \ConfigApi

All URIs are relative to *https://localhost:80/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**config_change**](ConfigApi.md#config_change) | **PUT** /config/{confParam} | Change parameter.
[**config_discover**](ConfigApi.md#config_discover) | **GET** /config | Discover, describe, and gather configuration.
[**config_get**](ConfigApi.md#config_get) | **GET** /config/{confParam} | Parameter value.



## config_change

> config_change(conf_param, conf_param_change)
Change parameter.

Changes the value of the requested parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conf_param** | **String** | Name of the parameter to describe | [required] |
**conf_param_change** | Option<[**ConfParamChange**](ConfParamChange.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_discover

> Vec<models::ConfParam> config_discover()
Discover, describe, and gather configuration.

Returns the list of configurable service parameters with their definitions and current values.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ConfParam>**](ConfParam.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config_get

> models::ConfParam config_get(conf_param)
Parameter value.

Returns the description and current value of the requested parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conf_param** | **String** | Name of the parameter to describe | [required] |

### Return type

[**models::ConfParam**](ConfParam.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

