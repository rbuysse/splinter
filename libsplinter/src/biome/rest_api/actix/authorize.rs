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

use std::sync::Arc;

#[cfg(feature = "biome-credentials")]
use jsonwebtoken::{decode, Validation};

use crate::actix_web::{Error as ActixError, HttpRequest, HttpResponse};
#[cfg(feature = "biome-credentials")]
use crate::biome::rest_api::resources::authorize::AuthorizationResult;
use crate::biome::rest_api::BiomeRestConfig;
use crate::futures::{Future, IntoFuture};
#[cfg(feature = "auth")]
use crate::rest_api::auth::identity::Identity;
use crate::rest_api::secrets::SecretManager;
#[cfg(not(feature = "auth"))]
use crate::rest_api::sessions::default_validation;
use crate::rest_api::ErrorResponse;
#[cfg(feature = "biome-credentials")]
use crate::rest_api::{actix_web_1::get_authorization_token, sessions::Claims};

/// Verifies the user has the correct permissions
#[cfg(feature = "biome-credentials")]
pub(crate) fn authorize_user(
    request: &HttpRequest,
    secret_manager: &Arc<dyn SecretManager>,
    validation: &Validation,
) -> AuthorizationResult {
    let token = match get_authorization_token(&request) {
        Ok(token) => match token.splitn(2, ':').nth(1) {
            Some(token) => token.to_string(),
            None => {
                debug!("Invalid token; should be in the format 'Biome:<JWT>'");
                return AuthorizationResult::Unauthorized;
            }
        },
        Err(err) => {
            debug!("Failed to get token: {}", err);
            return AuthorizationResult::Unauthorized;
        }
    };

    validate_claims(&token, secret_manager, validation)
}

#[cfg(feature = "biome-credentials")]
pub(crate) fn validate_claims(
    token: &str,
    secret_manager: &Arc<dyn SecretManager>,
    validation: &Validation,
) -> AuthorizationResult {
    let secret = match secret_manager.secret() {
        Ok(secret) => secret,
        Err(err) => {
            debug!("Failed to fetch secret {}", err);
            return AuthorizationResult::Failed;
        }
    };

    match decode::<Claims>(&token, secret.as_ref(), validation) {
        Ok(claims) => AuthorizationResult::Authorized(claims.claims),
        Err(err) => {
            debug!("Invalid token: {}", err);
            AuthorizationResult::Unauthorized
        }
    }
}

type ErrorHttpResponse = Box<dyn Future<Item = HttpResponse, Error = ActixError>>;

#[cfg(all(not(feature = "auth"), not(feature = "biome-credentials")))]
pub(crate) fn get_authorized_user(
    request: &HttpRequest,
    secret_manager: &Arc<dyn SecretManager>,
    rest_config: &BiomeRestConfig,
) -> Result<String, ErrorHttpResponse> {
    /// Nothing is configured at compile-time, any route making use of this can't be authorized.
    Err(Box::new(
        HttpResponse::Unauthorized()
            .json(ErrorResponse::unauthorized())
            .into_future(),
    ))
}

#[cfg(all(not(feature = "auth"), feature = "biome-credentials"))]
pub(crate) fn get_authorized_user(
    request: &HttpRequest,
    secret_manager: &Arc<dyn SecretManager>,
    rest_config: &BiomeRestConfig,
) -> Result<String, ErrorHttpResponse> {
    let validation = default_validation(&rest_config.issuer());

    match authorize_user(&request, &*secret_manager, &validation) {
        AuthorizationResult::Authorized(claims) => Ok(claims.user_id()),
        AuthorizationResult::Unauthorized => Err(Box::new(
            HttpResponse::Unauthorized()
                .json(ErrorResponse::unauthorized())
                .into_future(),
        )),
        AuthorizationResult::Failed => Err(Box::new(
            HttpResponse::InternalServerError()
                .json(ErrorResponse::internal_error())
                .into_future(),
        )),
    }
}

#[cfg(feature = "auth")]
pub(crate) fn get_authorized_user(
    request: &HttpRequest,
    _secret_manager: &Arc<dyn SecretManager>,
    _rest_config: &BiomeRestConfig,
) -> Result<String, ErrorHttpResponse> {
    match request.extensions().get::<Identity>() {
        Some(Identity::User(id)) => Ok(id.into()),
        _ => Err(Box::new(
            HttpResponse::Unauthorized()
                .json(ErrorResponse::unauthorized())
                .into_future(),
        )),
    }
}
