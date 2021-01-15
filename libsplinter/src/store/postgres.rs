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

//! Implementation of a `StoreFactory` for PostgreSQL

use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};

use super::StoreFactory;

/// A `StoryFactory` backed by a PostgreSQL database.
pub struct PgStoreFactory {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PgStoreFactory {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

impl StoreFactory for PgStoreFactory {
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

    #[cfg(feature = "biome-oauth-user-store-postgres")]
    fn get_biome_oauth_user_session_store(&self) -> Box<dyn crate::biome::OAuthUserSessionStore> {
        Box::new(crate::biome::DieselOAuthUserSessionStore::new(
            self.pool.clone(),
        ))
    }

    #[cfg(all(feature = "biome-oauth", not(feature = "postgres")))]
    fn get_biome_oauth_user_session_store(&self) -> Box<dyn crate::biome::OAuthUserSessionStore> {
        // This configuration cannot be reached within this implementation as the whole struct is
        // guarded by "postgres". It merely satisfies the compiler.
        unreachable!()
    }

    #[cfg(feature = "admin-service")]
    fn get_admin_service_store(&self) -> Box<dyn crate::admin::store::AdminServiceStore> {
        Box::new(crate::admin::store::diesel::DieselAdminServiceStore::new(
            self.pool.clone(),
        ))
    }

    #[cfg(feature = "oauth-inflight-request-store-postgres")]
    fn get_oauth_inflight_request_store(
        &self,
    ) -> Box<dyn crate::oauth::store::InflightOAuthRequestStore> {
        Box::new(crate::oauth::store::DieselInflightOAuthRequestStore::new(
            self.pool.clone(),
        ))
    }

    #[cfg(all(
        feature = "oauth",
        not(feature = "oauth-inflight-request-store-postgres")
    ))]
    fn get_oauth_inflight_request_store(
        &self,
    ) -> Box<dyn crate::oauth::store::InflightOAuthRequestStore> {
        unimplemented!()
    }

    #[cfg(feature = "registry-database")]
    fn get_registry_store(&self) -> Box<dyn crate::registry::RwRegistry> {
        Box::new(crate::registry::DieselRegistry::new(self.pool.clone()))
    }

    #[cfg(feature = "role-based-authorization-store-postgres")]
    fn get_role_based_authorization_store(
        &self,
    ) -> Box<dyn crate::rest_api::auth::rbac::store::RoleBasedAuthorizationStore> {
        Box::new(
            crate::rest_api::auth::rbac::store::DieselRoleBasedAuthorizationStore::new(
                self.pool.clone(),
            ),
        )
    }

    #[cfg(all(
        feature = "authorization",
        not(feature = "role-based-authorization-store-postgres")
    ))]
    fn get_role_based_authorization_store(
        &self,
    ) -> Box<dyn crate::rest_api::auth::rbac::store::RoleBasedAuthorizationStore> {
        unimplemented!()
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
