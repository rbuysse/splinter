# BiomeApi

All URIs are relative to *http://localhost:9000/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**biomeLoginPost**](BiomeApi.md#biomeLoginPost) | **POST** /biome/login | 
[**biomeRegisterPost**](BiomeApi.md#biomeRegisterPost) | **POST** /biome/register | 
[**biomeUsersGet**](BiomeApi.md#biomeUsersGet) | **GET** /biome/users | 
[**biomeUsersUserIdGet**](BiomeApi.md#biomeUsersUserIdGet) | **GET** /biome/users/{user_id} | 
[**biomeUsersUserIdKeysGet**](BiomeApi.md#biomeUsersUserIdKeysGet) | **GET** /biome/users/{user_id}/keys | 
[**biomeUsersUserIdKeysPatch**](BiomeApi.md#biomeUsersUserIdKeysPatch) | **PATCH** /biome/users/{user_id}/keys | 
[**biomeUsersUserIdKeysPost**](BiomeApi.md#biomeUsersUserIdKeysPost) | **POST** /biome/users/{user_id}/keys | 
[**biomeUsersUserIdPut**](BiomeApi.md#biomeUsersUserIdPut) | **PUT** /biome/users/{user_id} | 


<a name="biomeLoginPost"></a>
# **biomeLoginPost**
> inline_response_200_10 biomeLoginPost(splinterProtocolVersion, uNKNOWNBASETYPE)



    Authenticates a user with username and password credentials

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]
 **uNKNOWNBASETYPE** | [**UNKNOWN_BASE_TYPE**](/Models/UNKNOWN_BASE_TYPE.md)|  | [optional]

### Return type

[**inline_response_200_10**](/Models/inline_response_200_10.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="biomeRegisterPost"></a>
# **biomeRegisterPost**
> inline_response_200_9 biomeRegisterPost(splinterProtocolVersion, uNKNOWNBASETYPE)



    Create new user with username and password credentials

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]
 **uNKNOWNBASETYPE** | [**UNKNOWN_BASE_TYPE**](/Models/UNKNOWN_BASE_TYPE.md)|  | [optional]

### Return type

[**inline_response_200_9**](/Models/inline_response_200_9.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="biomeUsersGet"></a>
# **biomeUsersGet**
> List biomeUsersGet(splinterProtocolVersion)



    Lists all users

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]

### Return type

[**List**](/Models/inline_response_200_11.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="biomeUsersUserIdGet"></a>
# **biomeUsersUserIdGet**
> inline_response_200_11 biomeUsersUserIdGet(userId, splinterProtocolVersion)



    Fetch a user by ID

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **userId** | **String**| ID of the user | [default to null]
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]

### Return type

[**inline_response_200_11**](/Models/inline_response_200_11.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="biomeUsersUserIdKeysGet"></a>
# **biomeUsersUserIdKeysGet**
> List biomeUsersUserIdKeysGet(userId, splinterProtocolVersion)



    List keys of a user

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **userId** | **String**| ID of the user | [default to null]
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]

### Return type

[**List**](/Models/BiomeUserKey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="biomeUsersUserIdKeysPatch"></a>
# **biomeUsersUserIdKeysPatch**
> inline_response_200_14 biomeUsersUserIdKeysPatch(userId, splinterProtocolVersion, uNKNOWNBASETYPE)



    Update a key&#39;s display name

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **userId** | **String**| ID of the user | [default to null]
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]
 **uNKNOWNBASETYPE** | [**UNKNOWN_BASE_TYPE**](/Models/UNKNOWN_BASE_TYPE.md)|  | [optional]

### Return type

[**inline_response_200_14**](/Models/inline_response_200_14.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="biomeUsersUserIdKeysPost"></a>
# **biomeUsersUserIdKeysPost**
> inline_response_200_13 biomeUsersUserIdKeysPost(userId, splinterProtocolVersion, biomeUserKey)



    Add a new key for user

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **userId** | **String**| ID of the user | [default to null]
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]
 **biomeUserKey** | [**BiomeUserKey**](/Models/BiomeUserKey.md)|  | [optional]

### Return type

[**inline_response_200_13**](/Models/inline_response_200_13.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="biomeUsersUserIdPut"></a>
# **biomeUsersUserIdPut**
> inline_response_200_12 biomeUsersUserIdPut(userId, splinterProtocolVersion, uNKNOWNBASETYPE)



    Update a user

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **userId** | **String**| ID of the user | [default to null]
 **splinterProtocolVersion** | **Integer**| The protocol version which the client can understand | [optional] [default to null]
 **uNKNOWNBASETYPE** | [**UNKNOWN_BASE_TYPE**](/Models/UNKNOWN_BASE_TYPE.md)|  | [optional]

### Return type

[**inline_response_200_12**](/Models/inline_response_200_12.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

