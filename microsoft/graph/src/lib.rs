//! A fully generated, opinionated API client library for Microsoft Graph API.
//!
//! [![docs.rs](https://docs.rs/microsoft-graph-api/badge.svg)](https://docs.rs/microsoft-graph-api)
//!
//! ## API Details
//!
//! This OData service is located at https://graph.microsoft.com/v1.0
//!
//!
//!
//!
//!
//!
//! ## Client Details
//!
//! This client is generated from the [Microsoft Graph API OpenAPI
//! specs](https://raw.githubusercontent.com/microsoftgraph/msgraph-metadata/master/openapi/v1.0/openapi.yaml) based on API spec version `v1.0`. This way it will remain
//! up to date as features are added. The documentation for the crate is generated
//! along with the code to make this library easy to use.
//!
//!
//! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! microsoft-graph-api = "0.1.0"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```
//! use microsoft_graph_api::Client;
//!
//! let microsoft_graph_api = Client::new(
//!     String::from("api-key"),
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `MICROSOFT_GRAPH_API_API_KEY`
//!
//! And then you can create a client from the environment.
//!
//! ```
//! use microsoft_graph_api::Client;
//!
//! let microsoft_graph_api = Client::new_from_env();
//! ```
//!
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::nonstandard_macro_braces)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod types;
pub mod users_actions;
pub mod users_agreement_acceptance;
pub mod users_app_role_assignment;
pub mod users_authentication;
pub mod users_calendar;
pub mod users_calendar_group;
pub mod users_chat;
pub mod users_contact;
pub mod users_contact_folder;
pub mod users_device_management_troubleshooting_event;
pub mod users_directory_object;
pub mod users_drive;
pub mod users_event;
pub mod users_extension;
pub mod users_functions;
pub mod users_inference_classification;
pub mod users_license_details;
pub mod users_mail_folder;
pub mod users_mailbox_settings;
pub mod users_managed_app_registration;
pub mod users_managed_device;
pub mod users_message;
pub mod users_o_auth_2_permission_grant;
pub mod users_office_graph_insights;
pub mod users_onenote;
pub mod users_online_meeting;
pub mod users_outlook_user;
pub mod users_person;
pub mod users_planner_user;
pub mod users_presence;
pub mod users_profile_photo;
pub mod users_scoped_role_membership;
pub mod users_site;
pub mod users_team;
pub mod users_todo;
pub mod users_user;
pub mod users_user_activity;
pub mod users_user_settings;
pub mod users_user_teamwork;
#[doc(hidden)]
pub mod utils;

use thiserror::Error;
type ClientResult<T> = Result<T, ClientError>;

/// Errors returned by the client
#[derive(Debug, Error)]
pub enum ClientError {
    /// utf8 convertion error
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    /// URL Parsing Error
    #[error(transparent)]
    UrlParserError(#[from] url::ParseError),
    /// Serde JSON parsing error
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    /// Errors returned by reqwest
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// Errors returned by reqwest::header
    #[error(transparent)]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    /// Errors returned by reqwest middleware
    #[error(transparent)]
    ReqwestMiddleWareError(#[from] reqwest_middleware::Error),
    /// Generic HTTP Error
    #[error("HTTP Error. Code: {status}, message: {error}")]
    HttpError {
        status: http::StatusCode,
        error: String,
    },
}

pub const FALLBACK_HOST: &str = "https://graph.microsoft.com";

mod progenitor_support {
    use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

    const PATH_SET: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'#')
        .add(b'<')
        .add(b'>')
        .add(b'?')
        .add(b'`')
        .add(b'{')
        .add(b'}');

    #[allow(dead_code)]
    pub(crate) fn encode_path(pc: &str) -> String {
        utf8_percent_encode(pc, PATH_SET).to_string()
    }
}

#[derive(Debug, Default)]
pub(crate) struct Message {
    pub body: Option<reqwest::Body>,
    pub content_type: Option<String>,
}

use std::env;

#[derive(Debug, Default, Clone)]
pub struct RootDefaultServer {}

impl RootDefaultServer {
    pub fn default_url(&self) -> &str {
        "https://graph.microsoft.com/v1.0"
    }
}

/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {
    host: String,
    host_override: Option<String>,
    token: String,

    client: reqwest_middleware::ClientWithMiddleware,
}

impl Client {
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    pub fn new<T>(token: T) -> Self
    where
        T: ToString,
    {
        let client = reqwest::Client::builder().build();
        let retry_policy =
            reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
        match client {
            Ok(c) => {
                let client = reqwest_middleware::ClientBuilder::new(c)
                    // Trace HTTP requests. See the tracing crate to make use of these traces.
                    .with(reqwest_tracing::TracingMiddleware::default())
                    // Retry failed requests.
                    .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                        reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                        |req: &reqwest::Request| req.try_clone().is_some(),
                    ))
                    .build();

                let host = RootDefaultServer::default().default_url().to_string();

                Client {
                    host,
                    host_override: None,
                    token: token.to_string(),

                    client,
                }
            }
            Err(e) => panic!("creating reqwest client failed: {:?}", e),
        }
    }

    /// Override the host for all endpoins in the client.
    pub fn with_host_override<H>(&mut self, host: H) -> &mut Self
    where
        H: ToString,
    {
        self.host_override = Some(host.to_string());
        self
    }

    /// Disables the global host override for the client.
    pub fn remove_host_override(&mut self) -> &mut Self {
        self.host_override = None;
        self
    }

    pub fn get_host_override(&self) -> Option<&str> {
        self.host_override.as_deref()
    }

    pub(crate) fn url(&self, path: &str, host: Option<&str>) -> String {
        format!(
            "{}{}",
            self.get_host_override()
                .or(host)
                .unwrap_or(self.host.as_str()),
            path
        )
    }

    /// Create a new Client struct from environment variables. It
    /// takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key and your requests will work.
    /// We pass in the token and refresh token to the client so if you are storing
    /// it in a database, you can get it first.
    pub fn new_from_env() -> Self {
        let token =
            env::var("MICROSOFT_GRAPH_API_API_KEY").expect("must set MICROSOFT_GRAPH_API_API_KEY");

        Client::new(token)
    }

    async fn url_and_auth(&self, uri: &str) -> ClientResult<(reqwest::Url, Option<String>)> {
        let parsed_url = uri.parse::<reqwest::Url>()?;
        let auth = format!("Bearer {}", self.token);
        Ok((parsed_url, Some(auth)))
    }

    async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        message: Message,
    ) -> ClientResult<reqwest::Response> {
        let (url, auth) = self.url_and_auth(uri).await?;
        let instance = <&Client>::clone(&self);
        let mut req = instance.client.request(method.clone(), url);
        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(content_type) = &message.content_type {
            req = req.header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_str(content_type).unwrap(),
            );
        } else {
            req = req.header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("application/json"),
            );
        }

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }
        if let Some(body) = message.body {
            req = req.body(body);
        }
        Ok(req.send().await?)
    }

    async fn request<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        message: Message,
    ) -> ClientResult<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let response = self.request_raw(method, uri, message).await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("Received successful response. Read payload.");
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")?
            } else {
                serde_json::from_slice::<Out>(&response_body)?
            };
            Ok(parsed_response)
        } else {
            let error = if response_body.is_empty() {
                ClientError::HttpError {
                    status: status,
                    error: "empty response".into(),
                }
            } else {
                ClientError::HttpError {
                    status: status,
                    error: String::from_utf8_lossy(&response_body).into(),
                }
            };

            Err(error)
        }
    }

    async fn request_with_links<Out>(
        &self,
        method: http::Method,
        uri: &str,
        message: Message,
    ) -> ClientResult<(Option<crate::utils::NextLink>, Out)>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let response = self.request_raw(method, uri, message).await?;

        let status = response.status();
        let link = response
            .headers()
            .get(http::header::LINK)
            .and_then(|l| l.to_str().ok())
            .and_then(|l| parse_link_header::parse(l).ok())
            .as_ref()
            .and_then(crate::utils::next_link);

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("Received successful response. Read payload.");

            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")?
            } else {
                serde_json::from_slice::<Out>(&response_body)?
            };
            Ok((link, parsed_response))
        } else {
            let error = if response_body.is_empty() {
                ClientError::HttpError {
                    status: status,
                    error: "empty response".into(),
                }
            } else {
                ClientError::HttpError {
                    status: status,
                    error: String::from_utf8_lossy(&response_body).into(),
                }
            };
            Err(error)
        }
    }

    /* TODO: make this more DRY */
    #[allow(dead_code)]
    async fn post_form<Out>(&self, uri: &str, form: reqwest::multipart::Form) -> ClientResult<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let (url, auth) = self.url_and_auth(uri).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(http::Method::POST, url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        req = req.multipart(form);

        let response = req.send().await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("Received successful response. Read payload.");
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")?
            } else if std::any::TypeId::of::<Out>() == std::any::TypeId::of::<String>() {
                // Parse the output as a string.
                let s = String::from_utf8(response_body.to_vec())?;
                serde_json::from_value(serde_json::json!(&s))?
            } else {
                serde_json::from_slice::<Out>(&response_body)?
            };
            Ok(parsed_response)
        } else {
            let error = if response_body.is_empty() {
                ClientError::HttpError {
                    status: status,
                    error: "empty response".into(),
                }
            } else {
                ClientError::HttpError {
                    status: status,
                    error: String::from_utf8_lossy(&response_body).into(),
                }
            };

            Err(error)
        }
    }

    /* TODO: make this more DRY */
    #[allow(dead_code)]
    async fn request_with_accept_mime<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        accept_mime_type: &str,
    ) -> ClientResult<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let (url, auth) = self.url_and_auth(uri).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(method, url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_str(accept_mime_type)?,
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        let response = req.send().await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("Received successful response. Read payload.");
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")?
            } else if std::any::TypeId::of::<Out>() == std::any::TypeId::of::<String>() {
                // Parse the output as a string.
                let s = String::from_utf8(response_body.to_vec())?;
                serde_json::from_value(serde_json::json!(&s))?
            } else {
                serde_json::from_slice::<Out>(&response_body)?
            };
            Ok(parsed_response)
        } else {
            let error = if response_body.is_empty() {
                ClientError::HttpError {
                    status: status,
                    error: "empty response".into(),
                }
            } else {
                ClientError::HttpError {
                    status: status,
                    error: String::from_utf8_lossy(&response_body).into(),
                }
            };

            Err(error)
        }
    }

    /* TODO: make this more DRY */
    #[allow(dead_code)]
    async fn request_with_mime<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        content: &[u8],
        mime_type: &str,
    ) -> ClientResult<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let (url, auth) = self.url_and_auth(uri).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(method, url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_bytes(mime_type.as_bytes()).unwrap(),
        );
        // We are likely uploading a file so add the right headers.
        req = req.header(
            reqwest::header::HeaderName::from_static("x-upload-content-type"),
            reqwest::header::HeaderValue::from_static("application/octet-stream"),
        );
        req = req.header(
            reqwest::header::HeaderName::from_static("x-upload-content-length"),
            reqwest::header::HeaderValue::from_bytes(format!("{}", content.len()).as_bytes())
                .unwrap(),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        if content.len() > 1 {
            let b = bytes::Bytes::copy_from_slice(content);
            // We are uploading a file so add that as the body.
            req = req.body(b);
        }

        let response = req.send().await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("Received successful response. Read payload.");
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")?
            } else {
                serde_json::from_slice::<Out>(&response_body)?
            };
            Ok(parsed_response)
        } else {
            let error = if response_body.is_empty() {
                ClientError::HttpError {
                    status: status,
                    error: "empty response".into(),
                }
            } else {
                ClientError::HttpError {
                    status: status,
                    error: String::from_utf8_lossy(&response_body).into(),
                }
            };

            Err(error)
        }
    }

    async fn request_entity<D>(
        &self,
        method: http::Method,
        uri: &str,
        message: Message,
    ) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let r = self.request(method, uri, message).await?;
        Ok(r)
    }

    #[allow(dead_code)]
    async fn get<D>(&self, uri: &str, message: Message) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::GET, uri, message).await
    }

    #[allow(dead_code)]
    async fn get_all_pages<D>(&self, uri: &str, _message: Message) -> ClientResult<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        // TODO: implement this.
        self.unfold(uri).await
    }

    /// "unfold" paginated results of a vector of items
    #[allow(dead_code)]
    async fn unfold<D>(&self, uri: &str) -> ClientResult<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let mut global_items = Vec::new();
        let (new_link, mut items) = self.get_pages(uri).await?;
        let mut link = new_link;
        while !items.is_empty() {
            global_items.append(&mut items);
            // We need to get the next link.
            if let Some(url) = link.as_ref() {
                let url = reqwest::Url::parse(&url.0)?;
                let (new_link, new_items) = self.get_pages_url(&url).await?;
                link = new_link;
                items = new_items;
            }
        }

        Ok(global_items)
    }

    #[allow(dead_code)]
    async fn get_pages<D>(
        &self,
        uri: &str,
    ) -> ClientResult<(Option<crate::utils::NextLink>, Vec<D>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_with_links(http::Method::GET, uri, Message::default())
            .await
    }

    #[allow(dead_code)]
    async fn get_pages_url<D>(
        &self,
        url: &reqwest::Url,
    ) -> ClientResult<(Option<crate::utils::NextLink>, Vec<D>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_with_links(http::Method::GET, url.as_str(), Message::default())
            .await
    }

    #[allow(dead_code)]
    async fn post<D>(&self, uri: &str, message: Message) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::POST, uri, message).await
    }

    #[allow(dead_code)]
    async fn patch<D>(&self, uri: &str, message: Message) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PATCH, uri, message).await
    }

    #[allow(dead_code)]
    async fn put<D>(&self, uri: &str, message: Message) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PUT, uri, message).await
    }

    #[allow(dead_code)]
    async fn delete<D>(&self, uri: &str, message: Message) -> ClientResult<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::DELETE, uri, message)
            .await
    }

    /// Return a reference to an interface that provides access to users.user operations.
    pub fn users_user(&self) -> users_user::UsersUser {
        users_user::UsersUser::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.userActivity operations.
    pub fn users_user_activity(&self) -> users_user_activity::UsersUserActivity {
        users_user_activity::UsersUserActivity::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.Functions operations.
    pub fn users_functions(&self) -> users_functions::UsersFunctions {
        users_functions::UsersFunctions::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.agreementAcceptance operations.
    pub fn users_agreement_acceptance(
        &self,
    ) -> users_agreement_acceptance::UsersAgreementAcceptance {
        users_agreement_acceptance::UsersAgreementAcceptance::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.appRoleAssignment operations.
    pub fn users_app_role_assignment(&self) -> users_app_role_assignment::UsersAppRoleAssignment {
        users_app_role_assignment::UsersAppRoleAssignment::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.authentication operations.
    pub fn users_authentication(&self) -> users_authentication::UsersAuthentication {
        users_authentication::UsersAuthentication::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.Actions operations.
    pub fn users_actions(&self) -> users_actions::UsersActions {
        users_actions::UsersActions::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.calendar operations.
    pub fn users_calendar(&self) -> users_calendar::UsersCalendar {
        users_calendar::UsersCalendar::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.calendarGroup operations.
    pub fn users_calendar_group(&self) -> users_calendar_group::UsersCalendarGroup {
        users_calendar_group::UsersCalendarGroup::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.event operations.
    pub fn users_event(&self) -> users_event::UsersEvent {
        users_event::UsersEvent::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.chat operations.
    pub fn users_chat(&self) -> users_chat::UsersChat {
        users_chat::UsersChat::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.contactFolder operations.
    pub fn users_contact_folder(&self) -> users_contact_folder::UsersContactFolder {
        users_contact_folder::UsersContactFolder::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.contact operations.
    pub fn users_contact(&self) -> users_contact::UsersContact {
        users_contact::UsersContact::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.directoryObject operations.
    pub fn users_directory_object(&self) -> users_directory_object::UsersDirectoryObject {
        users_directory_object::UsersDirectoryObject::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.deviceManagementTroubleshootingEvent operations.
    pub fn users_device_management_troubleshooting_event(
        &self,
    ) -> users_device_management_troubleshooting_event::UsersDeviceManagementTroubleshootingEvent
    {
        users_device_management_troubleshooting_event::UsersDeviceManagementTroubleshootingEvent::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.drive operations.
    pub fn users_drive(&self) -> users_drive::UsersDrive {
        users_drive::UsersDrive::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.extension operations.
    pub fn users_extension(&self) -> users_extension::UsersExtension {
        users_extension::UsersExtension::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.site operations.
    pub fn users_site(&self) -> users_site::UsersSite {
        users_site::UsersSite::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.inferenceClassification operations.
    pub fn users_inference_classification(
        &self,
    ) -> users_inference_classification::UsersInferenceClassification {
        users_inference_classification::UsersInferenceClassification::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.officeGraphInsights operations.
    pub fn users_office_graph_insights(
        &self,
    ) -> users_office_graph_insights::UsersOfficeGraphInsights {
        users_office_graph_insights::UsersOfficeGraphInsights::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.team operations.
    pub fn users_team(&self) -> users_team::UsersTeam {
        users_team::UsersTeam::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.licenseDetails operations.
    pub fn users_license_details(&self) -> users_license_details::UsersLicenseDetails {
        users_license_details::UsersLicenseDetails::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.mailboxSettings operations.
    pub fn users_mailbox_settings(&self) -> users_mailbox_settings::UsersMailboxSettings {
        users_mailbox_settings::UsersMailboxSettings::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.mailFolder operations.
    pub fn users_mail_folder(&self) -> users_mail_folder::UsersMailFolder {
        users_mail_folder::UsersMailFolder::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.managedAppRegistration operations.
    pub fn users_managed_app_registration(
        &self,
    ) -> users_managed_app_registration::UsersManagedAppRegistration {
        users_managed_app_registration::UsersManagedAppRegistration::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.managedDevice operations.
    pub fn users_managed_device(&self) -> users_managed_device::UsersManagedDevice {
        users_managed_device::UsersManagedDevice::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.message operations.
    pub fn users_message(&self) -> users_message::UsersMessage {
        users_message::UsersMessage::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.oAuth2PermissionGrant operations.
    pub fn users_o_auth_2_permission_grant(
        &self,
    ) -> users_o_auth_2_permission_grant::UsersOAuth2PermissionGrant {
        users_o_auth_2_permission_grant::UsersOAuth2PermissionGrant::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.onenote operations.
    pub fn users_onenote(&self) -> users_onenote::UsersOnenote {
        users_onenote::UsersOnenote::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.onlineMeeting operations.
    pub fn users_online_meeting(&self) -> users_online_meeting::UsersOnlineMeeting {
        users_online_meeting::UsersOnlineMeeting::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.outlookUser operations.
    pub fn users_outlook_user(&self) -> users_outlook_user::UsersOutlookUser {
        users_outlook_user::UsersOutlookUser::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.person operations.
    pub fn users_person(&self) -> users_person::UsersPerson {
        users_person::UsersPerson::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.profilePhoto operations.
    pub fn users_profile_photo(&self) -> users_profile_photo::UsersProfilePhoto {
        users_profile_photo::UsersProfilePhoto::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.plannerUser operations.
    pub fn users_planner_user(&self) -> users_planner_user::UsersPlannerUser {
        users_planner_user::UsersPlannerUser::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.presence operations.
    pub fn users_presence(&self) -> users_presence::UsersPresence {
        users_presence::UsersPresence::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.scopedRoleMembership operations.
    pub fn users_scoped_role_membership(
        &self,
    ) -> users_scoped_role_membership::UsersScopedRoleMembership {
        users_scoped_role_membership::UsersScopedRoleMembership::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.userSettings operations.
    pub fn users_user_settings(&self) -> users_user_settings::UsersUserSettings {
        users_user_settings::UsersUserSettings::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.userTeamwork operations.
    pub fn users_user_teamwork(&self) -> users_user_teamwork::UsersUserTeamwork {
        users_user_teamwork::UsersUserTeamwork::new(self.clone())
    }

    /// Return a reference to an interface that provides access to users.todo operations.
    pub fn users_todo(&self) -> users_todo::UsersTodo {
        users_todo::UsersTodo::new(self.clone())
    }
}
