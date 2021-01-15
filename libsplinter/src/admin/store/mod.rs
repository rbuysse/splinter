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

//! Data store for writing and reading circuit state and pending circuit proposals.
//!
//! The [`AdminServiceStore`] trait provides the public interface for storing circuits and
//! proposals. Splinter provides the following implementations of this trait:
//!
//! * [`YamlAdminServiceStore`] - A YAML-backed store that is available by default
//! * [`DieselAdminServiceStore`] - A database-backed store, powered by [`Diesel`], that currently
//!   supports SQLite databases (with the `sqlite` feature) and PostgreSQL databases (with the
//!   `postgres` feature).
//!
//! [`AdminServiceStore`]: trait.AdminServiceStore.html
//! [`YamlAdminServiceStore`]: yaml/struct.YamlAdminServiceStore.html
//! [`DieselAdminServiceStore`]: diesel/struct.DieselAdminServiceStore.html
//! [`Diesel`]: https://crates.io/crates/diesel

mod circuit;
mod circuit_node;
mod circuit_proposal;
#[cfg(any(feature = "postgres", feature = "sqlite"))]
pub mod diesel;
pub mod error;
mod proposed_circuit;
mod proposed_node;
mod proposed_service;
mod service;
pub mod yaml;

use std::cmp::Ordering;
use std::fmt;

pub use self::circuit::{
    AuthorizationType, Circuit, CircuitBuilder, DurabilityType, PersistenceType, RouteType,
};
pub use self::circuit_node::{CircuitNode, CircuitNodeBuilder};
pub use self::circuit_proposal::{
    CircuitProposal, CircuitProposalBuilder, ProposalType, Vote, VoteRecord, VoteRecordBuilder,
};
use self::error::AdminServiceStoreError;
pub use self::proposed_circuit::{ProposedCircuit, ProposedCircuitBuilder};
pub use self::proposed_node::{ProposedNode, ProposedNodeBuilder};
pub use self::proposed_service::{ProposedService, ProposedServiceBuilder};
pub use self::service::{Service, ServiceBuilder};

pub const UNSET_CIRCUIT_VERSION: i32 = 1;

/// The unique ID of service made up of a circuit ID and the individual service ID.
/// A service ID is only required to be unique from within a circuit.
#[derive(Clone, Debug, PartialEq)]
pub struct ServiceId {
    circuit_id: String,
    service_id: String,
}

impl ServiceId {
    /// Create a new service ID
    ///
    /// # Arguments
    ///
    /// * `circuit_id` - the ID of the circuit the service belongs to
    /// * `service_id` - the individual ID for the service
    pub fn new(circuit_id: String, service_id: String) -> Self {
        ServiceId {
            circuit_id,
            service_id,
        }
    }

    /// Returns the circuit ID
    pub fn circuit(&self) -> &str {
        &self.circuit_id
    }

    /// Returns the service ID
    pub fn service_id(&self) -> &str {
        &self.service_id
    }

    /// Decompose this ServiceId into a tuple of (<circuit ID>, <service ID>).
    pub fn into_parts(self) -> (String, String) {
        (self.circuit_id, self.service_id)
    }
}

impl fmt::Display for ServiceId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{}", self.circuit_id, self.service_id)
    }
}

impl Eq for ServiceId {}

impl Ord for ServiceId {
    fn cmp(&self, other: &Self) -> Ordering {
        let compare = self.circuit_id.cmp(&other.circuit_id);
        if compare == Ordering::Equal {
            self.service_id.cmp(&other.service_id)
        } else {
            compare
        }
    }
}

impl PartialOrd for ServiceId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Predicate for filtering the lists of circuits and circuit proposals
pub enum CircuitPredicate {
    ManagementTypeEq(String),
    MembersInclude(Vec<String>),
}

impl CircuitPredicate {
    /// Apply this predicate against a given circuit
    pub fn apply_to_circuit(&self, circuit: &Circuit) -> bool {
        match self {
            CircuitPredicate::ManagementTypeEq(man_type) => {
                circuit.circuit_management_type() == man_type
            }
            CircuitPredicate::MembersInclude(nodes) => {
                for node_id in nodes.iter() {
                    if !circuit.members().contains(node_id) {
                        return false;
                    }
                }
                true
            }
        }
    }

    /// Apply this predicate against a given circuit proposal
    pub fn apply_to_proposals(&self, proposal: &CircuitProposal) -> bool {
        match self {
            CircuitPredicate::ManagementTypeEq(man_type) => {
                proposal.circuit().circuit_management_type() == man_type
            }
            CircuitPredicate::MembersInclude(nodes) => {
                for node_id in nodes {
                    if proposal
                        .circuit()
                        .members()
                        .iter()
                        .find(|node| node_id == node.node_id())
                        .is_none()
                    {
                        return false;
                    }
                }
                true
            }
        }
    }
}

/// Interface for performing CRUD operations on circuits, proposals, nodes, and services
pub trait AdminServiceStore: Send + Sync {
    /// Adds a circuit proposal to the store
    ///
    /// # Arguments
    ///
    ///  * `proposal` - The proposal to be added
    ///
    ///  Returns an error if a `CircuitProposal` with the same ID already exists
    fn add_proposal(&self, proposal: CircuitProposal) -> Result<(), AdminServiceStoreError>;

    /// Updates a circuit proposal in the store
    ///
    /// # Arguments
    ///
    ///  * `proposal` - The proposal with the updated information
    ///
    ///  Returns an error if a `CircuitProposal` with the same ID does not exist
    fn update_proposal(&self, proposal: CircuitProposal) -> Result<(), AdminServiceStoreError>;

    /// Removes a circuit proposal from the store
    ///
    /// # Arguments
    ///
    ///  * `proposal_id` - The unique ID of the circuit proposal to be removed
    ///
    ///  Returns an error if a `CircuitProposal` with specified ID does not exist
    fn remove_proposal(&self, proposal_id: &str) -> Result<(), AdminServiceStoreError>;

    /// Fetches a circuit proposal from the store
    ///
    /// # Arguments
    ///
    ///  * `proposal_id` - The unique ID of the circuit proposal to be returned
    fn get_proposal(
        &self,
        proposal_id: &str,
    ) -> Result<Option<CircuitProposal>, AdminServiceStoreError>;

    /// List circuit proposals from the store
    ///
    /// The proposals returned can be filtered by provided `CircuitPredicate`. This enables
    /// filtering by management type and members.
    fn list_proposals(
        &self,
        predicates: &[CircuitPredicate],
    ) -> Result<Box<dyn ExactSizeIterator<Item = CircuitProposal>>, AdminServiceStoreError>;

    /// Adds a circuit to the store along with the associated services and nodes
    ///
    /// # Arguments
    ///
    ///  * `circuit` - The circuit to be added to state
    ///  * `nodes` - A list of nodes that represent the circuit's members
    ///
    ///  Returns an error if a `Circuit` with the same ID already exists
    fn add_circuit(
        &self,
        circuit: Circuit,
        nodes: Vec<CircuitNode>,
    ) -> Result<(), AdminServiceStoreError>;

    /// Updates a circuit in the store
    ///
    /// # Arguments
    ///
    ///  * `circuit` - The circuit with the updated information
    ///
    ///  Returns an error if a `CircuitProposal` with the same ID does not exist
    fn update_circuit(&self, circuit: Circuit) -> Result<(), AdminServiceStoreError>;

    /// Removes a circuit from the store
    ///
    /// # Arguments
    ///
    ///  * `circuit_id` - The unique ID of the circuit to be removed
    ///
    ///  Returns an error if a `Circuit` with the specified ID does not exist
    fn remove_circuit(&self, circuit_id: &str) -> Result<(), AdminServiceStoreError>;

    /// Fetches a circuit from the store
    ///
    /// # Arguments
    ///
    ///  * `circuit_id` - The unique ID of the circuit to be returned
    fn get_circuit(&self, circuit_id: &str) -> Result<Option<Circuit>, AdminServiceStoreError>;

    /// List all circuits from the store
    ///
    /// `CircuitPredicate`s may be provided for filtering which circuits are returned.
    fn list_circuits(
        &self,
        predicates: &[CircuitPredicate],
    ) -> Result<Box<dyn ExactSizeIterator<Item = Circuit>>, AdminServiceStoreError>;

    /// Adds a circuit, along with the associated services and nodes, to the store based on the
    /// proposal that is already in state. The associated circuit proposal for the circuit ID is
    /// also removed.
    ///
    /// # Arguments
    ///
    ///  * `circuit_id` - The ID of the circuit proposal that should be converted to a circuit
    fn upgrade_proposal_to_circuit(&self, circuit_id: &str) -> Result<(), AdminServiceStoreError>;

    /// Fetches a node from the store
    ///
    /// # Arguments
    ///
    ///  * `node_id` - The unique ID of the node to be returned
    fn get_node(&self, node_id: &str) -> Result<Option<CircuitNode>, AdminServiceStoreError>;

    /// List all nodes from the store
    fn list_nodes(
        &self,
    ) -> Result<Box<dyn ExactSizeIterator<Item = CircuitNode>>, AdminServiceStoreError>;

    /// Fetches a service from the store
    ///
    /// # Arguments
    ///
    ///  * `service_id` - The `ServiceId` of a service made up of the circuit ID and service ID
    fn get_service(
        &self,
        service_id: &ServiceId,
    ) -> Result<Option<Service>, AdminServiceStoreError>;

    /// List all services in a specific circuit from the store
    ///
    /// # Arguments
    ///
    ///  * `circuit_id` - The unique ID of the circuit the services belong to
    fn list_services(
        &self,
        circuit_id: &str,
    ) -> Result<Box<dyn ExactSizeIterator<Item = Service>>, AdminServiceStoreError>;

    fn clone_boxed(&self) -> Box<dyn AdminServiceStore>;
}

impl Clone for Box<dyn AdminServiceStore> {
    fn clone(&self) -> Self {
        self.clone_boxed()
    }
}
