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

//! Implementation of a `StoreFactory` for in memory

#[cfg(feature = "sqlite")]
use diesel::{
    r2d2::{ConnectionManager, Pool},
    sqlite::SqliteConnection,
};

#[cfg(feature = "admin-service-event-store")]
use crate::admin::service::event::store::memory::MemoryAdminServiceEventStore;
#[cfg(feature = "biome-oauth")]
use crate::biome::MemoryOAuthUserSessionStore;
#[cfg(feature = "biome-credentials")]
use crate::biome::{
    CredentialsStore, MemoryCredentialsStore, MemoryRefreshTokenStore, RefreshTokenStore,
};
#[cfg(feature = "biome-key-management")]
use crate::biome::{KeyStore, MemoryKeyStore};
#[cfg(feature = "biome-profile")]
use crate::biome::{MemoryUserProfileStore, UserProfileStore};
#[cfg(feature = "oauth")]
use crate::oauth::store::MemoryInflightOAuthRequestStore;

use super::StoreFactory;

#[cfg(feature = "admin-service-event-store")]
pub(in crate::store) const DEFAULT_IN_MEMORY_EVENT_LIMIT: usize = 100;

/// A `StoryFactory` backed by memory.
#[derive(Default)]
pub struct MemoryStoreFactory {
    #[cfg(feature = "biome-credentials")]
    biome_credentials_store: MemoryCredentialsStore,
    #[cfg(feature = "biome-key-management")]
    biome_key_store: MemoryKeyStore,
    #[cfg(feature = "biome-credentials")]
    biome_refresh_token_store: MemoryRefreshTokenStore,
    #[cfg(feature = "biome-oauth")]
    biome_oauth_user_session_store: MemoryOAuthUserSessionStore,
    #[cfg(feature = "oauth")]
    inflight_request_store: MemoryInflightOAuthRequestStore,
    #[cfg(feature = "biome-profile")]
    biome_profile_store: MemoryUserProfileStore,
}

impl MemoryStoreFactory {
    pub fn new() -> Self {
        #[cfg(feature = "biome-credentials")]
        let biome_credentials_store = MemoryCredentialsStore::new();

        #[cfg(all(feature = "biome-key-management", feature = "biome-credentials"))]
        let biome_key_store = MemoryKeyStore::new(biome_credentials_store.clone());
        #[cfg(all(feature = "biome-key-management", not(feature = "biome-credentials")))]
        let biome_key_store = MemoryKeyStore::new();

        #[cfg(feature = "biome-oauth")]
        let biome_oauth_user_session_store = MemoryOAuthUserSessionStore::new();

        #[cfg(feature = "oauth")]
        let inflight_request_store = MemoryInflightOAuthRequestStore::new();

        #[cfg(feature = "biome-profile")]
        let biome_profile_store = MemoryUserProfileStore::new();

        Self {
            #[cfg(feature = "biome-credentials")]
            biome_credentials_store,
            #[cfg(feature = "biome-key-management")]
            biome_key_store,
            #[cfg(feature = "biome-credentials")]
            biome_refresh_token_store: MemoryRefreshTokenStore::new(),
            #[cfg(feature = "biome-oauth")]
            biome_oauth_user_session_store,
            #[cfg(feature = "oauth")]
            inflight_request_store,
            #[cfg(feature = "biome-profile")]
            biome_profile_store,
        }
    }
}

impl StoreFactory for MemoryStoreFactory {
    #[cfg(feature = "biome-credentials")]
    fn get_biome_credentials_store(&self) -> Box<dyn CredentialsStore> {
        Box::new(self.biome_credentials_store.clone())
    }

    #[cfg(feature = "biome-key-management")]
    fn get_biome_key_store(&self) -> Box<dyn KeyStore> {
        Box::new(self.biome_key_store.clone())
    }

    #[cfg(feature = "biome-credentials")]
    fn get_biome_refresh_token_store(&self) -> Box<dyn RefreshTokenStore> {
        Box::new(self.biome_refresh_token_store.clone())
    }

    #[cfg(feature = "biome-oauth")]
    fn get_biome_oauth_user_session_store(&self) -> Box<dyn crate::biome::OAuthUserSessionStore> {
        Box::new(self.biome_oauth_user_session_store.clone())
    }

    #[cfg(all(feature = "admin-service", feature = "sqlite"))]
    fn get_admin_service_store(&self) -> Box<dyn crate::admin::store::AdminServiceStore> {
        let connection_manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = Pool::builder()
            .max_size(1)
            .build(connection_manager)
            .expect("Failed to build connection pool");

        crate::migrations::run_sqlite_migrations(
            &*pool.get().expect("Failed to get connection for migrations"),
        )
        .expect("Failed to run migrations");

        Box::new(crate::admin::store::diesel::DieselAdminServiceStore::new(
            pool,
        ))
    }

    #[cfg(all(feature = "admin-service", not(feature = "sqlite")))]
    fn get_admin_service_store(&self) -> Box<dyn crate::admin::store::AdminServiceStore> {
        unimplemented!()
    }

    #[cfg(feature = "oauth")]
    fn get_oauth_inflight_request_store(
        &self,
    ) -> Box<dyn crate::oauth::store::InflightOAuthRequestStore> {
        Box::new(self.inflight_request_store.clone())
    }

    #[cfg(all(feature = "registry-database", feature = "sqlite"))]
    fn get_registry_store(&self) -> Box<dyn crate::registry::RwRegistry> {
        let connection_manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = Pool::builder()
            .max_size(1)
            .build(connection_manager)
            .expect("Failed to build connection pool");

        crate::migrations::run_sqlite_migrations(
            &*pool.get().expect("Failed to get connection for migrations"),
        )
        .expect("Failed to run migrations");

        Box::new(crate::registry::DieselRegistry::new(pool))
    }

    #[cfg(all(feature = "registry-database", not(feature = "sqlite")))]
    fn get_registry_store(&self) -> Box<dyn crate::registry::RwRegistry> {
        unimplemented!()
    }

    #[cfg(all(feature = "authorization", feature = "sqlite"))]
    fn get_role_based_authorization_store(
        &self,
    ) -> Box<dyn crate::rest_api::auth::rbac::store::RoleBasedAuthorizationStore> {
        let connection_manager = ConnectionManager::<SqliteConnection>::new(":memory:");
        let pool = Pool::builder()
            .max_size(1)
            .build(connection_manager)
            .expect("Failed to build connection pool");

        crate::migrations::run_sqlite_migrations(
            &*pool.get().expect("Failed to get connection for migrations"),
        )
        .expect("Failed to run migrations");

        Box::new(crate::rest_api::auth::rbac::store::DieselRoleBasedAuthorizationStore::new(pool))
    }

    #[cfg(all(feature = "authorization", not(feature = "sqlite")))]
    fn get_role_based_authorization_store(
        &self,
    ) -> Box<dyn crate::rest_api::auth::roles::store::RoleBasedAuthorizationStore> {
        unimplemented!()
    }

    #[cfg(feature = "biome-profile")]
    fn get_biome_user_profile_store(&self) -> Box<dyn UserProfileStore> {
        Box::new(self.biome_profile_store.clone())
    }

    #[cfg(feature = "admin-service-event-store")]
    fn get_admin_service_event_store(
        &self,
    ) -> Box<dyn crate::admin::service::event::store::AdminServiceEventStore> {
        MemoryAdminServiceEventStore::new_boxed_with_bound(
            std::num::NonZeroUsize::new(DEFAULT_IN_MEMORY_EVENT_LIMIT).unwrap(),
        )
    }
}
