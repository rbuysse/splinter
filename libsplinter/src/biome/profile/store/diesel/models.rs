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

use super::schema::user_profile;

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[table_name = "user_profile"]
pub struct ProfileModel {
    pub id: i64,
    pub user_id: String,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub email: Option<String>,
    pub picture: Option<String>,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "user_profile"]
pub struct NewProfileModel {
    pub user_id: String,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub email: Option<String>,
    pub picture: Option<String>,
}
