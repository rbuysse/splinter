# DefaultApi

All URIs are relative to *http://localhost:9000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**scabbardCircuitServiceIdBatchStatusesPost**](DefaultApi.md#scabbardCircuitServiceIdBatchStatusesPost) | **POST** /scabbard/{circuit}/{service_id}/batch_statuses | 
[**scabbardCircuitServiceIdBatchesPost**](DefaultApi.md#scabbardCircuitServiceIdBatchesPost) | **POST** /scabbard/{circuit}/{service_id}/batches | 
[**scabbardCircuitServiceIdStateAddressGet**](DefaultApi.md#scabbardCircuitServiceIdStateAddressGet) | **GET** /scabbard/{circuit}/{service_id}/state/{address} | 
[**scabbardCircuitServiceIdStateGet**](DefaultApi.md#scabbardCircuitServiceIdStateGet) | **GET** /scabbard/{circuit}/{service_id}/state | 


<a name="scabbardCircuitServiceIdBatchStatusesPost"></a>
# **scabbardCircuitServiceIdBatchStatusesPost**
> List scabbardCircuitServiceIdBatchStatusesPost(circuit, serviceId, ids, splinterProtocolVersion, wait)



    Send a list of Sabre batches to the specified Scabbard service

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **circuit** | **String**| circuit the targeted service belongs to | [default to null]
 **serviceId** | **String**| ID of the targeted service | [default to null]
 **ids** | **String**| Comma separated list of batch ids | [default to null]
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]
 **wait** | **Integer**| time (in seconds) to wait for batches to be committed | [optional] [default to 300]

### Return type

[**List**](/Models/BatchStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="scabbardCircuitServiceIdBatchesPost"></a>
# **scabbardCircuitServiceIdBatchesPost**
> Link scabbardCircuitServiceIdBatchesPost(circuit, serviceId, splinterProtocolVersion)



    Send a list of Sabre batches to the specified Scabbard service

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **circuit** | **String**| circuit the targeted service belongs to | [default to null]
 **serviceId** | **String**| ID of the targeted service | [default to null]
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]

### Return type

[**Link**](/Models/Link.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="scabbardCircuitServiceIdStateAddressGet"></a>
# **scabbardCircuitServiceIdStateAddressGet**
> List scabbardCircuitServiceIdStateAddressGet(circuit, serviceId, address, splinterProtocolVersion)



    Experimental - Get the value at a given address in state from the specified Scabbard service

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **circuit** | **String**| circuit the targeted service belongs to | [default to null]
 **serviceId** | **String**| ID of the targeted service | [default to null]
 **address** | **String**| The address of the value to retrieve from state | [default to null]
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]

### Return type

[**List**](/Models/integer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="scabbardCircuitServiceIdStateGet"></a>
# **scabbardCircuitServiceIdStateGet**
> List scabbardCircuitServiceIdStateGet(circuit, serviceId, splinterProtocolVersion, prefix)



    Experimental - Get a list of entries in state from the specified Scabbard service

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **circuit** | **String**| circuit the targeted service belongs to | [default to null]
 **serviceId** | **String**| ID of the targeted service | [default to null]
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]
 **prefix** | **String**| The address prefix for filtering state entries | [optional] [default to null]

### Return type

[**List**](/Models/inline_response_200_8.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

