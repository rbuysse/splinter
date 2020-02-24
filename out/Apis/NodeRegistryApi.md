# NodeRegistryApi

All URIs are relative to *http://localhost:9000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**adminNodesGet**](NodeRegistryApi.md#adminNodesGet) | **GET** /admin/nodes | 
[**adminNodesIdentityDelete**](NodeRegistryApi.md#adminNodesIdentityDelete) | **DELETE** /admin/nodes/{identity} | 
[**adminNodesIdentityGet**](NodeRegistryApi.md#adminNodesIdentityGet) | **GET** /admin/nodes/{identity} | 
[**adminNodesIdentityPut**](NodeRegistryApi.md#adminNodesIdentityPut) | **PUT** /admin/nodes/{identity} | 
[**adminNodesPost**](NodeRegistryApi.md#adminNodesPost) | **POST** /admin/nodes | 


<a name="adminNodesGet"></a>
# **adminNodesGet**
> inline_response_200_6 adminNodesGet(splinterProtocolVersion, offset, limit, filter)



    List nodes in the Node Registry

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]
 **offset** | **Integer**| paging offset | [optional] [default to 0]
 **limit** | **Integer**| maximum number of items to return (max 100) | [optional] [default to 100]
 **filter** | **String**| url-encodeded stringified JSON containing property filters on the node&#39;s metadata properties in the format   {METADATA_PROPERTY:[{\&quot;operator\&quot;:OPERATOR,\&quot;value\&quot;:VALUE}]}  | [optional] [default to null]

### Return type

[**inline_response_200_6**](/Models/inline_response_200_6.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="adminNodesIdentityDelete"></a>
# **adminNodesIdentityDelete**
> adminNodesIdentityDelete(identity, splinterProtocolVersion)



    Delete a node from the Node Registry

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **identity** | **String**| identity of node to delete | [default to null]
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="adminNodesIdentityGet"></a>
# **adminNodesIdentityGet**
> inline_response_200_7 adminNodesIdentityGet(identity, splinterProtocolVersion)



    Fetch a nodes in the Node Registry by their identity

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **identity** | **String**| identity of node to fetch | [default to null]
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]

### Return type

[**inline_response_200_7**](/Models/inline_response_200_7.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="adminNodesIdentityPut"></a>
# **adminNodesIdentityPut**
> adminNodesIdentityPut(identity, registeredNode, splinterProtocolVersion)



    Add a node to or replace a node in the Node Registry. This action is idempotent.

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **identity** | **String**| identity of node to add/replace | [default to null]
 **registeredNode** | [**RegisteredNode**](/Models/RegisteredNode.md)|  |
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="adminNodesPost"></a>
# **adminNodesPost**
> adminNodesPost(registeredNode, splinterProtocolVersion)



    Add node to the Node Registry

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **registeredNode** | [**RegisteredNode**](/Models/RegisteredNode.md)|  |
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

