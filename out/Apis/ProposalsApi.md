# ProposalsApi

All URIs are relative to *http://localhost:9000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**adminProposalsCircuitIdGet**](ProposalsApi.md#adminProposalsCircuitIdGet) | **GET** /admin/proposals/{circuit_id} | 
[**adminProposalsGet**](ProposalsApi.md#adminProposalsGet) | **GET** /admin/proposals | 


<a name="adminProposalsCircuitIdGet"></a>
# **adminProposalsCircuitIdGet**
> inline_response_200_1 adminProposalsCircuitIdGet(circuitId, splinterProtocolVersion)



    Experimental - Fetch a proposal in Admin Service state by its circuit id

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **circuitId** | **String**| circuit id of the proposal to fetch | [default to null]
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]

### Return type

[**inline_response_200_1**](/Models/inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="adminProposalsGet"></a>
# **adminProposalsGet**
> inline_response_200 adminProposalsGet(splinterProtocolVersion, offset, limit, filter)



    Experimental - List of proposals in Admin Service state

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]
 **offset** | **Integer**| paging offset | [optional] [default to 0]
 **limit** | **Integer**| maximum number of items to return (max 100) | [optional] [default to 100]
 **filter** | **String**| circuit management type of the proposed circuit | [optional] [default to null]

### Return type

[**inline_response_200**](/Models/inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

