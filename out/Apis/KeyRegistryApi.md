# KeyRegistryApi

All URIs are relative to *http://localhost:9000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**adminKeysGet**](KeyRegistryApi.md#adminKeysGet) | **GET** /admin/keys | 
[**adminKeysPublicKeyGet**](KeyRegistryApi.md#adminKeysPublicKeyGet) | **GET** /admin/keys/{public_key} | 


<a name="adminKeysGet"></a>
# **adminKeysGet**
> inline_response_200_4 adminKeysGet(splinterProtocolVersion, offset, limit)



    List public key information in the Key Registry

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]
 **offset** | **Integer**| paging offset | [optional] [default to 0]
 **limit** | **Integer**| maximum number of items to return (max 100) | [optional] [default to 100]

### Return type

[**inline_response_200_4**](/Models/inline_response_200_4.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="adminKeysPublicKeyGet"></a>
# **adminKeysPublicKeyGet**
> inline_response_200_5 adminKeysPublicKeyGet(publicKey, splinterProtocolVersion)



    Fetch public key information in the Key Registry by public key

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **publicKey** | **String**| public key to query, in hex | [default to null]
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]

### Return type

[**inline_response_200_5**](/Models/inline_response_200_5.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

