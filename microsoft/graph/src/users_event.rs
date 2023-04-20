use crate::Client;
use crate::ClientResult;

pub struct UsersEvent {
    pub client: Client,
}

impl UsersEvent {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersEvent { client }
    }

    /**
    * Get calendarView from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView` endpoint.
    *
    * The calendar view for the calendar. Read-only. Nullable.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-list-calendarview?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `start_date_time: &str` -- The start date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T19:00:00-08:00.
    * * `end_date_time: &str` -- The end date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T20:00:00-08:00.
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_list_calendar_view(
        &self,
        user_id: &str,
        start_date_time: &str,
        end_date_time: &str,
        top: u64,
        skip: u64,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !end_date_time.is_empty() {
            query_args.push(("endDateTime".to_string(), end_date_time.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !start_date_time.is_empty() {
            query_args.push(("startDateTime".to_string(), start_date_time.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get calendarView from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}` endpoint.
    *
    * The calendar view for the calendar. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `start_date_time: &str` -- The start date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T19:00:00-08:00.
    * * `end_date_time: &str` -- The end date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T20:00:00-08:00.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_get_calendar_view(
        &self,
        user_id: &str,
        event_id: &str,
        start_date_time: &str,
        end_date_time: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphEventAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !end_date_time.is_empty() {
            query_args.push(("endDateTime".to_string(), end_date_time.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !start_date_time.is_empty() {
            query_args.push(("startDateTime".to_string(), start_date_time.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * List attachments.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/attachments` endpoint.
    *
    * Retrieve a list of attachment objects attached to an event.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-list-attachments?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_calendar_view_list_attachment(
        &self,
        user_id: &str,
        event_id: &str,
        top: u64,
        skip: u64,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/attachments?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Add attachment.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/attachments` endpoint.
    *
    * Use this API to add an attachment to an existing event. This operation limits the size of the attachment you can add to under 3 MB. If an organizer adds an attachment to a meeting event, the organizer can subsequently update the event to send the attachment and update the event for each attendee as well.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-post-attachments?view=graph-rest-1.0>
    */
    pub async fn users_calendar_view_create_attachments(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphAttachmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/attachments",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get attachments from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/attachments/{attachment-id}` endpoint.
    *
    * The collection of FileAttachment, ItemAttachment, and referenceAttachment attachments for the event. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_calendar_view_get_attachment(
        &self,
        user_id: &str,
        event_id: &str,
        attachment_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/attachments/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&attachment_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property attachments for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/calendarView/{event-id}/attachments/{attachment-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_delete_attachments(
        &self,
        user_id: &str,
        event_id: &str,
        attachment_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/attachments/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&attachment_id.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/attachments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_attachments_get_count_ed_80(
        &self,
        user_id: &str,
        event_id: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/attachments/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get calendar from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/calendar` endpoint.
    *
    * The calendar that contains the event. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn users_calendar_view_get(
        &self,
        user_id: &str,
        event_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/calendar?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get extensions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/extensions` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_calendar_view_list_extension(
        &self,
        user_id: &str,
        event_id: &str,
        top: u64,
        skip: u64,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/extensions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create open extension.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/extensions` endpoint.
    *
    * Create an open extension (openTypeExtension object) and add custom properties in a new or existing instance of a resource. You can create an open extension in a resource instance and store custom data to it all in the same operation, except for specific resources. See known limitations of open extensions for more information. The table in the Permissions section lists the resources that support open extensions.
    *
    * FROM: <https://docs.microsoft.com/graph/api/opentypeextension-post-opentypeextension?view=graph-rest-1.0>
    */
    pub async fn users_calendar_view_create_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/extensions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get extensions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_calendar_view_get_extension(
        &self,
        user_id: &str,
        event_id: &str,
        extension_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/extensions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property extensions for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/calendarView/{event-id}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_delete_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Update the navigation property extensions in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/calendarView/{event-id}/extensions/{extension-id}` endpoint.
    */
    pub async fn users_calendar_view_update_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_extensions_get_count_df_44(
        &self,
        user_id: &str,
        event_id: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/extensions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get instances from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/instances` endpoint.
    *
    * The occurrences of a recurring series, if the event is a series master. This property includes occurrences that are part of the recurrence pattern, and exceptions that have been modified, but does not include occurrences that have been cancelled from the series. Navigation property. Read-only. Nullable.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-list-instances?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `start_date_time: &str` -- The start date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T19:00:00-08:00.
    * * `end_date_time: &str` -- The end date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T20:00:00-08:00.
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn users_calendar_view_list_instance(
        &self,
        user_id: &str,
        event_id: &str,
        start_date_time: &str,
        end_date_time: &str,
        top: u64,
        skip: u64,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !end_date_time.is_empty() {
            query_args.push(("endDateTime".to_string(), end_date_time.to_string()));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !start_date_time.is_empty() {
            query_args.push(("startDateTime".to_string(), start_date_time.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get instances from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}` endpoint.
    *
    * The occurrences of a recurring series, if the event is a series master. This property includes occurrences that are part of the recurrence pattern, and exceptions that have been modified, but does not include occurrences that have been cancelled from the series. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `start_date_time: &str` -- The start date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T19:00:00-08:00.
    * * `end_date_time: &str` -- The end date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T20:00:00-08:00.
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn users_calendar_view_get_instance(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        start_date_time: &str,
        end_date_time: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphEventAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !end_date_time.is_empty() {
            query_args.push(("endDateTime".to_string(), end_date_time.to_string()));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !start_date_time.is_empty() {
            query_args.push(("startDateTime".to_string(), start_date_time.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * List attachments.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/attachments` endpoint.
    *
    * Retrieve a list of attachment objects attached to an event.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-list-attachments?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_calendar_view_instances_list_attachment(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        top: u64,
        skip: u64,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/attachments?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Add attachment.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/attachments` endpoint.
    *
    * Use this API to add an attachment to an existing event. This operation limits the size of the attachment you can add to under 3 MB. If an organizer adds an attachment to a meeting event, the organizer can subsequently update the event to send the attachment and update the event for each attendee as well.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-post-attachments?view=graph-rest-1.0>
    */
    pub async fn users_calendar_view_instances_create_attachments(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphAttachmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/attachments",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get attachments from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/attachments/{attachment-id}` endpoint.
    *
    * The collection of FileAttachment, ItemAttachment, and referenceAttachment attachments for the event. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_calendar_view_instances_get_attachment(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        attachment_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/attachments/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(&attachment_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property attachments for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/attachments/{attachment-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_instances_delete_attachments(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        attachment_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/attachments/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(&attachment_id.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/attachments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_instances_attachments_get_count_d_5d_8(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/attachments/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get calendar from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/calendar` endpoint.
    *
    * The calendar that contains the event. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn users_calendar_view_instances_get(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/calendar?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get extensions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/extensions` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_calendar_view_instances_list_extension(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        top: u64,
        skip: u64,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/extensions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create open extension.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/extensions` endpoint.
    *
    * Create an open extension (openTypeExtension object) and add custom properties in a new or existing instance of a resource. You can create an open extension in a resource instance and store custom data to it all in the same operation, except for specific resources. See known limitations of open extensions for more information. The table in the Permissions section lists the resources that support open extensions.
    *
    * FROM: <https://docs.microsoft.com/graph/api/opentypeextension-post-opentypeextension?view=graph-rest-1.0>
    */
    pub async fn users_calendar_view_instances_create_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/extensions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get extensions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_calendar_view_instances_get_extension(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/extensions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property extensions for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_instances_delete_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Update the navigation property extensions in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    */
    pub async fn users_calendar_view_instances_update_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_instances_extensions_get_count_7713(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/extensions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get multiValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_calendar_view_instances_list_multi_value_extended_propertie(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        top: u64,
        skip: u64,
        search: &str,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !search.is_empty() {
            query_args.push(("$search".to_string(), search.to_string()));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/multiValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create new navigation property to multiValueExtendedProperties for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties` endpoint.
    */
    pub async fn users_calendar_view_instances_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/multiValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get multiValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_calendar_view_instances_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/multiValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(
                    &multi_value_legacy_extended_property_id.to_string()
                ),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property multiValueExtendedProperties for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_instances_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(
                    &multi_value_legacy_extended_property_id.to_string()
                ),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Update the navigation property multiValueExtendedProperties in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn users_calendar_view_instances_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(
                    &multi_value_legacy_extended_property_id.to_string()
                ),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_instances_multi_value_extended_properties_get_count_7226(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        search: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !search.is_empty() {
            query_args.push(("$search".to_string(), search.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/multiValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get singleValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_calendar_view_instances_list_single_value_extended_propertie(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        top: u64,
        skip: u64,
        search: &str,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !search.is_empty() {
            query_args.push(("$search".to_string(), search.to_string()));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/singleValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create new navigation property to singleValueExtendedProperties for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties` endpoint.
    */
    pub async fn users_calendar_view_instances_create_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/singleValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get singleValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_calendar_view_instances_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/singleValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(
                    &single_value_legacy_extended_property_id.to_string()
                ),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property singleValueExtendedProperties for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_instances_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(
                    &single_value_legacy_extended_property_id.to_string()
                ),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Update the navigation property singleValueExtendedProperties in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn users_calendar_view_instances_update_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(
                    &single_value_legacy_extended_property_id.to_string()
                ),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_instances_single_value_extended_properties_get_count_15_4c(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        search: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !search.is_empty() {
            query_args.push(("$search".to_string(), search.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/singleValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/instances/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_instances_get_count_aa_5e(
        &self,
        user_id: &str,
        event_id: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get multiValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/multiValueExtendedProperties` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_calendar_view_list_multi_value_extended_propertie(
        &self,
        user_id: &str,
        event_id: &str,
        top: u64,
        skip: u64,
        search: &str,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !search.is_empty() {
            query_args.push(("$search".to_string(), search.to_string()));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/multiValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create new navigation property to multiValueExtendedProperties for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/multiValueExtendedProperties` endpoint.
    */
    pub async fn users_calendar_view_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/multiValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get multiValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_calendar_view_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/multiValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(
                    &multi_value_legacy_extended_property_id.to_string()
                ),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property multiValueExtendedProperties for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/calendarView/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(
                    &multi_value_legacy_extended_property_id.to_string()
                ),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Update the navigation property multiValueExtendedProperties in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/calendarView/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn users_calendar_view_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(
                    &multi_value_legacy_extended_property_id.to_string()
                ),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_multi_value_extended_properties_get_count_1aec(
        &self,
        user_id: &str,
        event_id: &str,
        search: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !search.is_empty() {
            query_args.push(("$search".to_string(), search.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/multiValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get singleValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/singleValueExtendedProperties` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_calendar_view_list_single_value_extended_propertie(
        &self,
        user_id: &str,
        event_id: &str,
        top: u64,
        skip: u64,
        search: &str,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !search.is_empty() {
            query_args.push(("$search".to_string(), search.to_string()));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/singleValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create new navigation property to singleValueExtendedProperties for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/singleValueExtendedProperties` endpoint.
    */
    pub async fn users_calendar_view_create_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/singleValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get singleValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_calendar_view_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/singleValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(
                    &single_value_legacy_extended_property_id.to_string()
                ),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property singleValueExtendedProperties for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/calendarView/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(
                    &single_value_legacy_extended_property_id.to_string()
                ),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Update the navigation property singleValueExtendedProperties in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/calendarView/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn users_calendar_view_update_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(
                    &single_value_legacy_extended_property_id.to_string()
                ),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/{event-id}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_single_value_extended_properties_get_count_7_8d_8(
        &self,
        user_id: &str,
        event_id: &str,
        search: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !search.is_empty() {
            query_args.push(("$search".to_string(), search.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/singleValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarView/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_calendar_view_get_count_9543(
        &self,
        user_id: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * List events.
    *
    * This function performs a `GET` to the `/users/{user-id}/events` endpoint.
    *
    * Get a list of event objects in the user's mailbox. The list contains single
    * instance meetings and series masters. To get expanded event instances, you can get the calendar view, or
    * get the instances of an event. Currently, this operation returns event bodies in only HTML format. There are two scenarios where an app can get events in another user's calendar:
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-list-events?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_list_event(
        &self,
        user_id: &str,
        top: u64,
        skip: u64,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create Event.
    *
    * This function performs a `POST` to the `/users/{user-id}/events` endpoint.
    *
    * Create an event in the user's default calendar or specified calendar. By default, the **allowNewTimeProposals** property is set to true when an event is created, which means invitees can propose a different date/time for the event. See Propose new meeting times for more information on how to propose a time, and how to receive and accept a new time proposal. You can specify the time zone for each of the start and end times of the event as part of their values, because the
    * **start** and **end** properties are of dateTimeTimeZone type. First find the supported time zones to make sure you set only time zones that have been configured for the user's mailbox server.  When an event is sent, the server sends invitations to all the attendees. **Setting the location in an event** An Exchange administrator can set up a mailbox and an email address for a resource such as a meeting room, or equipment
    * like a projector. Users can then invite the resource as an attendee to a meeting. On behalf of the resource, the server accepts or rejects
    * the meeting request based on the free/busy schedule of the resource.
    * If the server accepts a meeting for the resource, it creates an event for the meeting in the resource's calendar. If the meeting is rescheduled,
    * the server automatically updates the event in the resource's calendar. Another advantage of setting up a mailbox for a resource is to control scheduling of the resource, for example, only executives
    * or their delegates can book a private meeting room. If you're organizing an event that involves a meeting location: Additionally, if the meeting location has been set up as a resource, or if the event involves some equipment that has been set up as a resource:
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-post-events?view=graph-rest-1.0>
    */
    pub async fn users_create_events(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphEventAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphEventAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/events",
                crate::progenitor_support::encode_path(&user_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get events from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}` endpoint.
    *
    * The user's events. Default is to show Events under the Default Calendar. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_get_event(
        &self,
        user_id: &str,
        event_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphEventAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property events for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/events/{event-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_delete_events(
        &self,
        user_id: &str,
        event_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Update the navigation property events in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/events/{event-id}` endpoint.
    */
    pub async fn users_update_events(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphEventAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphEventAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * List attachments.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/attachments` endpoint.
    *
    * Retrieve a list of attachment objects attached to an event.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-list-attachments?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_list_attachment(
        &self,
        user_id: &str,
        event_id: &str,
        top: u64,
        skip: u64,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/attachments?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Add attachment.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/attachments` endpoint.
    *
    * Use this API to add an attachment to an existing event. This operation limits the size of the attachment you can add to under 3 MB. If an organizer adds an attachment to a meeting event, the organizer can subsequently update the event to send the attachment and update the event for each attendee as well.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-post-attachments?view=graph-rest-1.0>
    */
    pub async fn s_create_attachments(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphAttachmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/attachments",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get attachments from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/attachments/{attachment-id}` endpoint.
    *
    * The collection of FileAttachment, ItemAttachment, and referenceAttachment attachments for the event. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_attachment(
        &self,
        user_id: &str,
        event_id: &str,
        attachment_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/attachments/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&attachment_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property attachments for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/events/{event-id}/attachments/{attachment-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_attachments(
        &self,
        user_id: &str,
        event_id: &str,
        attachment_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/attachments/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&attachment_id.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/attachments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_attachments_get_count_71_1f(
        &self,
        user_id: &str,
        event_id: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/attachments/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get calendar from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/calendar` endpoint.
    *
    * The calendar that contains the event. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_get_calendar(
        &self,
        user_id: &str,
        event_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/calendar?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get extensions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/extensions` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_list_extension(
        &self,
        user_id: &str,
        event_id: &str,
        top: u64,
        skip: u64,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/extensions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create open extension.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/extensions` endpoint.
    *
    * Create an open extension (openTypeExtension object) and add custom properties in a new or existing instance of a resource. You can create an open extension in a resource instance and store custom data to it all in the same operation, except for specific resources. See known limitations of open extensions for more information. The table in the Permissions section lists the resources that support open extensions.
    *
    * FROM: <https://docs.microsoft.com/graph/api/opentypeextension-post-opentypeextension?view=graph-rest-1.0>
    */
    pub async fn s_create_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/extensions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get extensions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_extension(
        &self,
        user_id: &str,
        event_id: &str,
        extension_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/extensions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property extensions for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/events/{event-id}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Update the navigation property extensions in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/events/{event-id}/extensions/{extension-id}` endpoint.
    */
    pub async fn s_update_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_extensions_get_count_0041(
        &self,
        user_id: &str,
        event_id: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/extensions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get instances from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/instances` endpoint.
    *
    * The occurrences of a recurring series, if the event is a series master. This property includes occurrences that are part of the recurrence pattern, and exceptions that have been modified, but does not include occurrences that have been cancelled from the series. Navigation property. Read-only. Nullable.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-list-instances?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `start_date_time: &str` -- The start date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T19:00:00-08:00.
    * * `end_date_time: &str` -- The end date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T20:00:00-08:00.
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_list_instance(
        &self,
        user_id: &str,
        event_id: &str,
        start_date_time: &str,
        end_date_time: &str,
        top: u64,
        skip: u64,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !end_date_time.is_empty() {
            query_args.push(("endDateTime".to_string(), end_date_time.to_string()));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !start_date_time.is_empty() {
            query_args.push(("startDateTime".to_string(), start_date_time.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get instances from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}` endpoint.
    *
    * The occurrences of a recurring series, if the event is a series master. This property includes occurrences that are part of the recurrence pattern, and exceptions that have been modified, but does not include occurrences that have been cancelled from the series. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `start_date_time: &str` -- The start date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T19:00:00-08:00.
    * * `end_date_time: &str` -- The end date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T20:00:00-08:00.
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_get_instance(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        start_date_time: &str,
        end_date_time: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphEventAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !end_date_time.is_empty() {
            query_args.push(("endDateTime".to_string(), end_date_time.to_string()));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !start_date_time.is_empty() {
            query_args.push(("startDateTime".to_string(), start_date_time.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * List attachments.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/attachments` endpoint.
    *
    * Retrieve a list of attachment objects attached to an event.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-list-attachments?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_instances_list_attachment(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        top: u64,
        skip: u64,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/attachments?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Add attachment.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/attachments` endpoint.
    *
    * Use this API to add an attachment to an existing event. This operation limits the size of the attachment you can add to under 3 MB. If an organizer adds an attachment to a meeting event, the organizer can subsequently update the event to send the attachment and update the event for each attendee as well.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-post-attachments?view=graph-rest-1.0>
    */
    pub async fn s_instances_create_attachments(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphAttachmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/attachments",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get attachments from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/attachments/{attachment-id}` endpoint.
    *
    * The collection of FileAttachment, ItemAttachment, and referenceAttachment attachments for the event. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_instances_get_attachment(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        attachment_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/attachments/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(&attachment_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property attachments for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/attachments/{attachment-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_instances_delete_attachments(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        attachment_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/attachments/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(&attachment_id.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/attachments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_instances_attachments_get_count_3c_02(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/attachments/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get calendar from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/calendar` endpoint.
    *
    * The calendar that contains the event. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_instances_get_calendar(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/calendar?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get extensions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/extensions` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_instances_list_extension(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        top: u64,
        skip: u64,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/extensions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create open extension.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/extensions` endpoint.
    *
    * Create an open extension (openTypeExtension object) and add custom properties in a new or existing instance of a resource. You can create an open extension in a resource instance and store custom data to it all in the same operation, except for specific resources. See known limitations of open extensions for more information. The table in the Permissions section lists the resources that support open extensions.
    *
    * FROM: <https://docs.microsoft.com/graph/api/opentypeextension-post-opentypeextension?view=graph-rest-1.0>
    */
    pub async fn s_instances_create_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/extensions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get extensions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_instances_get_extension(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/extensions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property extensions for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_instances_delete_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Update the navigation property extensions in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    */
    pub async fn s_instances_update_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_instances_extensions_get_count_8eed(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/extensions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get multiValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_instances_list_multi_value_extended_propertie(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        top: u64,
        skip: u64,
        search: &str,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !search.is_empty() {
            query_args.push(("$search".to_string(), search.to_string()));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/multiValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create new navigation property to multiValueExtendedProperties for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties` endpoint.
    */
    pub async fn s_instances_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/multiValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get multiValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_instances_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/multiValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(
                    &multi_value_legacy_extended_property_id.to_string()
                ),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property multiValueExtendedProperties for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_instances_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(
                    &multi_value_legacy_extended_property_id.to_string()
                ),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Update the navigation property multiValueExtendedProperties in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_instances_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(
                    &multi_value_legacy_extended_property_id.to_string()
                ),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_instances_multi_value_extended_properties_get_count_c_257(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        search: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !search.is_empty() {
            query_args.push(("$search".to_string(), search.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/multiValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get singleValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_instances_list_single_value_extended_propertie(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        top: u64,
        skip: u64,
        search: &str,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !search.is_empty() {
            query_args.push(("$search".to_string(), search.to_string()));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/singleValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create new navigation property to singleValueExtendedProperties for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties` endpoint.
    */
    pub async fn s_instances_create_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/singleValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get singleValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_instances_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/singleValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(
                    &single_value_legacy_extended_property_id.to_string()
                ),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property singleValueExtendedProperties for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_instances_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(
                    &single_value_legacy_extended_property_id.to_string()
                ),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Update the navigation property singleValueExtendedProperties in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_instances_update_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                crate::progenitor_support::encode_path(
                    &single_value_legacy_extended_property_id.to_string()
                ),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_instances_single_value_extended_properties_get_count_f_929(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        search: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !search.is_empty() {
            query_args.push(("$search".to_string(), search.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/singleValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(&event_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/instances/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_instances_get_count_f_922(
        &self,
        user_id: &str,
        event_id: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get multiValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/multiValueExtendedProperties` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_list_multi_value_extended_propertie(
        &self,
        user_id: &str,
        event_id: &str,
        top: u64,
        skip: u64,
        search: &str,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !search.is_empty() {
            query_args.push(("$search".to_string(), search.to_string()));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/multiValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create new navigation property to multiValueExtendedProperties for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/multiValueExtendedProperties` endpoint.
    */
    pub async fn s_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/multiValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get multiValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/multiValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(
                    &multi_value_legacy_extended_property_id.to_string()
                ),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property multiValueExtendedProperties for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/events/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(
                    &multi_value_legacy_extended_property_id.to_string()
                ),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Update the navigation property multiValueExtendedProperties in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/events/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(
                    &multi_value_legacy_extended_property_id.to_string()
                ),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_multi_value_extended_properties_get_count_ffcc(
        &self,
        user_id: &str,
        event_id: &str,
        search: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !search.is_empty() {
            query_args.push(("$search".to_string(), search.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/multiValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get singleValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/singleValueExtendedProperties` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_list_single_value_extended_propertie(
        &self,
        user_id: &str,
        event_id: &str,
        top: u64,
        skip: u64,
        search: &str,
        filter: &str,
        count: bool,
        orderby: &[String],
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::Me> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count {
            query_args.push(("$count".to_string(), count.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !orderby.is_empty() {
            query_args.push(("$orderby".to_string(), orderby.join(" ")));
        }
        if !search.is_empty() {
            query_args.push(("$search".to_string(), search.to_string()));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        if !skip.to_string().is_empty() {
            query_args.push(("$skip".to_string(), skip.to_string()));
        }
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/singleValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create new navigation property to singleValueExtendedProperties for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/singleValueExtendedProperties` endpoint.
    */
    pub async fn s_create_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/singleValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get singleValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/singleValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(
                    &single_value_legacy_extended_property_id.to_string()
                ),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property singleValueExtendedProperties for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/events/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(
                    &single_value_legacy_extended_property_id.to_string()
                ),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Update the navigation property singleValueExtendedProperties in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/events/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_update_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                crate::progenitor_support::encode_path(
                    &single_value_legacy_extended_property_id.to_string()
                ),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/{event-id}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_single_value_extended_properties_get_count_fe_0f(
        &self,
        user_id: &str,
        event_id: &str,
        search: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        if !search.is_empty() {
            query_args.push(("$search".to_string(), search.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/singleValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/events/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_get_count_d_443(&self, user_id: &str, filter: &str) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/events/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
