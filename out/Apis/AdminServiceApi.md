# AdminServiceApi

All URIs are relative to *http://localhost:9000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**adminSubmitPost**](AdminServiceApi.md#adminSubmitPost) | **POST** /admin/submit | 
[**wsAdminRegisterTypeGet**](AdminServiceApi.md#wsAdminRegisterTypeGet) | **GET** /ws/admin/register/{type} | 


<a name="adminSubmitPost"></a>
# **adminSubmitPost**
> adminSubmitPost(body, splinterProtocolVersion)



    Send circuit management payload in bytes to admin service

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | **File**|  |
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json

<a name="wsAdminRegisterTypeGet"></a>
# **wsAdminRegisterTypeGet**
> ApplicationRegistration wsAdminRegisterTypeGet(type, splinterProtocolVersion, last)



    Register the handler for a circuit management type

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **type** | **String**| The circuit management type is the type of circuit the handler will manage | [default to null]
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]
 **last** | **Integer**| A timestamp in milliseconds from the Unix Epoch, indicating the last received event. | [optional] [default to 0]

### Return type

[**ApplicationRegistration**](/Models/ApplicationRegistration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

