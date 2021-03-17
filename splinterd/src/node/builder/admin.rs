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

//! Builder for the AdminSubsystem

use std::time::Duration;

use cylinder::{secp256k1::Secp256k1Context, Context, VerifierFactory};
use scabbard::service::ScabbardFactoryBuilder;
use splinter::circuit::routing::RoutingTableWriter;
use splinter::error::InternalError;
use splinter::peer::PeerManagerConnector;
use splinter::store::{memory::MemoryStoreFactory, StoreFactory};
use splinter::transport::inproc::InprocTransport;

use crate::node::builder::scabbard::ScabbardConfig;
use crate::node::runnable::admin::RunnableAdminSubsystem;

const DEFAULT_ADMIN_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Default)]
pub struct AdminSubsystemBuilder {
    node_id: Option<String>,
    admin_timeout: Option<Duration>,
    store_factory: Option<Box<dyn StoreFactory>>,
    peer_connector: Option<PeerManagerConnector>,
    routing_writer: Option<Box<dyn RoutingTableWriter>>,
    service_transport: Option<InprocTransport>,
    signing_context: Option<Box<dyn Context>>,
    scabbard_config: Option<ScabbardConfig>,
}

impl AdminSubsystemBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Specifies the id for the node. Defaults to a random node id.
    pub fn with_node_id(mut self, node_id: String) -> Self {
        self.node_id = Some(node_id);
        self
    }

    /// Specifies the timeout for admin requests. Defaults to 30 seconds.
    pub fn with_admin_timeout(mut self, admin_timeout: Duration) -> Self {
        self.admin_timeout = Some(admin_timeout);
        self
    }

    /// Specifies the store factory to use with the node. Defaults to the MemoryStoreFactory.
    pub fn with_store_factory(mut self, store_factory: Box<dyn StoreFactory>) -> Self {
        self.store_factory = Some(store_factory);
        self
    }

    /// Specifies the peer connector to use with the node
    pub fn with_peer_connector(mut self, peer_connector: PeerManagerConnector) -> Self {
        self.peer_connector = Some(peer_connector);
        self
    }

    /// Specifies the routing table writer that will be used by the admin service
    pub fn with_routing_writer(mut self, routing_writer: Box<dyn RoutingTableWriter>) -> Self {
        self.routing_writer = Some(routing_writer);
        self
    }

    /// Specifies the transport to be used to set up inproc connections
    pub fn with_service_transport(mut self, service_transport: InprocTransport) -> Self {
        self.service_transport = Some(service_transport);
        self
    }

    /// Configure a signing context. Defaults to [cylinder::secp256k1::Secp256k1Context].
    pub fn with_signing_context(mut self, signing_context: Box<dyn Context>) -> Self {
        self.signing_context = Some(signing_context);
        self
    }

    /// Make scabbard services available to circuits created by this admin subsystem.
    pub fn with_scabbard(mut self, scabbard_config: ScabbardConfig) -> Self {
        self.scabbard_config = Some(scabbard_config);
        self
    }

    pub fn build(mut self) -> Result<RunnableAdminSubsystem, InternalError> {
        let node_id = self.node_id.take().ok_or_else(|| {
            InternalError::with_message("Cannot build AdminSubsystem without a node id".to_string())
        })?;

        let admin_timeout = self.admin_timeout.unwrap_or(DEFAULT_ADMIN_TIMEOUT);

        let store_factory = match self.store_factory {
            Some(store_factory) => store_factory,
            None => Box::new(MemoryStoreFactory::new()?),
        };

        let peer_connector = self.peer_connector.take().ok_or_else(|| {
            InternalError::with_message(
                "Cannot build AdminSubsystem without a peer connector".to_string(),
            )
        })?;

        let routing_writer = self.routing_writer.take().ok_or_else(|| {
            InternalError::with_message(
                "Cannot build AdminSubsystem without a routing writer".to_string(),
            )
        })?;

        let service_transport = self.service_transport.take().ok_or_else(|| {
            InternalError::with_message(
                "Cannot build AdminSubsystem without a service transport".to_string(),
            )
        })?;

        let signing_context = self
            .signing_context
            .unwrap_or_else(|| Box::new(Secp256k1Context::new()));

        let admin_service_verifier = signing_context.new_verifier();

        let scabbard_service_factory = self
            .scabbard_config
            .map(|config| {
                ScabbardFactoryBuilder::new()
                    .with_state_db_dir(config.data_dir.to_string_lossy().into())
                    .with_state_db_size(config.database_size)
                    .with_receipt_db_dir(config.data_dir.to_string_lossy().into())
                    .with_receipt_db_size(config.database_size)
                    .with_signature_verifier_factory(Box::new(VerifierFactoryWrapper(
                        signing_context,
                    )))
                    .build()
                    .map_err(|e| InternalError::from_source(Box::new(e)))
            })
            .transpose()?;

        Ok(RunnableAdminSubsystem {
            node_id,
            admin_timeout,
            store_factory,
            peer_connector,
            routing_writer,
            service_transport,
            admin_service_verifier,
            scabbard_service_factory,
        })
    }
}

struct VerifierFactoryWrapper(Box<dyn Context>);

impl VerifierFactory for VerifierFactoryWrapper {
    fn new_verifier(&self) -> Box<dyn cylinder::Verifier> {
        self.0.new_verifier()
    }
}