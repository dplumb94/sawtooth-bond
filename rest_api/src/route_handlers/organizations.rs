// Copyright 2018 Bitwise IO
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

use errors::ApiError;
use rocket::http::RawStr;
use rocket_contrib::{Json, Value};

#[post("/organizations")]
pub fn create_organization() -> Result<Json<Value>, ApiError> {
    Err(ApiError::NotImplemented)
}

#[get("/organizations")]
pub fn list_organizations() -> Result<Json<Value>, ApiError> {
    Err(ApiError::NotImplemented)
}

#[post("/organizations/<_organization_id>")]
pub fn update_organization(_organization_id: &RawStr) -> Result<Json<Value>, ApiError> {
    Err(ApiError::NotImplemented)
}

#[get("/organizations/<_organization_id>")]
pub fn retrieve_organization(_organization_id: &RawStr) -> Result<Json<Value>, ApiError> {
    Err(ApiError::NotImplemented)
}

#[post("/organizations/<_organization_id>/auth")]
pub fn update_organization_auth(_organization_id: &RawStr) -> Result<Json<Value>, ApiError> {
    Err(ApiError::NotImplemented)
}