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

//! Contains a `StoreFactory` trait, which is an abstract factory for building stores
//! backed by a single storage mechanism (e.g. database)

pub mod memory;
#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "sqlite")]
pub mod sqlite;

use std::str::FromStr;

#[cfg(feature = "sqlite")]
use self::sqlite::ForeignKeyCustomizer;
#[cfg(feature = "diesel")]
use diesel::r2d2::{ConnectionManager, Pool};

use crate::error::{InternalError, InvalidArgumentError};

/// An abstract factory for creating Splinter stores backed by the same storage
pub trait StoreFactory {
    /// Get a new `CredentialsStore`
    #[cfg(feature = "biome-credentials")]
    fn get_biome_credentials_store(&self) -> Box<dyn crate::biome::CredentialsStore>;

    /// Get a new `KeyStore`
    #[cfg(feature = "biome-key-management")]
    fn get_biome_key_store(&self) -> Box<dyn crate::biome::KeyStore>;

    /// Get a new `RefreshTokenStore`
    #[cfg(feature = "biome-credentials")]
    fn get_biome_refresh_token_store(&self) -> Box<dyn crate::biome::RefreshTokenStore>;

    /// Get a new `OAuthUserSessionStore`
    #[cfg(feature = "biome-oauth")]
    fn get_biome_oauth_user_session_store(&self) -> Box<dyn crate::biome::OAuthUserSessionStore>;

    #[cfg(feature = "admin-service")]
    fn get_admin_service_store(&self) -> Box<dyn crate::admin::store::AdminServiceStore>;

    #[cfg(feature = "oauth")]
    fn get_oauth_inflight_request_store(
        &self,
    ) -> Box<dyn crate::oauth::store::InflightOAuthRequestStore>;

    #[cfg(feature = "registry-database")]
    fn get_registry_store(&self) -> Box<dyn crate::registry::RwRegistry>;

    #[cfg(feature = "authorization")]
    fn get_role_based_authorization_store(
        &self,
    ) -> Box<dyn crate::rest_api::auth::rbac::store::RoleBasedAuthorizationStore>;

    #[cfg(feature = "biome-profile")]
    fn get_biome_user_profile_store(&self) -> Box<dyn crate::biome::UserProfileStore>;

    #[cfg(feature = "admin-service-event-store")]
    fn get_admin_service_event_store(
        &self,
    ) -> Box<dyn crate::admin::service::event::store::AdminServiceEventStore>;
}

/// Creates a `StoreFactory` backed by the given connection
///
/// # Arguments
///
/// * `connection_uri` - The identifier of the storage connection that will be used by all stores
///   created by the resulting factory
pub fn create_store_factory(
    connection_uri: ConnectionUri,
) -> Result<Box<dyn StoreFactory>, InternalError> {
    match connection_uri {
        ConnectionUri::Memory => Ok(Box::new(memory::MemoryStoreFactory::new())),
        #[cfg(feature = "postgres")]
        ConnectionUri::Postgres(url) => {
            let connection_manager = ConnectionManager::<diesel::pg::PgConnection>::new(url);
            let pool = Pool::builder().build(connection_manager).map_err(|err| {
                InternalError::from_source_with_prefix(
                    Box::new(err),
                    "Failed to build connection pool".to_string(),
                )
            })?;
            Ok(Box::new(postgres::PgStoreFactory::new(pool)))
        }
        #[cfg(feature = "sqlite")]
        ConnectionUri::Sqlite(conn_str) => {
            let connection_manager =
                ConnectionManager::<diesel::sqlite::SqliteConnection>::new(&conn_str);
            let mut pool_builder =
                Pool::builder().connection_customizer(Box::new(ForeignKeyCustomizer::default()));
            // A new database is created for each connection to the in-memory SQLite
            // implementation; to ensure that the resulting stores will operate on the same
            // database, only one connection is allowed.
            if conn_str == ":memory:" {
                pool_builder = pool_builder.max_size(1);
            }
            let pool = pool_builder.build(connection_manager).map_err(|err| {
                InternalError::from_source_with_prefix(
                    Box::new(err),
                    "Failed to build connection pool".to_string(),
                )
            })?;
            Ok(Box::new(sqlite::SqliteStoreFactory::new(pool)))
        }
    }
}

/// The possible connection types and identifiers for a `StoreFactory`
pub enum ConnectionUri {
    Memory,
    #[cfg(feature = "postgres")]
    Postgres(String),
    #[cfg(feature = "sqlite")]
    Sqlite(String),
}

impl FromStr for ConnectionUri {
    type Err = InvalidArgumentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "memory" => Ok(ConnectionUri::Memory),
            #[cfg(feature = "postgres")]
            _ if s.starts_with("postgres://") => Ok(ConnectionUri::Postgres(s.into())),
            #[cfg(feature = "sqlite")]
            _ => Ok(ConnectionUri::Sqlite(s.into())),
            #[cfg(not(feature = "sqlite"))]
            _ => Err(InvalidArgumentError::new(
                "s".to_string(),
                format!("No compatible connection type: {}", s),
            )),
        }
    }
}
