# CircuitsApi

All URIs are relative to *http://localhost:9000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**adminCircuitsCircuitIdGet**](CircuitsApi.md#adminCircuitsCircuitIdGet) | **GET** /admin/circuits/{circuit_id} | 
[**adminCircuitsGet**](CircuitsApi.md#adminCircuitsGet) | **GET** /admin/circuits | 


<a name="adminCircuitsCircuitIdGet"></a>
# **adminCircuitsCircuitIdGet**
> inline_response_200_3 adminCircuitsCircuitIdGet(circuitId, splinterProtocolVersion)



    Experimental - Fetch a circuit in Splinter State by its circuit id

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **circuitId** | **String**| cirucit id of circuit to fetch | [default to null]
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]

### Return type

[**inline_response_200_3**](/Models/inline_response_200_3.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="adminCircuitsGet"></a>
# **adminCircuitsGet**
> inline_response_200_2 adminCircuitsGet(splinterProtocolVersion, offset, limit, filter)



    Experimental - List of circuits in Splinter state

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]
 **offset** | **Integer**| paging offset | [optional] [default to 0]
 **limit** | **Integer**| maximum number of items to return (max 100) | [optional] [default to 100]
 **filter** | **String**| node id that must be present in the circuits returned | [optional] [default to null]

### Return type

[**inline_response_200_2**](/Models/inline_response_200_2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

