# DiagnosticsApi

All URIs are relative to *http://localhost:9000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**statusGet**](DiagnosticsApi.md#statusGet) | **GET** /status | 


<a name="statusGet"></a>
# **statusGet**
> Status statusGet(splinterProtocolVersion)



    Used to check if server is successfully running

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]

### Return type

[**Status**](/Models/Status.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

