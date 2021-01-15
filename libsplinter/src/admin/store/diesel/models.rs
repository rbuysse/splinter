// Copyright 2018-2021 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Database representations used to implement a diesel backend for the `AdminServiceStore`.
//! These structs differ slightly from their associated native representation to conform to
//! the requirements for storing data with a diesel backend.

use std::convert::TryFrom;

use crate::admin::store::diesel::schema::{
    circuit, circuit_member, circuit_proposal, node_endpoint, proposed_circuit, proposed_node,
    proposed_node_endpoint, proposed_service, proposed_service_argument, service, service_argument,
    vote_record,
};
use crate::admin::store::error::AdminServiceStoreError;
use crate::admin::store::{
    AuthorizationType, DurabilityType, PersistenceType, ProposalType, RouteType, Vote, VoteRecord,
    VoteRecordBuilder,
};
use crate::admin::store::{Circuit, CircuitProposal, ProposedCircuit};
use crate::error::{InternalError, InvalidStateError};

/// Database model representation of a `CircuitProposal`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "circuit_proposal"]
#[primary_key(circuit_id)]
pub struct CircuitProposalModel {
    pub proposal_type: String,
    pub circuit_id: String,
    pub circuit_hash: String,
    pub requester: Vec<u8>,
    pub requester_node_id: String,
}

impl From<&CircuitProposal> for CircuitProposalModel {
    fn from(proposal: &CircuitProposal) -> Self {
        CircuitProposalModel {
            proposal_type: String::from(proposal.proposal_type()),
            circuit_id: proposal.circuit_id().into(),
            circuit_hash: proposal.circuit_hash().into(),
            requester: proposal.requester().to_vec(),
            requester_node_id: proposal.requester_node_id().into(),
        }
    }
}

/// Database model representation of a `ProposedCircuit`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "proposed_circuit"]
#[belongs_to(CircuitProposalModel, foreign_key = "circuit_id")]
#[primary_key(circuit_id)]
pub struct ProposedCircuitModel {
    pub circuit_id: String,
    pub authorization_type: String,
    pub persistence: String,
    pub durability: String,
    pub routes: String,
    pub circuit_management_type: String,
    pub application_metadata: Option<Vec<u8>>,
    pub comments: Option<String>,
    pub display_name: Option<String>,
    pub circuit_version: i32,
}

impl From<&ProposedCircuit> for ProposedCircuitModel {
    fn from(proposed_circuit: &ProposedCircuit) -> Self {
        ProposedCircuitModel {
            circuit_id: proposed_circuit.circuit_id().into(),
            authorization_type: String::from(proposed_circuit.authorization_type()),
            persistence: String::from(proposed_circuit.persistence()),
            durability: String::from(proposed_circuit.durability()),
            routes: String::from(proposed_circuit.routes()),
            circuit_management_type: proposed_circuit.circuit_management_type().into(),
            application_metadata: proposed_circuit.application_metadata().clone(),
            comments: proposed_circuit.comments().clone(),
            display_name: proposed_circuit.display_name().clone(),
            circuit_version: proposed_circuit.circuit_version(),
        }
    }
}

/// Database model representation of a `VoteRecord`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "vote_record"]
#[belongs_to(CircuitProposalModel, foreign_key = "circuit_id")]
#[primary_key(circuit_id, voter_node_id)]
pub struct VoteRecordModel {
    pub circuit_id: String,
    pub public_key: Vec<u8>,
    pub vote: String,
    pub voter_node_id: String,
    pub position: i32,
}

impl TryFrom<&CircuitProposal> for Vec<VoteRecordModel> {
    type Error = AdminServiceStoreError;

    fn try_from(proposal: &CircuitProposal) -> Result<Self, Self::Error> {
        proposal
            .votes()
            .iter()
            .enumerate()
            .map(|(idx, vote)| {
                Ok(VoteRecordModel {
                    circuit_id: proposal.circuit_id().into(),
                    public_key: vote.public_key().into(),
                    vote: String::from(vote.vote()),
                    voter_node_id: vote.voter_node_id().into(),
                    position: i32::try_from(idx).map_err(|_| {
                        AdminServiceStoreError::InternalError(InternalError::with_message(
                            "Unable to convert index into i32".to_string(),
                        ))
                    })?,
                })
            })
            .collect::<Result<Vec<VoteRecordModel>, AdminServiceStoreError>>()
    }
}

impl TryFrom<&VoteRecordModel> for VoteRecord {
    type Error = AdminServiceStoreError;
    fn try_from(vote: &VoteRecordModel) -> Result<Self, Self::Error> {
        VoteRecordBuilder::new()
            .with_public_key(&vote.public_key)
            .with_vote(&Vote::try_from(vote.vote.clone())?)
            .with_voter_node_id(&vote.voter_node_id)
            .build()
            .map_err(AdminServiceStoreError::InvalidStateError)
    }
}

/// Database model representation of a `ProposedNode`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "proposed_node"]
#[belongs_to(ProposedCircuitModel, foreign_key = "circuit_id")]
#[primary_key(circuit_id, node_id)]
pub struct ProposedNodeModel {
    pub circuit_id: String,
    pub node_id: String,
    pub position: i32,
}

impl TryFrom<&ProposedCircuit> for Vec<ProposedNodeModel> {
    type Error = AdminServiceStoreError;

    fn try_from(proposed_circuit: &ProposedCircuit) -> Result<Self, Self::Error> {
        proposed_circuit
            .members()
            .iter()
            .enumerate()
            .map(|(idx, node)| {
                Ok(ProposedNodeModel {
                    circuit_id: proposed_circuit.circuit_id().into(),
                    node_id: node.node_id().into(),
                    position: i32::try_from(idx).map_err(|_| {
                        AdminServiceStoreError::InternalError(InternalError::with_message(
                            "Unable to convert index into i32".to_string(),
                        ))
                    })?,
                })
            })
            .collect::<Result<Vec<ProposedNodeModel>, AdminServiceStoreError>>()
    }
}

/// Database model representation of the endpoint values associated with a `ProposedNode`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "proposed_node_endpoint"]
#[belongs_to(ProposedCircuitModel, foreign_key = "circuit_id")]
#[primary_key(circuit_id, node_id, endpoint)]
pub struct ProposedNodeEndpointModel {
    pub node_id: String,
    pub circuit_id: String,
    pub endpoint: String,
    pub position: i32,
}

impl TryFrom<&ProposedCircuit> for Vec<ProposedNodeEndpointModel> {
    type Error = AdminServiceStoreError;

    fn try_from(proposed_circuit: &ProposedCircuit) -> Result<Self, Self::Error> {
        let mut endpoint_models = Vec::new();
        for node in proposed_circuit.members() {
            endpoint_models.extend(
                node.endpoints()
                    .iter()
                    .enumerate()
                    .map(|(idx, endpoint)| {
                        Ok(ProposedNodeEndpointModel {
                            node_id: node.node_id().into(),
                            circuit_id: proposed_circuit.circuit_id().into(),
                            endpoint: endpoint.clone(),
                            position: i32::try_from(idx).map_err(|_| {
                                AdminServiceStoreError::InternalError(InternalError::with_message(
                                    "Unable to convert index into i32".to_string(),
                                ))
                            })?,
                        })
                    })
                    .collect::<Result<Vec<ProposedNodeEndpointModel>, AdminServiceStoreError>>()?,
            );
        }
        Ok(endpoint_models)
    }
}

/// Database model representation of a `ProposedService`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "proposed_service"]
#[belongs_to(ProposedCircuitModel, foreign_key = "circuit_id")]
#[primary_key(circuit_id, service_id)]
pub struct ProposedServiceModel {
    pub circuit_id: String,
    pub service_id: String,
    pub service_type: String,
    pub node_id: String,
    pub position: i32,
}

impl TryFrom<&ProposedCircuit> for Vec<ProposedServiceModel> {
    type Error = AdminServiceStoreError;

    fn try_from(proposed_circuit: &ProposedCircuit) -> Result<Self, Self::Error> {
        Ok(proposed_circuit
            .roster()
            .iter()
            .enumerate()
            .map(|(idx, service)| {
                Ok(ProposedServiceModel {
                    circuit_id: proposed_circuit.circuit_id().into(),
                    service_id: service.service_id().into(),
                    service_type: service.service_type().into(),
                    node_id: service.node_id().into(),
                    position: i32::try_from(idx).map_err(|_| {
                        AdminServiceStoreError::InternalError(InternalError::with_message(
                            "Unable to convert index into i32".to_string(),
                        ))
                    })?,
                })
            })
            .collect::<Result<Vec<ProposedServiceModel>, AdminServiceStoreError>>()?)
    }
}

/// Database model representation of the arguments associated with a `ProposedService`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "proposed_service_argument"]
#[belongs_to(ProposedCircuitModel, foreign_key = "circuit_id")]
#[primary_key(circuit_id, service_id, key)]
pub struct ProposedServiceArgumentModel {
    pub circuit_id: String,
    pub service_id: String,
    pub key: String,
    pub value: String,
    pub position: i32,
}

impl TryFrom<&ProposedCircuit> for Vec<ProposedServiceArgumentModel> {
    type Error = AdminServiceStoreError;

    fn try_from(proposed_circuit: &ProposedCircuit) -> Result<Self, Self::Error> {
        let mut service_arguments = Vec::new();
        for service in proposed_circuit.roster() {
            service_arguments.extend(
                service
                    .arguments()
                    .iter()
                    .enumerate()
                    .map(|(idx, (key, value))| {
                        Ok(ProposedServiceArgumentModel {
                            circuit_id: proposed_circuit.circuit_id().into(),
                            service_id: service.service_id().into(),
                            key: key.into(),
                            value: value.into(),
                            position: i32::try_from(idx).map_err(|_| {
                                AdminServiceStoreError::InternalError(InternalError::with_message(
                                    "Unable to convert index into i32".to_string(),
                                ))
                            })?,
                        })
                    })
                    .collect::<Result<Vec<ProposedServiceArgumentModel>, AdminServiceStoreError>>(
                    )?,
            );
        }
        Ok(service_arguments)
    }
}

/// Database model representation of `Service`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "service"]
#[belongs_to(CircuitModel, foreign_key = "circuit_id")]
#[primary_key(circuit_id, service_id)]
pub struct ServiceModel {
    pub circuit_id: String,
    pub service_id: String,
    pub service_type: String,
    pub node_id: String,
    pub position: i32,
}

impl TryFrom<&Circuit> for Vec<ServiceModel> {
    type Error = AdminServiceStoreError;

    fn try_from(circuit: &Circuit) -> Result<Self, Self::Error> {
        circuit
            .roster()
            .iter()
            .enumerate()
            .map(|(idx, service)| {
                Ok(ServiceModel {
                    circuit_id: circuit.circuit_id().into(),
                    service_id: service.service_id().into(),
                    service_type: service.service_type().into(),
                    node_id: service.node_id().into(),
                    position: i32::try_from(idx).map_err(|_| {
                        AdminServiceStoreError::InternalError(InternalError::with_message(
                            "Unable to convert index into i32".to_string(),
                        ))
                    })?,
                })
            })
            .collect::<Result<Vec<ServiceModel>, AdminServiceStoreError>>()
    }
}

/// Database model representation of the arguments in a `Service`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "service_argument"]
#[belongs_to(CircuitModel, foreign_key = "circuit_id")]
#[primary_key(circuit_id, service_id, key)]
pub struct ServiceArgumentModel {
    pub circuit_id: String,
    pub service_id: String,
    pub key: String,
    pub value: String,
    pub position: i32,
}

impl TryFrom<&Circuit> for Vec<ServiceArgumentModel> {
    type Error = AdminServiceStoreError;

    fn try_from(circuit: &Circuit) -> Result<Self, Self::Error> {
        let mut service_arguments = Vec::new();
        for service in circuit.roster() {
            service_arguments.extend(
                service
                    .arguments()
                    .iter()
                    .enumerate()
                    .map(|(idx, (key, value))| {
                        Ok(ServiceArgumentModel {
                            circuit_id: circuit.circuit_id().into(),
                            service_id: service.service_id().into(),
                            key: key.clone(),
                            value: value.clone(),
                            position: i32::try_from(idx).map_err(|_| {
                                AdminServiceStoreError::InternalError(InternalError::with_message(
                                    "Unable to convert index into i32".to_string(),
                                ))
                            })?,
                        })
                    })
                    .collect::<Result<Vec<ServiceArgumentModel>, AdminServiceStoreError>>()?,
            );
        }
        Ok(service_arguments)
    }
}

/// Database model representation of a `Circuit`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "circuit"]
#[primary_key(circuit_id)]
pub struct CircuitModel {
    pub circuit_id: String,
    pub authorization_type: String,
    pub persistence: String,
    pub durability: String,
    pub routes: String,
    pub circuit_management_type: String,
    pub display_name: Option<String>,
    pub circuit_version: i32,
}

impl From<&Circuit> for CircuitModel {
    fn from(circuit: &Circuit) -> Self {
        CircuitModel {
            circuit_id: circuit.circuit_id().into(),
            authorization_type: String::from(circuit.authorization_type()),
            persistence: String::from(circuit.persistence()),
            durability: String::from(circuit.durability()),
            routes: String::from(circuit.routes()),
            circuit_management_type: circuit.circuit_management_type().into(),
            display_name: circuit.display_name().clone(),
            circuit_version: circuit.circuit_version(),
        }
    }
}

/// Database model representation of the `members` of a `Circuit`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "circuit_member"]
#[belongs_to(CircuitModel, foreign_key = "circuit_id")]
#[primary_key(circuit_id, node_id)]
pub struct CircuitMemberModel {
    pub circuit_id: String,
    pub node_id: String,
    pub position: i32,
}

impl TryFrom<&Circuit> for Vec<CircuitMemberModel> {
    type Error = AdminServiceStoreError;

    fn try_from(circuit: &Circuit) -> Result<Self, Self::Error> {
        circuit
            .members()
            .iter()
            .enumerate()
            .map(|(idx, node_id)| {
                Ok(CircuitMemberModel {
                    circuit_id: circuit.circuit_id().into(),
                    node_id: node_id.clone(),
                    position: i32::try_from(idx).map_err(|_| {
                        AdminServiceStoreError::InternalError(InternalError::with_message(
                            "Unable to convert index into i32".to_string(),
                        ))
                    })?,
                })
            })
            .collect::<Result<Vec<CircuitMemberModel>, AdminServiceStoreError>>()
    }
}

/// Database model representation of the endpoint values associated with a `Circuit` member `node_id`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "node_endpoint"]
#[primary_key(node_id, endpoint)]
pub struct NodeEndpointModel {
    pub node_id: String,
    pub endpoint: String,
}

// All enums associated with the above structs have TryFrom and From implemented in order to
// translate the enums to a `Text` representation to be stored in the database.

impl TryFrom<String> for Vote {
    type Error = AdminServiceStoreError;
    fn try_from(variant: String) -> Result<Self, Self::Error> {
        match variant.as_ref() {
            "Accept" => Ok(Vote::Accept),
            "Reject" => Ok(Vote::Reject),
            _ => Err(AdminServiceStoreError::InvalidStateError(
                InvalidStateError::with_message("Unable to convert string to Vote".into()),
            )),
        }
    }
}

impl From<&Vote> for String {
    fn from(variant: &Vote) -> Self {
        match variant {
            Vote::Accept => String::from("Accept"),
            Vote::Reject => String::from("Reject"),
        }
    }
}

impl TryFrom<String> for ProposalType {
    type Error = AdminServiceStoreError;
    fn try_from(variant: String) -> Result<Self, Self::Error> {
        match variant.as_ref() {
            "Create" => Ok(ProposalType::Create),
            "UpdateRoster" => Ok(ProposalType::UpdateRoster),
            "AddNode" => Ok(ProposalType::AddNode),
            "RemoveNode" => Ok(ProposalType::RemoveNode),
            "Destroy" => Ok(ProposalType::Destroy),
            _ => Err(AdminServiceStoreError::InvalidStateError(
                InvalidStateError::with_message("Unable to convert string to ProposalType".into()),
            )),
        }
    }
}

impl From<&ProposalType> for String {
    fn from(variant: &ProposalType) -> Self {
        match variant {
            ProposalType::Create => String::from("Create"),
            ProposalType::UpdateRoster => String::from("UpdateRoster"),
            ProposalType::AddNode => String::from("AddNode"),
            ProposalType::RemoveNode => String::from("RemoveNode"),
            ProposalType::Destroy => String::from("Destroy"),
        }
    }
}

impl TryFrom<String> for AuthorizationType {
    type Error = AdminServiceStoreError;
    fn try_from(variant: String) -> Result<Self, Self::Error> {
        match variant.as_ref() {
            "Trust" => Ok(AuthorizationType::Trust),
            _ => Err(AdminServiceStoreError::InvalidStateError(
                InvalidStateError::with_message(
                    "Unable to convert string to AuthorizationType".into(),
                ),
            )),
        }
    }
}

impl From<&AuthorizationType> for String {
    fn from(variant: &AuthorizationType) -> Self {
        match variant {
            AuthorizationType::Trust => String::from("Trust"),
        }
    }
}

impl TryFrom<String> for PersistenceType {
    type Error = AdminServiceStoreError;
    fn try_from(variant: String) -> Result<Self, Self::Error> {
        match variant.as_ref() {
            "Any" => Ok(PersistenceType::Any),
            _ => Err(AdminServiceStoreError::InvalidStateError(
                InvalidStateError::with_message(
                    "Unable to convert string to PersistenceType".into(),
                ),
            )),
        }
    }
}

impl From<&PersistenceType> for String {
    fn from(variant: &PersistenceType) -> Self {
        match variant {
            PersistenceType::Any => String::from("Any"),
        }
    }
}

impl TryFrom<String> for DurabilityType {
    type Error = AdminServiceStoreError;
    fn try_from(variant: String) -> Result<Self, Self::Error> {
        match variant.as_ref() {
            "NoDurability" => Ok(DurabilityType::NoDurability),
            _ => Err(AdminServiceStoreError::InvalidStateError(
                InvalidStateError::with_message(
                    "Unable to convert string to DurabilityType".into(),
                ),
            )),
        }
    }
}

impl From<&DurabilityType> for String {
    fn from(variant: &DurabilityType) -> Self {
        match variant {
            DurabilityType::NoDurability => String::from("NoDurability"),
        }
    }
}

impl TryFrom<String> for RouteType {
    type Error = AdminServiceStoreError;
    fn try_from(variant: String) -> Result<Self, Self::Error> {
        match variant.as_ref() {
            "Any" => Ok(RouteType::Any),
            _ => Err(AdminServiceStoreError::InvalidStateError(
                InvalidStateError::with_message("Unable to convert string to RouteType".into()),
            )),
        }
    }
}

impl From<&RouteType> for String {
    fn from(variant: &RouteType) -> Self {
        match variant {
            RouteType::Any => String::from("Any"),
        }
    }
}
