# Documentation for SplinterD API

<a name="documentation-for-api-endpoints"></a>
## Documentation for API Endpoints

All URIs are relative to *http://localhost:9000/api*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AdminServiceApi* | [**adminSubmitPost**](Apis/AdminServiceApi.md#adminsubmitpost) | **POST** /admin/submit | 
*AdminServiceApi* | [**wsAdminRegisterTypeGet**](Apis/AdminServiceApi.md#wsadminregistertypeget) | **GET** /ws/admin/register/{type} | 
*BiomeApi* | [**biomeLoginPost**](Apis/BiomeApi.md#biomeloginpost) | **POST** /biome/login | 
*BiomeApi* | [**biomeRegisterPost**](Apis/BiomeApi.md#biomeregisterpost) | **POST** /biome/register | 
*BiomeApi* | [**biomeUsersGet**](Apis/BiomeApi.md#biomeusersget) | **GET** /biome/users | 
*BiomeApi* | [**biomeUsersUserIdGet**](Apis/BiomeApi.md#biomeusersuseridget) | **GET** /biome/users/{user_id} | 
*BiomeApi* | [**biomeUsersUserIdKeysGet**](Apis/BiomeApi.md#biomeusersuseridkeysget) | **GET** /biome/users/{user_id}/keys | 
*BiomeApi* | [**biomeUsersUserIdKeysPatch**](Apis/BiomeApi.md#biomeusersuseridkeyspatch) | **PATCH** /biome/users/{user_id}/keys | 
*BiomeApi* | [**biomeUsersUserIdKeysPost**](Apis/BiomeApi.md#biomeusersuseridkeyspost) | **POST** /biome/users/{user_id}/keys | 
*BiomeApi* | [**biomeUsersUserIdPut**](Apis/BiomeApi.md#biomeusersuseridput) | **PUT** /biome/users/{user_id} | 
*CircuitsApi* | [**adminCircuitsCircuitIdGet**](Apis/CircuitsApi.md#admincircuitscircuitidget) | **GET** /admin/circuits/{circuit_id} | 
*CircuitsApi* | [**adminCircuitsGet**](Apis/CircuitsApi.md#admincircuitsget) | **GET** /admin/circuits | 
*DefaultApi* | [**scabbardCircuitServiceIdBatchStatusesPost**](Apis/DefaultApi.md#scabbardcircuitserviceidbatchstatusespost) | **POST** /scabbard/{circuit}/{service_id}/batch_statuses | 
*DefaultApi* | [**scabbardCircuitServiceIdBatchesPost**](Apis/DefaultApi.md#scabbardcircuitserviceidbatchespost) | **POST** /scabbard/{circuit}/{service_id}/batches | 
*DefaultApi* | [**scabbardCircuitServiceIdStateAddressGet**](Apis/DefaultApi.md#scabbardcircuitserviceidstateaddressget) | **GET** /scabbard/{circuit}/{service_id}/state/{address} | 
*DefaultApi* | [**scabbardCircuitServiceIdStateGet**](Apis/DefaultApi.md#scabbardcircuitserviceidstateget) | **GET** /scabbard/{circuit}/{service_id}/state | 
*DiagnosticsApi* | [**statusGet**](Apis/DiagnosticsApi.md#statusget) | **GET** /status | 
*KeyRegistryApi* | [**adminKeysGet**](Apis/KeyRegistryApi.md#adminkeysget) | **GET** /admin/keys | 
*KeyRegistryApi* | [**adminKeysPublicKeyGet**](Apis/KeyRegistryApi.md#adminkeyspublickeyget) | **GET** /admin/keys/{public_key} | 
*NodeRegistryApi* | [**adminNodesGet**](Apis/NodeRegistryApi.md#adminnodesget) | **GET** /admin/nodes | 
*NodeRegistryApi* | [**adminNodesIdentityDelete**](Apis/NodeRegistryApi.md#adminnodesidentitydelete) | **DELETE** /admin/nodes/{identity} | 
*NodeRegistryApi* | [**adminNodesIdentityGet**](Apis/NodeRegistryApi.md#adminnodesidentityget) | **GET** /admin/nodes/{identity} | 
*NodeRegistryApi* | [**adminNodesIdentityPut**](Apis/NodeRegistryApi.md#adminnodesidentityput) | **PUT** /admin/nodes/{identity} | 
*NodeRegistryApi* | [**adminNodesPost**](Apis/NodeRegistryApi.md#adminnodespost) | **POST** /admin/nodes | 
*ProposalsApi* | [**adminProposalsCircuitIdGet**](Apis/ProposalsApi.md#adminproposalscircuitidget) | **GET** /admin/proposals/{circuit_id} | 
*ProposalsApi* | [**adminProposalsGet**](Apis/ProposalsApi.md#adminproposalsget) | **GET** /admin/proposals | 


<a name="documentation-for-models"></a>
## Documentation for Models

 - [/Models.ApplicationRegistration](Models/ApplicationRegistration.md)
 - [/Models.BatchStatus](Models/BatchStatus.md)
 - [/Models.BatchStatusStatus](Models/BatchStatusStatus.md)
 - [/Models.BatchStatusStatusMessage](Models/BatchStatusStatusMessage.md)
 - [/Models.BiomeUserKey](Models/BiomeUserKey.md)
 - [/Models.Circuit](Models/Circuit.md)
 - [/Models.CircuitMember](Models/CircuitMember.md)
 - [/Models.CircuitService](Models/CircuitService.md)
 - [/Models.Error](Models/Error.md)
 - [/Models.ErrorBiome](Models/ErrorBiome.md)
 - [/Models.InlineResponse200](Models/InlineResponse200.md)
 - [/Models.InlineResponse2001](Models/InlineResponse2001.md)
 - [/Models.InlineResponse20010](Models/InlineResponse20010.md)
 - [/Models.InlineResponse20011](Models/InlineResponse20011.md)
 - [/Models.InlineResponse20012](Models/InlineResponse20012.md)
 - [/Models.InlineResponse20013](Models/InlineResponse20013.md)
 - [/Models.InlineResponse20014](Models/InlineResponse20014.md)
 - [/Models.InlineResponse2002](Models/InlineResponse2002.md)
 - [/Models.InlineResponse2003](Models/InlineResponse2003.md)
 - [/Models.InlineResponse2004](Models/InlineResponse2004.md)
 - [/Models.InlineResponse2005](Models/InlineResponse2005.md)
 - [/Models.InlineResponse2006](Models/InlineResponse2006.md)
 - [/Models.InlineResponse2007](Models/InlineResponse2007.md)
 - [/Models.InlineResponse2008](Models/InlineResponse2008.md)
 - [/Models.InlineResponse2009](Models/InlineResponse2009.md)
 - [/Models.Link](Models/Link.md)
 - [/Models.Paging](Models/Paging.md)
 - [/Models.Proposal](Models/Proposal.md)
 - [/Models.ProposalProposalType](Models/ProposalProposalType.md)
 - [/Models.PublicKeyInfo](Models/PublicKeyInfo.md)
 - [/Models.RegisteredNode](Models/RegisteredNode.md)
 - [/Models.Status](Models/Status.md)
 - [/Models.VoteRecord](Models/VoteRecord.md)
 - [/Models.VoteRecordVote](Models/VoteRecordVote.md)


<a name="documentation-for-authorization"></a>
## Documentation for Authorization

All endpoints do not require authorization.
