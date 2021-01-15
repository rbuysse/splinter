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

//! Database representations used to implement a diesel backend for the `AdminServiceEventStore`.

use std::convert::TryFrom;

use crate::admin::service::event::{
    store::{
        diesel::schema::{
            admin_event_circuit_proposal, admin_event_proposed_circuit, admin_event_proposed_node,
            admin_event_proposed_node_endpoint, admin_event_proposed_service,
            admin_event_proposed_service_argument, admin_event_vote_record, admin_service_event,
        },
        AdminServiceEventStoreError,
    },
    AdminServiceEvent, EventType,
};
use crate::admin::service::messages::{self, CreateCircuit};
use crate::admin::store::{CircuitProposal, Vote, VoteRecord, VoteRecordBuilder};
use crate::error::{InternalError, InvalidStateError};

/// Database model representation of an `AdminServiceEvent`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "admin_service_event"]
#[primary_key(id)]
pub struct AdminServiceEventModel {
    pub id: i64,
    pub event_type: String,
    pub data: Option<Vec<u8>>,
}

#[derive(AsChangeset, Insertable, PartialEq, Debug)]
#[table_name = "admin_service_event"]
pub struct NewAdminServiceEventModel<'a> {
    pub event_type: &'a str,
    pub data: Option<&'a [u8]>,
}

/// Database model representation of a `CircuitProposal` from an `AdminServiceEvent`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "admin_event_circuit_proposal"]
#[belongs_to(AdminServiceEventModel, foreign_key = "event_id")]
#[primary_key(event_id)]
pub struct AdminEventCircuitProposalModel {
    pub event_id: i64,
    pub proposal_type: String,
    pub circuit_id: String,
    pub circuit_hash: String,
    pub requester: Vec<u8>,
    pub requester_node_id: String,
}

impl From<(i64, &messages::CircuitProposal)> for AdminEventCircuitProposalModel {
    fn from((event_id, proposal): (i64, &messages::CircuitProposal)) -> Self {
        AdminEventCircuitProposalModel {
            event_id,
            proposal_type: String::from(&proposal.proposal_type),
            circuit_id: proposal.circuit_id.to_string(),
            circuit_hash: proposal.circuit_hash.to_string(),
            requester: proposal.requester.to_vec(),
            requester_node_id: proposal.requester_node_id.to_string(),
        }
    }
}

/// Database model representation of a `CreateCircuit` from an `AdminServiceEvent`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "admin_event_proposed_circuit"]
#[belongs_to(AdminServiceEventModel, foreign_key = "event_id")]
#[primary_key(event_id)]
pub struct AdminEventProposedCircuitModel {
    pub event_id: i64,
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

impl From<(i64, &CreateCircuit)> for AdminEventProposedCircuitModel {
    fn from((event_id, create_circuit): (i64, &CreateCircuit)) -> Self {
        let application_metadata = if create_circuit.application_metadata.is_empty() {
            None
        } else {
            Some(create_circuit.application_metadata.to_vec())
        };

        AdminEventProposedCircuitModel {
            event_id,
            circuit_id: create_circuit.circuit_id.to_string(),
            authorization_type: String::from(&create_circuit.authorization_type),
            persistence: String::from(&create_circuit.persistence),
            durability: String::from(&create_circuit.durability),
            routes: String::from(&create_circuit.routes),
            circuit_management_type: create_circuit.circuit_management_type.to_string(),
            application_metadata,
            comments: create_circuit.comments.clone(),
            display_name: create_circuit.display_name.clone(),
            circuit_version: create_circuit.circuit_version,
        }
    }
}

/// Database model representation of a `VoteRecord` from an `AdminServiceEvent`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "admin_event_vote_record"]
#[belongs_to(AdminServiceEventModel, foreign_key = "event_id")]
#[primary_key(event_id, voter_node_id)]
pub struct AdminEventVoteRecordModel {
    pub event_id: i64,
    pub public_key: Vec<u8>,
    pub vote: String,
    pub voter_node_id: String,
    pub position: i32,
}

impl AdminEventVoteRecordModel {
    // Creates a list of `AdminEventVoteRecordModel` from a `CircuitProposal` associated with
    // an `AdminServiceEvent`
    pub(super) fn list_from_proposal_with_id(
        event_id: i64,
        proposal: &messages::CircuitProposal,
    ) -> Result<Vec<AdminEventVoteRecordModel>, AdminServiceEventStoreError> {
        proposal
            .votes
            .iter()
            .enumerate()
            .map(|(idx, vote)| {
                Ok(AdminEventVoteRecordModel {
                    event_id,
                    public_key: vote.public_key.to_vec(),
                    vote: String::from(&vote.vote),
                    voter_node_id: vote.voter_node_id.to_string(),
                    position: i32::try_from(idx).map_err(|_| {
                        AdminServiceEventStoreError::InternalError(InternalError::with_message(
                            "Unable to convert index into i32".to_string(),
                        ))
                    })?,
                })
            })
            .collect()
    }
}

impl TryFrom<&AdminEventVoteRecordModel> for VoteRecord {
    type Error = InvalidStateError;
    fn try_from(
        admin_event_vote_record_model: &AdminEventVoteRecordModel,
    ) -> Result<Self, Self::Error> {
        VoteRecordBuilder::new()
            .with_public_key(&admin_event_vote_record_model.public_key)
            .with_vote(
                &Vote::try_from(admin_event_vote_record_model.vote.clone()).map_err(|_| {
                    InvalidStateError::with_message("Unable to convert string to Vote".into())
                })?,
            )
            .with_voter_node_id(&admin_event_vote_record_model.voter_node_id)
            .build()
    }
}

/// Database model representation of a `AdminEventProposedNode` from an `AdminServiceEvent`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "admin_event_proposed_node"]
#[belongs_to(AdminServiceEventModel, foreign_key = "event_id")]
#[primary_key(event_id, node_id)]
pub struct AdminEventProposedNodeModel {
    pub event_id: i64,
    pub node_id: String,
    pub position: i32,
}

impl AdminEventProposedNodeModel {
    // Creates a list of `AdminEventProposedNodeModel` from a `CircuitProposal` associated with
    // an `AdminServiceEvent`
    pub(super) fn list_from_proposal_with_id(
        event_id: i64,
        proposal: &messages::CircuitProposal,
    ) -> Result<Vec<AdminEventProposedNodeModel>, AdminServiceEventStoreError> {
        proposal
            .circuit
            .members
            .iter()
            .enumerate()
            .map(|(idx, node)| {
                Ok(AdminEventProposedNodeModel {
                    event_id,
                    node_id: node.node_id.to_string(),
                    position: i32::try_from(idx).map_err(|_| {
                        AdminServiceEventStoreError::InternalError(InternalError::with_message(
                            "Unable to convert index into i32".to_string(),
                        ))
                    })?,
                })
            })
            .collect()
    }
}

/// Database model representation of the endpoint values associated with a `ProposedNode` from an
/// `AdminServiceEvent`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "admin_event_proposed_node_endpoint"]
#[belongs_to(AdminServiceEventModel, foreign_key = "event_id")]
#[primary_key(event_id, node_id, endpoint)]
pub struct AdminEventProposedNodeEndpointModel {
    pub event_id: i64,
    pub node_id: String,
    pub endpoint: String,
    pub position: i32,
}

impl AdminEventProposedNodeEndpointModel {
    // Creates a list of `AdminEventProposedNodeEndpointModel` from a `CircuitProposal` associated
    // with an `AdminServiceEvent`
    pub(super) fn list_from_proposal_with_id(
        event_id: i64,
        proposal: &messages::CircuitProposal,
    ) -> Result<Vec<AdminEventProposedNodeEndpointModel>, AdminServiceEventStoreError> {
        let mut endpoint_models = Vec::new();
        for node in &proposal.circuit.members {
            endpoint_models.extend(
                node.endpoints
                    .iter()
                    .enumerate()
                    .map(|(idx, endpoint)| Ok(AdminEventProposedNodeEndpointModel {
                        event_id,
                        node_id: node.node_id.to_string(),
                        endpoint: endpoint.to_string(),
                        position: i32::try_from(idx).map_err(|_| {
                            AdminServiceEventStoreError::InternalError(InternalError::with_message(
                                "Unable to convert index into i32".to_string(),
                            ))
                        })?,
                    }))
                    .collect::<Result<
                        Vec<AdminEventProposedNodeEndpointModel>,AdminServiceEventStoreError>
                    >()?,
            );
        }
        Ok(endpoint_models)
    }
}

/// Database model representation of a `ProposedService` from an `AdminServiceEvent`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "admin_event_proposed_service"]
#[belongs_to(AdminServiceEventModel, foreign_key = "event_id")]
#[primary_key(event_id, service_id)]
pub struct AdminEventProposedServiceModel {
    pub event_id: i64,
    pub service_id: String,
    pub service_type: String,
    pub node_id: String,
    pub position: i32,
}

impl AdminEventProposedServiceModel {
    // Creates a list of `AdminEventProposedServiceModel` from a `CircuitProposal` associated
    // with an `AdminServiceEvent`
    pub(super) fn list_from_proposal_with_id(
        event_id: i64,
        proposal: &messages::CircuitProposal,
    ) -> Result<Vec<AdminEventProposedServiceModel>, AdminServiceEventStoreError> {
        proposal
            .circuit
            .roster
            .iter()
            .enumerate()
            .map(|(idx, service)| {
                Ok(AdminEventProposedServiceModel {
                    event_id,
                    service_id: service.service_id.to_string(),
                    service_type: service.service_type.to_string(),
                    node_id: service
                        .allowed_nodes
                        .get(0)
                        .ok_or_else(|| {
                            AdminServiceEventStoreError::InvalidStateError(
                                InvalidStateError::with_message(
                                    "Must contain 1 node ID".to_string(),
                                ),
                            )
                        })?
                        .to_string(),
                    position: i32::try_from(idx).map_err(|_| {
                        AdminServiceEventStoreError::InternalError(InternalError::with_message(
                            "Unable to convert index into i32".to_string(),
                        ))
                    })?,
                })
            })
            .collect::<Result<Vec<AdminEventProposedServiceModel>, AdminServiceEventStoreError>>()
    }
}

/// Database model representation of the arguments associated with a `ProposedService` from an
/// `AdminServiceEvent`
#[derive(Debug, PartialEq, Associations, Identifiable, Insertable, Queryable, QueryableByName)]
#[table_name = "admin_event_proposed_service_argument"]
#[belongs_to(AdminServiceEventModel, foreign_key = "event_id")]
#[primary_key(event_id, service_id, key)]
pub struct AdminEventProposedServiceArgumentModel {
    pub event_id: i64,
    pub service_id: String,
    pub key: String,
    pub value: String,
    pub position: i32,
}

impl AdminEventProposedServiceArgumentModel {
    // Creates a list of `AdminEventProposedServiceArgumentModel` from a `CircuitProposal` associated
    // with an `AdminServiceEvent`
    pub(super) fn list_from_proposal_with_id(
        event_id: i64,
        proposal: &messages::CircuitProposal,
    ) -> Result<Vec<AdminEventProposedServiceArgumentModel>, AdminServiceEventStoreError> {
        let mut service_arguments = Vec::new();
        for service in &proposal.circuit.roster {
            service_arguments.extend(
                service
                    .arguments
                    .iter()
                    .enumerate()
                    .map(|(idx, (key, value))| {
                        Ok(AdminEventProposedServiceArgumentModel {
                            event_id,
                            service_id: service.service_id.to_string(),
                            key: key.into(),
                            value: value.into(),
                            position: i32::try_from(idx).map_err(|_| {
                                AdminServiceEventStoreError::InternalError(
                                    InternalError::with_message(
                                        "Unable to convert index into i32".to_string(),
                                    ),
                                )
                            })?,
                        })
                    })
                    .collect::<Result<
                        Vec<AdminEventProposedServiceArgumentModel>,
                        AdminServiceEventStoreError,
                    >>()?,
            );
        }
        Ok(service_arguments)
    }
}

// All enums associated with the above structs have TryFrom and From implemented in order to
// translate the enums to a `Text` representation to be stored in the database.

impl From<&messages::Vote> for String {
    fn from(variant: &messages::Vote) -> Self {
        match variant {
            messages::Vote::Accept => String::from("Accept"),
            messages::Vote::Reject => String::from("Reject"),
        }
    }
}

impl From<&messages::ProposalType> for String {
    fn from(variant: &messages::ProposalType) -> Self {
        match variant {
            messages::ProposalType::Create => String::from("Create"),
            messages::ProposalType::UpdateRoster => String::from("UpdateRoster"),
            messages::ProposalType::AddNode => String::from("AddNode"),
            messages::ProposalType::RemoveNode => String::from("RemoveNode"),
            messages::ProposalType::Destroy => String::from("Destroy"),
        }
    }
}

impl From<&messages::AuthorizationType> for String {
    fn from(variant: &messages::AuthorizationType) -> Self {
        match variant {
            messages::AuthorizationType::Trust => String::from("Trust"),
        }
    }
}

impl From<&messages::PersistenceType> for String {
    fn from(variant: &messages::PersistenceType) -> Self {
        match variant {
            messages::PersistenceType::Any => String::from("Any"),
        }
    }
}

impl From<&messages::DurabilityType> for String {
    fn from(variant: &messages::DurabilityType) -> Self {
        match variant {
            messages::DurabilityType::NoDurability => String::from("NoDurability"),
        }
    }
}

impl From<&messages::RouteType> for String {
    fn from(variant: &messages::RouteType) -> Self {
        match variant {
            messages::RouteType::Any => String::from("Any"),
        }
    }
}

impl<'a> From<&'a messages::AdminServiceEvent> for NewAdminServiceEventModel<'a> {
    fn from(event: &'a messages::AdminServiceEvent) -> Self {
        match event {
            messages::AdminServiceEvent::ProposalSubmitted(_) => NewAdminServiceEventModel {
                event_type: "ProposalSubmitted",
                data: None,
            },
            messages::AdminServiceEvent::ProposalVote((_, data)) => NewAdminServiceEventModel {
                event_type: "ProposalVote",
                data: Some(data),
            },
            messages::AdminServiceEvent::ProposalAccepted((_, data)) => NewAdminServiceEventModel {
                event_type: "ProposalAccepted",
                data: Some(data),
            },
            messages::AdminServiceEvent::ProposalRejected((_, data)) => NewAdminServiceEventModel {
                event_type: "ProposalRejected",
                data: Some(data),
            },
            messages::AdminServiceEvent::CircuitReady(_) => NewAdminServiceEventModel {
                event_type: "CircuitReady",
                data: None,
            },
        }
    }
}

impl TryFrom<(AdminServiceEventModel, CircuitProposal)> for AdminServiceEvent {
    type Error = AdminServiceEventStoreError;

    fn try_from(
        (event_model, proposal): (AdminServiceEventModel, CircuitProposal),
    ) -> Result<Self, Self::Error> {
        match (event_model.event_type.as_ref(), event_model.data) {
            ("ProposalSubmitted", None) => Ok(AdminServiceEvent {
                event_id: event_model.id,
                event_type: EventType::ProposalSubmitted,
                proposal,
            }),
            ("ProposalVote", Some(requester)) => Ok(AdminServiceEvent {
                event_id: event_model.id,
                event_type: EventType::ProposalVote { requester },
                proposal,
            }),
            ("ProposalAccepted", Some(requester)) => Ok(AdminServiceEvent {
                event_id: event_model.id,
                event_type: EventType::ProposalAccepted { requester },
                proposal,
            }),
            ("ProposalRejected", Some(requester)) => Ok(AdminServiceEvent {
                event_id: event_model.id,
                event_type: EventType::ProposalRejected { requester },
                proposal,
            }),
            ("CircuitReady", None) => Ok(AdminServiceEvent {
                event_id: event_model.id,
                event_type: EventType::CircuitReady,
                proposal,
            }),
            _ => Err(AdminServiceEventStoreError::InvalidStateError(
                InvalidStateError::with_message(
                    "Unable to convert AdminServiceEventModel to AdminServiceEvent".into(),
                ),
            )),
        }
    }
}
