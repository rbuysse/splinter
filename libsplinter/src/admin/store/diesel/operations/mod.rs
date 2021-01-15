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

//! Provides database operations for the `DieselAdminServiceStore`.

pub(super) mod add_circuit;
pub(super) mod add_proposal;
pub(super) mod get_circuit;
pub(super) mod get_node;
pub(super) mod get_proposal;
pub(super) mod get_service;
pub(super) mod list_circuits;
pub(super) mod list_nodes;
pub(super) mod list_proposals;
pub(super) mod list_services;
pub(super) mod remove_circuit;
pub(super) mod remove_proposal;
pub(super) mod update_circuit;
pub(super) mod update_proposal;
pub(super) mod upgrade;

pub struct AdminServiceStoreOperations<'a, C> {
    conn: &'a C,
}

impl<'a, C: diesel::Connection> AdminServiceStoreOperations<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        AdminServiceStoreOperations { conn }
    }
}
