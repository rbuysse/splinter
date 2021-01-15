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

use diesel::{
    dsl::{delete, insert_into, update},
    prelude::*,
};

use crate::rest_api::auth::rbac::store::{
    diesel::{
        models::{RoleModel, RolePermissionModel},
        schema::{role_permissions, roles},
    },
    Role, RoleBasedAuthorizationStoreError,
};

use super::RoleBasedAuthorizationStoreOperations;

pub trait RoleBasedAuthorizationStoreUpdateRole {
    fn update_role(&self, role: Role) -> Result<(), RoleBasedAuthorizationStoreError>;
}

#[cfg(feature = "sqlite")]
impl<'a> RoleBasedAuthorizationStoreUpdateRole
    for RoleBasedAuthorizationStoreOperations<'a, diesel::sqlite::SqliteConnection>
{
    fn update_role(&self, role: Role) -> Result<(), RoleBasedAuthorizationStoreError> {
        let (role, permissions): (RoleModel, Vec<RolePermissionModel>) = role.into();

        self.conn.transaction::<_, _, _>(|| {
            delete(role_permissions::table.filter(role_permissions::role_id.eq(&role.id)))
                .execute(self.conn)?;

            update(roles::table.find(&role.id))
                .set(roles::display_name.eq(&role.display_name))
                .execute(self.conn)?;

            insert_into(role_permissions::table)
                .values(permissions)
                .execute(self.conn)?;

            Ok(())
        })
    }
}

#[cfg(feature = "role-based-authorization-store-postgres")]
impl<'a> RoleBasedAuthorizationStoreUpdateRole
    for RoleBasedAuthorizationStoreOperations<'a, diesel::pg::PgConnection>
{
    fn update_role(&self, role: Role) -> Result<(), RoleBasedAuthorizationStoreError> {
        let (role, permissions): (RoleModel, Vec<RolePermissionModel>) = role.into();

        self.conn.transaction::<_, _, _>(|| {
            delete(role_permissions::table.filter(role_permissions::role_id.eq(&role.id)))
                .execute(self.conn)?;

            update(roles::table.find(&role.id))
                .set(roles::display_name.eq(&role.display_name))
                .execute(self.conn)?;

            insert_into(role_permissions::table)
                .values(permissions)
                .execute(self.conn)?;

            Ok(())
        })
    }
}
