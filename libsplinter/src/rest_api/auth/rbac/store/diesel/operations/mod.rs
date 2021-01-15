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

pub(super) mod add_assignment;
pub(super) mod add_role;
pub(super) mod get_assignment;
pub(super) mod get_role;
pub(super) mod list_assignments;
pub(super) mod list_roles;
pub(super) mod remove_assignment;
pub(super) mod remove_role;
pub(super) mod update_assignment;
pub(super) mod update_role;

pub(super) struct RoleBasedAuthorizationStoreOperations<'a, C> {
    conn: &'a C,
}

impl<'a, C> RoleBasedAuthorizationStoreOperations<'a, C>
where
    C: diesel::Connection,
{
    pub fn new(conn: &'a C) -> Self {
        Self { conn }
    }
}
