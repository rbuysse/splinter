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

//! Builders for [OAuthClient](crate::oauth::OAuthClient) structs.

#[cfg(feature = "oauth-github")]
mod github;
#[cfg(feature = "oauth-openid")]
mod openid;

use crate::error::InvalidStateError;

use super::error::OAuthClientBuildError;
use super::{new_basic_client, store::InflightOAuthRequestStore, OAuthClient, SubjectProvider};

#[cfg(feature = "oauth-github")]
pub use github::GithubOAuthClientBuilder;

#[cfg(feature = "oauth-openid")]
pub use openid::OpenIdOAuthClientBuilder;

/// A builder for a new [`OAuthClient`].
///
/// This builder constructs an [`OAuthClient`] using the most general parameters. Configurations
/// that set values specific to certain providers may be available, depending on which features
/// have been enabled at compile time.
#[derive(Default)]
pub struct OAuthClientBuilder {
    client_id: Option<String>,
    client_secret: Option<String>,
    auth_url: Option<String>,
    redirect_url: Option<String>,
    token_url: Option<String>,
    extra_auth_params: Vec<(String, String)>,
    scopes: Vec<String>,
    subject_provider: Option<Box<dyn SubjectProvider>>,
    inflight_request_store: Option<Box<dyn InflightOAuthRequestStore>>,
}

impl OAuthClientBuilder {
    /// Constructs a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds an `OAuthClient`.
    ///
    /// # Errors
    ///
    /// Returns an [`OAuthClientBuildError`] if any of the auth, redirect, or token URLs are
    /// invalid.
    pub fn build(self) -> Result<OAuthClient, OAuthClientBuildError> {
        let client_id = self.client_id.ok_or_else(|| {
            InvalidStateError::with_message(
                "A client ID is required to successfully build an OAuthClient".into(),
            )
        })?;
        let client_secret = self.client_secret.ok_or_else(|| {
            InvalidStateError::with_message(
                "A client secret is required to successfully build an OAuthClient".into(),
            )
        })?;
        let auth_url = self.auth_url.ok_or_else(|| {
            InvalidStateError::with_message(
                "An auth URL is required to successfully build an OAuthClient".into(),
            )
        })?;
        let redirect_url = self.redirect_url.ok_or_else(|| {
            InvalidStateError::with_message(
                "A redirect URL is required to successfully build an OAuthClient".into(),
            )
        })?;
        let token_url = self.token_url.ok_or_else(|| {
            InvalidStateError::with_message(
                "A token URL is required to successfully build an OAuthClient".into(),
            )
        })?;
        let subject_provider = self.subject_provider.ok_or_else(|| {
            InvalidStateError::with_message(
                "A subject provider is required to successfully build an OAuthClient".into(),
            )
        })?;
        let inflight_request_store = self.inflight_request_store.ok_or_else(|| {
            InvalidStateError::with_message(
                "An in-flight request store is required to successfully build an OAuthClient"
                    .into(),
            )
        })?;
        OAuthClient::new(
            new_basic_client(client_id, client_secret, auth_url, redirect_url, token_url)?,
            self.extra_auth_params,
            self.scopes,
            subject_provider.clone(),
            inflight_request_store,
        )
        .map_err(OAuthClientBuildError::from)
    }

    /// Sets the client ID for the OAuth2 provider.
    pub fn with_client_id(mut self, client_id: String) -> Self {
        self.client_id = Some(client_id);
        self
    }

    /// Sets the client secret for the OAuth2 provider.
    pub fn with_client_secret(mut self, client_secret: String) -> Self {
        self.client_secret = Some(client_secret);
        self
    }

    /// Sets the authorize URL for the OAuth2 provider.
    pub fn with_auth_url(mut self, auth_url: String) -> Self {
        self.auth_url = Some(auth_url);
        self
    }

    /// Sets the redirect URL for the OAuth2 provider.
    pub fn with_redirect_url(mut self, redirect_url: String) -> Self {
        self.redirect_url = Some(redirect_url);
        self
    }

    /// Sets the token URL for the OAuth2 provider.
    pub fn with_token_url(mut self, token_url: String) -> Self {
        self.token_url = Some(token_url);
        self
    }

    /// Sets extra parameters that will be added to an authorization request
    pub fn with_extra_auth_params(mut self, extra_auth_params: Vec<(String, String)>) -> Self {
        let mut extra_auth_params = extra_auth_params;
        self.extra_auth_params.append(&mut extra_auth_params);
        self
    }

    /// Sets the scopes to request from the OAuth2 provider.
    pub fn with_scopes(mut self, scopes: Vec<String>) -> Self {
        let mut scopes = scopes;
        self.scopes.append(&mut scopes);
        self
    }

    /// Sets the subject provider to use to request the user's subject identifier.
    pub fn with_subject_provider(mut self, subject_provider: Box<dyn SubjectProvider>) -> Self {
        self.subject_provider = Some(subject_provider);
        self
    }

    /// Sets the in-flight request store in order to store values between requests to and from the
    /// OAuth2 provider.
    pub fn with_inflight_request_store(
        mut self,
        inflight_request_store: Box<dyn InflightOAuthRequestStore>,
    ) -> Self {
        self.inflight_request_store = Some(inflight_request_store);
        self
    }
}
