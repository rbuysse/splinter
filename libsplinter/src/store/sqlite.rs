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

//! Implementation of a `StoreFactory` for SQLite

use diesel::{
    connection::SimpleConnection,
    r2d2::{ConnectionManager, CustomizeConnection, Pool},
    sqlite::SqliteConnection,
};

use super::StoreFactory;

/// A `StoreFactory` backed by a SQLite database.
pub struct SqliteStoreFactory {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl SqliteStoreFactory {
    /// Create a new `SqliteStoreFactory`.
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        Self { pool }
    }
}

impl StoreFactory for SqliteStoreFactory {
    #[cfg(feature = "biome-credentials")]
    fn get_biome_credentials_store(&self) -> Box<dyn crate::biome::CredentialsStore> {
        Box::new(crate::biome::DieselCredentialsStore::new(self.pool.clone()))
    }

    #[cfg(feature = "biome-key-management")]
    fn get_biome_key_store(&self) -> Box<dyn crate::biome::KeyStore> {
        Box::new(crate::biome::DieselKeyStore::new(self.pool.clone()))
    }

    #[cfg(feature = "biome-credentials")]
    fn get_biome_refresh_token_store(&self) -> Box<dyn crate::biome::RefreshTokenStore> {
        Box::new(crate::biome::DieselRefreshTokenStore::new(
            self.pool.clone(),
        ))
    }

    #[cfg(feature = "biome-oauth")]
    fn get_biome_oauth_user_session_store(&self) -> Box<dyn crate::biome::OAuthUserSessionStore> {
        Box::new(crate::biome::DieselOAuthUserSessionStore::new(
            self.pool.clone(),
        ))
    }

    #[cfg(feature = "admin-service")]
    fn get_admin_service_store(&self) -> Box<dyn crate::admin::store::AdminServiceStore> {
        Box::new(crate::admin::store::diesel::DieselAdminServiceStore::new(
            self.pool.clone(),
        ))
    }

    #[cfg(feature = "oauth")]
    fn get_oauth_inflight_request_store(
        &self,
    ) -> Box<dyn crate::oauth::store::InflightOAuthRequestStore> {
        Box::new(crate::oauth::store::DieselInflightOAuthRequestStore::new(
            self.pool.clone(),
        ))
    }

    #[cfg(feature = "registry-database")]
    fn get_registry_store(&self) -> Box<dyn crate::registry::RwRegistry> {
        Box::new(crate::registry::DieselRegistry::new(self.pool.clone()))
    }

    #[cfg(feature = "authorization")]
    fn get_role_based_authorization_store(
        &self,
    ) -> Box<dyn crate::rest_api::auth::rbac::store::RoleBasedAuthorizationStore> {
        Box::new(
            crate::rest_api::auth::rbac::store::DieselRoleBasedAuthorizationStore::new(
                self.pool.clone(),
            ),
        )
    }

    #[cfg(feature = "biome-profile")]
    fn get_biome_user_profile_store(&self) -> Box<dyn crate::biome::UserProfileStore> {
        Box::new(crate::biome::DieselUserProfileStore::new(self.pool.clone()))
    }

    #[cfg(feature = "admin-service-event-store-diesel")]
    fn get_admin_service_event_store(
        &self,
    ) -> Box<dyn crate::admin::service::event::store::AdminServiceEventStore> {
        Box::new(
            crate::admin::service::event::store::diesel::DieselAdminServiceEventStore::new(
                self.pool.clone(),
            ),
        )
    }

    #[cfg(all(
        feature = "admin-service-event-store",
        not(feature = "admin-service-event-store-diesel")
    ))]
    fn get_admin_service_event_store(
        &self,
    ) -> Box<dyn crate::admin::service::event::store::AdminServiceEventStore> {
        unimplemented!()
    }
}

#[derive(Default, Debug)]
/// Foreign keys must be enabled on a per connection basis. This customizer will be added to the
/// SQLite pool builder and then ran against every connection returned from the pool.
pub struct ForeignKeyCustomizer;

impl CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for ForeignKeyCustomizer {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        conn.batch_execute("PRAGMA foreign_keys = ON;")
            .map_err(diesel::r2d2::Error::QueryError)
    }
}
