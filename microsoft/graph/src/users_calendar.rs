use crate::Client;
use crate::ClientResult;

pub struct UsersCalendar {
    pub client: Client,
}

impl UsersCalendar {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersCalendar { client }
    }

    /**
    * Get calendar.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendar` endpoint.
    *
    * Get the properties and relationships of a calendar object. The calendar can be one for a user,
    * or the default calendar of a Microsoft 365 group. There are two scenarios where an app can get another user's calendar:
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendar-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn users_get_calendar(
        &self,
        user_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendar?{}",
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
    * Update calendar.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/calendar` endpoint.
    *
    * Update the properties of a calendar object. The calendar can be one for a user,
    * or the default calendar of a Microsoft 365 group.
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendar-update?view=graph-rest-1.0>
    */
    pub async fn users_update_calendar(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphCalendarAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar",
                crate::progenitor_support::encode_path(&user_id.to_string()),
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
    * Get calendarPermissions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarPermissions` endpoint.
    *
    * The permissions of the users with whom the calendar is shared.
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn list_permission(
        &self,
        user_id: &str,
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
                "/users/{}/calendar/calendarPermissions?{}",
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
    * Create calendarPermission.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarPermissions` endpoint.
    *
    * Create a calendarPermission resource to specify the identity and role of the user with whom the specified calendar is being shared or delegated.
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendar-post-calendarpermissions?view=graph-rest-1.0>
    */
    pub async fn create_permissions(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphCalendarPermissionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarPermissionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarPermissions",
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
    * Get calendarPermissions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarPermissions/{calendarPermission-id}` endpoint.
    *
    * The permissions of the users with whom the calendar is shared.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn get_permission(
        &self,
        user_id: &str,
        calendar_permission_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarPermissionAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarPermissions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_permission_id.to_string()),
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
    * Delete navigation property calendarPermissions for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/calendarPermissions/{calendarPermission-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn delete_permissions(
        &self,
        user_id: &str,
        calendar_permission_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarPermissions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_permission_id.to_string()),
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
    * Update the navigation property calendarPermissions in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/calendar/calendarPermissions/{calendarPermission-id}` endpoint.
    */
    pub async fn update_permissions(
        &self,
        user_id: &str,
        calendar_permission_id: &str,
        body: &crate::types::MicrosoftGraphCalendarPermissionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarPermissionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarPermissions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_permission_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarPermissions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn permissions_get_count_b_877(
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
                "/users/{}/calendar/calendarPermissions/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView` endpoint.
    *
    * The calendar view for the calendar. Navigation property. Read-only.
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendar-list-calendarview?view=graph-rest-1.0>
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
    pub async fn list_view(
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
                "/users/{}/calendar/calendarView?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}` endpoint.
    *
    * The calendar view for the calendar. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `start_date_time: &str` -- The start date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T19:00:00-08:00.
    * * `end_date_time: &str` -- The end date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T20:00:00-08:00.
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn get_view(
        &self,
        user_id: &str,
        event_id: &str,
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
                "/users/{}/calendar/calendarView/{}?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/attachments` endpoint.
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
    pub async fn view_list_attachment(
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
                "/users/{}/calendar/calendarView/{}/attachments?{}",
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
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/attachments` endpoint.
    *
    * Use this API to add an attachment to an existing event. This operation limits the size of the attachment you can add to under 3 MB. If an organizer adds an attachment to a meeting event, the organizer can subsequently update the event to send the attachment and update the event for each attendee as well.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-post-attachments?view=graph-rest-1.0>
    */
    pub async fn view_create_attachments(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphAttachmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/attachments",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/attachments/{attachment-id}` endpoint.
    *
    * The collection of FileAttachment, ItemAttachment, and referenceAttachment attachments for the event. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn view_get_attachment(
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
                "/users/{}/calendar/calendarView/{}/attachments/{}?{}",
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/calendarView/{event-id}/attachments/{attachment-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_delete_attachments(
        &self,
        user_id: &str,
        event_id: &str,
        attachment_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/attachments/{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/attachments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_attachments_get_count_3470(
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
                "/users/{}/calendar/calendarView/{}/attachments/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/calendar` endpoint.
    *
    * The calendar that contains the event. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn view_get(
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
                "/users/{}/calendar/calendarView/{}/calendar?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/extensions` endpoint.
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
    pub async fn view_list_extension(
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
                "/users/{}/calendar/calendarView/{}/extensions?{}",
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
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/extensions` endpoint.
    *
    * Create an open extension (openTypeExtension object) and add custom properties in a new or existing instance of a resource. You can create an open extension in a resource instance and store custom data to it all in the same operation, except for specific resources. See known limitations of open extensions for more information. The table in the Permissions section lists the resources that support open extensions.
    *
    * FROM: <https://docs.microsoft.com/graph/api/opentypeextension-post-opentypeextension?view=graph-rest-1.0>
    */
    pub async fn view_create_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/extensions",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn view_get_extension(
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
                "/users/{}/calendar/calendarView/{}/extensions/{}?{}",
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/calendarView/{event-id}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_delete_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/extensions/{}",
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendar/calendarView/{event-id}/extensions/{extension-id}` endpoint.
    */
    pub async fn view_update_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/extensions/{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_extensions_get_count_3_2b_0(
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
                "/users/{}/calendar/calendarView/{}/extensions/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances` endpoint.
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
    pub async fn view_list_instance(
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
                "/users/{}/calendar/calendarView/{}/instances?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}` endpoint.
    *
    * The occurrences of a recurring series, if the event is a series master. This property includes occurrences that are part of the recurrence pattern, and exceptions that have been modified, but does not include occurrences that have been cancelled from the series. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `start_date_time: &str` -- The start date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T19:00:00-08:00.
    * * `end_date_time: &str` -- The end date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T20:00:00-08:00.
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn view_get_instance(
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
                "/users/{}/calendar/calendarView/{}/instances/{}?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/attachments` endpoint.
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
    pub async fn view_instances_list_attachment(
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
                "/users/{}/calendar/calendarView/{}/instances/{}/attachments?{}",
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
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/attachments` endpoint.
    *
    * Use this API to add an attachment to an existing event. This operation limits the size of the attachment you can add to under 3 MB. If an organizer adds an attachment to a meeting event, the organizer can subsequently update the event to send the attachment and update the event for each attendee as well.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-post-attachments?view=graph-rest-1.0>
    */
    pub async fn view_instances_create_attachments(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphAttachmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/attachments",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/attachments/{attachment-id}` endpoint.
    *
    * The collection of FileAttachment, ItemAttachment, and referenceAttachment attachments for the event. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn view_instances_get_attachment(
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
                "/users/{}/calendar/calendarView/{}/instances/{}/attachments/{}?{}",
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/attachments/{attachment-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_instances_delete_attachments(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        attachment_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/attachments/{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/attachments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_instances_attachments_get_count_051(
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
                "/users/{}/calendar/calendarView/{}/instances/{}/attachments/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/calendar` endpoint.
    *
    * The calendar that contains the event. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn view_instances_get(
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
                "/users/{}/calendar/calendarView/{}/instances/{}/calendar?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/extensions` endpoint.
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
    pub async fn view_instances_list_extension(
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
                "/users/{}/calendar/calendarView/{}/instances/{}/extensions?{}",
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
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/extensions` endpoint.
    *
    * Create an open extension (openTypeExtension object) and add custom properties in a new or existing instance of a resource. You can create an open extension in a resource instance and store custom data to it all in the same operation, except for specific resources. See known limitations of open extensions for more information. The table in the Permissions section lists the resources that support open extensions.
    *
    * FROM: <https://docs.microsoft.com/graph/api/opentypeextension-post-opentypeextension?view=graph-rest-1.0>
    */
    pub async fn view_instances_create_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/extensions",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn view_instances_get_extension(
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
                "/users/{}/calendar/calendarView/{}/instances/{}/extensions/{}?{}",
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_instances_delete_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/extensions/{}",
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    */
    pub async fn view_instances_update_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/extensions/{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_instances_extensions_get_count_6272(
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
                "/users/{}/calendar/calendarView/{}/instances/{}/extensions/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties` endpoint.
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
    pub async fn view_instances_list_multi_value_extended_propertie(
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
                "/users/{}/calendar/calendarView/{}/instances/{}/multiValueExtendedProperties?{}",
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
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties` endpoint.
    */
    pub async fn view_instances_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/multiValueExtendedProperties",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn view_instances_get_multi_value_extended_propertie(
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
&format!("/users/{}/calendar/calendarView/{}/instances/{}/multiValueExtendedProperties/{}?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),query_), None);
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_instances_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/multiValueExtendedProperties/{}",
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn view_instances_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/multiValueExtendedProperties/{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_instances_multi_value_extended_properties_get_count_1353(
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
&format!("/users/{}/calendar/calendarView/{}/instances/{}/multiValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties` endpoint.
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
    pub async fn view_instances_list_single_value_extended_propertie(
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
                "/users/{}/calendar/calendarView/{}/instances/{}/singleValueExtendedProperties?{}",
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
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties` endpoint.
    */
    pub async fn view_instances_create_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/singleValueExtendedProperties",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn view_instances_get_single_value_extended_propertie(
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
&format!("/users/{}/calendar/calendarView/{}/instances/{}/singleValueExtendedProperties/{}?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),query_), None);
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_instances_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/singleValueExtendedProperties/{}",
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn view_instances_update_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/singleValueExtendedProperties/{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_instances_single_value_extended_properties_get_count_b_8_0e(
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
&format!("/users/{}/calendar/calendarView/{}/instances/{}/singleValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_instances_get_count_fb_2d(
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
                "/users/{}/calendar/calendarView/{}/instances/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/multiValueExtendedProperties` endpoint.
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
    pub async fn view_list_multi_value_extended_propertie(
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
                "/users/{}/calendar/calendarView/{}/multiValueExtendedProperties?{}",
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
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/multiValueExtendedProperties` endpoint.
    */
    pub async fn view_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/multiValueExtendedProperties",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn view_get_multi_value_extended_propertie(
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
                "/users/{}/calendar/calendarView/{}/multiValueExtendedProperties/{}?{}",
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/calendarView/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/multiValueExtendedProperties/{}",
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendar/calendarView/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn view_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/multiValueExtendedProperties/{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_multi_value_extended_properties_get_count_dabc(
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
                "/users/{}/calendar/calendarView/{}/multiValueExtendedProperties/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/singleValueExtendedProperties` endpoint.
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
    pub async fn view_list_single_value_extended_propertie(
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
                "/users/{}/calendar/calendarView/{}/singleValueExtendedProperties?{}",
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
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/singleValueExtendedProperties` endpoint.
    */
    pub async fn view_create_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/singleValueExtendedProperties",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn view_get_single_value_extended_propertie(
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
                "/users/{}/calendar/calendarView/{}/singleValueExtendedProperties/{}?{}",
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/calendarView/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/singleValueExtendedProperties/{}",
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendar/calendarView/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn view_update_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/singleValueExtendedProperties/{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/{event-id}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_single_value_extended_properties_get_count_dcab(
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
                "/users/{}/calendar/calendarView/{}/singleValueExtendedProperties/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/calendarView/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn view_get_count_3c_51(&self, user_id: &str, filter: &str) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events` endpoint.
    *
    * Retrieve a list of events in a calendar. The calendar can be one for a user, or the default calendar of a Microsoft 365 group. The list of events contains single instance meetings and series masters. To get expanded event instances, you can get the calendar view, or
    * get the instances of an event.
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendar-list-events?view=graph-rest-1.0>
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
    pub async fn list_event(
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
                "/users/{}/calendar/events?{}",
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
    * Create event.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/events` endpoint.
    *
    * Use this API to create a new event in a calendar. The calendar can be one for a user, or the default calendar of a Microsoft 365 group.
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendar-post-events?view=graph-rest-1.0>
    */
    pub async fn create_events(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphEventAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphEventAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}` endpoint.
    *
    * The events in the calendar. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn get_event(
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
                "/users/{}/calendar/events/{}?{}",
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/events/{event-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn delete_events(
        &self,
        user_id: &str,
        event_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}",
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendar/events/{event-id}` endpoint.
    */
    pub async fn update_events(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphEventAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphEventAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/attachments` endpoint.
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
    pub async fn events_list_attachment(
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
                "/users/{}/calendar/events/{}/attachments?{}",
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
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/attachments` endpoint.
    *
    * Use this API to add an attachment to an existing event. This operation limits the size of the attachment you can add to under 3 MB. If an organizer adds an attachment to a meeting event, the organizer can subsequently update the event to send the attachment and update the event for each attendee as well.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-post-attachments?view=graph-rest-1.0>
    */
    pub async fn events_create_attachments(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphAttachmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/attachments",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/attachments/{attachment-id}` endpoint.
    *
    * The collection of FileAttachment, ItemAttachment, and referenceAttachment attachments for the event. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn events_get_attachment(
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
                "/users/{}/calendar/events/{}/attachments/{}?{}",
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/events/{event-id}/attachments/{attachment-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_delete_attachments(
        &self,
        user_id: &str,
        event_id: &str,
        attachment_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/attachments/{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/attachments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_attachments_get_count_11_4f(
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
                "/users/{}/calendar/events/{}/attachments/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/calendar` endpoint.
    *
    * The calendar that contains the event. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn events_get(
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
                "/users/{}/calendar/events/{}/calendar?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/extensions` endpoint.
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
    pub async fn events_list_extension(
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
                "/users/{}/calendar/events/{}/extensions?{}",
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
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/extensions` endpoint.
    *
    * Create an open extension (openTypeExtension object) and add custom properties in a new or existing instance of a resource. You can create an open extension in a resource instance and store custom data to it all in the same operation, except for specific resources. See known limitations of open extensions for more information. The table in the Permissions section lists the resources that support open extensions.
    *
    * FROM: <https://docs.microsoft.com/graph/api/opentypeextension-post-opentypeextension?view=graph-rest-1.0>
    */
    pub async fn events_create_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/extensions",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn events_get_extension(
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
                "/users/{}/calendar/events/{}/extensions/{}?{}",
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/events/{event-id}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_delete_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/extensions/{}",
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendar/events/{event-id}/extensions/{extension-id}` endpoint.
    */
    pub async fn events_update_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/extensions/{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_extensions_get_count_1_5ec(
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
                "/users/{}/calendar/events/{}/extensions/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/instances` endpoint.
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
    pub async fn events_list_instance(
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
                "/users/{}/calendar/events/{}/instances?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}` endpoint.
    *
    * The occurrences of a recurring series, if the event is a series master. This property includes occurrences that are part of the recurrence pattern, and exceptions that have been modified, but does not include occurrences that have been cancelled from the series. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `start_date_time: &str` -- The start date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T19:00:00-08:00.
    * * `end_date_time: &str` -- The end date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T20:00:00-08:00.
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn events_get_instance(
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
                "/users/{}/calendar/events/{}/instances/{}?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/attachments` endpoint.
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
    pub async fn events_instances_list_attachment(
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
                "/users/{}/calendar/events/{}/instances/{}/attachments?{}",
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
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/attachments` endpoint.
    *
    * Use this API to add an attachment to an existing event. This operation limits the size of the attachment you can add to under 3 MB. If an organizer adds an attachment to a meeting event, the organizer can subsequently update the event to send the attachment and update the event for each attendee as well.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-post-attachments?view=graph-rest-1.0>
    */
    pub async fn events_instances_create_attachments(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphAttachmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/attachments",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/attachments/{attachment-id}` endpoint.
    *
    * The collection of FileAttachment, ItemAttachment, and referenceAttachment attachments for the event. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn events_instances_get_attachment(
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
                "/users/{}/calendar/events/{}/instances/{}/attachments/{}?{}",
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/attachments/{attachment-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_instances_delete_attachments(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        attachment_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/attachments/{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/attachments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_instances_attachments_get_count_4bf_6(
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
                "/users/{}/calendar/events/{}/instances/{}/attachments/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/calendar` endpoint.
    *
    * The calendar that contains the event. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn events_instances_get(
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
                "/users/{}/calendar/events/{}/instances/{}/calendar?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/extensions` endpoint.
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
    pub async fn events_instances_list_extension(
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
                "/users/{}/calendar/events/{}/instances/{}/extensions?{}",
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
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/extensions` endpoint.
    *
    * Create an open extension (openTypeExtension object) and add custom properties in a new or existing instance of a resource. You can create an open extension in a resource instance and store custom data to it all in the same operation, except for specific resources. See known limitations of open extensions for more information. The table in the Permissions section lists the resources that support open extensions.
    *
    * FROM: <https://docs.microsoft.com/graph/api/opentypeextension-post-opentypeextension?view=graph-rest-1.0>
    */
    pub async fn events_instances_create_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/extensions",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn events_instances_get_extension(
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
                "/users/{}/calendar/events/{}/instances/{}/extensions/{}?{}",
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_instances_delete_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/extensions/{}",
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    */
    pub async fn events_instances_update_extensions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/extensions/{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_instances_extensions_get_count_6ae_3(
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
                "/users/{}/calendar/events/{}/instances/{}/extensions/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties` endpoint.
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
    pub async fn events_instances_list_multi_value_extended_propertie(
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
                "/users/{}/calendar/events/{}/instances/{}/multiValueExtendedProperties?{}",
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
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties` endpoint.
    */
    pub async fn events_instances_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/multiValueExtendedProperties",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn events_instances_get_multi_value_extended_propertie(
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
                "/users/{}/calendar/events/{}/instances/{}/multiValueExtendedProperties/{}?{}",
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_instances_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/multiValueExtendedProperties/{}",
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn events_instances_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/multiValueExtendedProperties/{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_instances_multi_value_extended_properties_get_count_9802(
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
                "/users/{}/calendar/events/{}/instances/{}/multiValueExtendedProperties/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties` endpoint.
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
    pub async fn events_instances_list_single_value_extended_propertie(
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
                "/users/{}/calendar/events/{}/instances/{}/singleValueExtendedProperties?{}",
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
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties` endpoint.
    */
    pub async fn events_instances_create_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/singleValueExtendedProperties",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn events_instances_get_single_value_extended_propertie(
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
                "/users/{}/calendar/events/{}/instances/{}/singleValueExtendedProperties/{}?{}",
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_instances_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/singleValueExtendedProperties/{}",
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn events_instances_update_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/singleValueExtendedProperties/{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_instances_single_value_extended_properties_get_count_1_7e_3(
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
                "/users/{}/calendar/events/{}/instances/{}/singleValueExtendedProperties/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/instances/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_instances_get_count_1d_53(
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
                "/users/{}/calendar/events/{}/instances/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/multiValueExtendedProperties` endpoint.
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
    pub async fn events_list_multi_value_extended_propertie(
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
                "/users/{}/calendar/events/{}/multiValueExtendedProperties?{}",
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
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/multiValueExtendedProperties` endpoint.
    */
    pub async fn events_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/multiValueExtendedProperties",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn events_get_multi_value_extended_propertie(
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
                "/users/{}/calendar/events/{}/multiValueExtendedProperties/{}?{}",
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/events/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/multiValueExtendedProperties/{}",
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendar/events/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn events_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/multiValueExtendedProperties/{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_multi_value_extended_properties_get_count_2_5fa(
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
                "/users/{}/calendar/events/{}/multiValueExtendedProperties/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/singleValueExtendedProperties` endpoint.
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
    pub async fn events_list_single_value_extended_propertie(
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
                "/users/{}/calendar/events/{}/singleValueExtendedProperties?{}",
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
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/singleValueExtendedProperties` endpoint.
    */
    pub async fn events_create_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/singleValueExtendedProperties",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn events_get_single_value_extended_propertie(
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
                "/users/{}/calendar/events/{}/singleValueExtendedProperties/{}?{}",
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/events/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/singleValueExtendedProperties/{}",
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendar/events/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn events_update_single_value_extended_properties(
        &self,
        user_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/singleValueExtendedProperties/{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/{event-id}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_single_value_extended_properties_get_count_1224(
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
                "/users/{}/calendar/events/{}/singleValueExtendedProperties/$count?{}",
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/events/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn events_get_count_1a_22(&self, user_id: &str, filter: &str) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/$count?{}",
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
    * Get multiValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendar/multiValueExtendedProperties` endpoint.
    *
    * The collection of multi-value extended properties defined for the calendar. Read-only. Nullable.
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
    pub async fn list_multi_value_extended_propertie(
        &self,
        user_id: &str,
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
                "/users/{}/calendar/multiValueExtendedProperties?{}",
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
    * Create new navigation property to multiValueExtendedProperties for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/multiValueExtendedProperties` endpoint.
    */
    pub async fn create_multi_value_extended_properties(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/multiValueExtendedProperties",
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
    * Get multiValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendar/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the calendar. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn get_multi_value_extended_propertie(
        &self,
        user_id: &str,
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
                "/users/{}/calendar/multiValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendar/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn update_multi_value_extended_properties(
        &self,
        user_id: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn multi_value_extended_properties_get_count_7316(
        &self,
        user_id: &str,
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
                "/users/{}/calendar/multiValueExtendedProperties/$count?{}",
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
    * Get singleValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendar/singleValueExtendedProperties` endpoint.
    *
    * The collection of single-value extended properties defined for the calendar. Read-only. Nullable.
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
    pub async fn list_single_value_extended_propertie(
        &self,
        user_id: &str,
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
                "/users/{}/calendar/singleValueExtendedProperties?{}",
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
    * Create new navigation property to singleValueExtendedProperties for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/singleValueExtendedProperties` endpoint.
    */
    pub async fn create_single_value_extended_properties(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/singleValueExtendedProperties",
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
    * Get singleValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendar/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the calendar. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn get_single_value_extended_propertie(
        &self,
        user_id: &str,
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
                "/users/{}/calendar/singleValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendar/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn delete_single_value_extended_properties(
        &self,
        user_id: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendar/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn update_single_value_extended_properties(
        &self,
        user_id: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendar/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn single_value_extended_properties_get_count_c_4fe(
        &self,
        user_id: &str,
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
                "/users/{}/calendar/singleValueExtendedProperties/$count?{}",
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
    * List calendars.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendars` endpoint.
    *
    * Get all the user's calendars (`/calendars` navigation property), get the calendars from the default calendar group or from a specific calendar group.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-list-calendars?view=graph-rest-1.0>
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
    pub async fn users_list_calendar(
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
                "/users/{}/calendars?{}",
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
    * Create calendar.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars` endpoint.
    *
    * Create a new calendar for a user.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-post-calendars?view=graph-rest-1.0>
    */
    pub async fn users_create_calendars(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphCalendarAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars",
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
    * Get calendars from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}` endpoint.
    *
    * The user's calendars. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_get_calendar_users_calendar(
        &self,
        user_id: &str,
        calendar_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarAllOf> {
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
                "/users/{}/calendars/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * Delete navigation property calendars for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_delete_calendars(
        &self,
        user_id: &str,
        calendar_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * Update the navigation property calendars in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/calendars/{calendar-id}` endpoint.
    */
    pub async fn users_update_calendars(
        &self,
        user_id: &str,
        calendar_id: &str,
        body: &crate::types::MicrosoftGraphCalendarAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * Get calendarPermissions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarPermissions` endpoint.
    *
    * The permissions of the users with whom the calendar is shared.
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_list_calendar_permission(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarPermissions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * Create calendarPermission.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarPermissions` endpoint.
    *
    * Create a calendarPermission resource to specify the identity and role of the user with whom the specified calendar is being shared or delegated.
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendar-post-calendarpermissions?view=graph-rest-1.0>
    */
    pub async fn s_create_calendar_permissions(
        &self,
        user_id: &str,
        calendar_id: &str,
        body: &crate::types::MicrosoftGraphCalendarPermissionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarPermissionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarPermissions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * Get calendarPermissions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarPermissions/{calendarPermission-id}` endpoint.
    *
    * The permissions of the users with whom the calendar is shared.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_get_calendar_permission(
        &self,
        user_id: &str,
        calendar_id: &str,
        calendar_permission_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarPermissionAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarPermissions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_permission_id.to_string()),
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
    * Delete navigation property calendarPermissions for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/calendarPermissions/{calendarPermission-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_calendar_permissions(
        &self,
        user_id: &str,
        calendar_id: &str,
        calendar_permission_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarPermissions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_permission_id.to_string()),
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
    * Update the navigation property calendarPermissions in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/calendars/{calendar-id}/calendarPermissions/{calendarPermission-id}` endpoint.
    */
    pub async fn s_update_calendar_permissions(
        &self,
        user_id: &str,
        calendar_id: &str,
        calendar_permission_id: &str,
        body: &crate::types::MicrosoftGraphCalendarPermissionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarPermissionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarPermissions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_permission_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarPermissions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_permissions_get_count_224(
        &self,
        user_id: &str,
        calendar_id: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarPermissions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView` endpoint.
    *
    * The calendar view for the calendar. Navigation property. Read-only.
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendar-list-calendarview?view=graph-rest-1.0>
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
    pub async fn s_list_calendar_view(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}` endpoint.
    *
    * The calendar view for the calendar. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `start_date_time: &str` -- The start date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T19:00:00-08:00.
    * * `end_date_time: &str` -- The end date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T20:00:00-08:00.
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_get_calendar_view(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/attachments` endpoint.
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
    pub async fn s_calendar_view_list_attachment(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/attachments?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/attachments` endpoint.
    *
    * Use this API to add an attachment to an existing event. This operation limits the size of the attachment you can add to under 3 MB. If an organizer adds an attachment to a meeting event, the organizer can subsequently update the event to send the attachment and update the event for each attendee as well.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-post-attachments?view=graph-rest-1.0>
    */
    pub async fn s_calendar_view_create_attachments(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphAttachmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/attachments",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/attachments/{attachment-id}` endpoint.
    *
    * The collection of FileAttachment, ItemAttachment, and referenceAttachment attachments for the event. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendar_view_get_attachment(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/attachments/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/attachments/{attachment-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_delete_attachments(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        attachment_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/attachments/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/attachments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_attachments_get_count_0b_1f(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/attachments/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/calendar` endpoint.
    *
    * The calendar that contains the event. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_calendar_view_get(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/calendar?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/extensions` endpoint.
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
    pub async fn s_calendar_view_list_extension(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/extensions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/extensions` endpoint.
    *
    * Create an open extension (openTypeExtension object) and add custom properties in a new or existing instance of a resource. You can create an open extension in a resource instance and store custom data to it all in the same operation, except for specific resources. See known limitations of open extensions for more information. The table in the Permissions section lists the resources that support open extensions.
    *
    * FROM: <https://docs.microsoft.com/graph/api/opentypeextension-post-opentypeextension?view=graph-rest-1.0>
    */
    pub async fn s_calendar_view_create_extensions(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/extensions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendar_view_get_extension(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/extensions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_delete_extensions(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/extensions/{extension-id}` endpoint.
    */
    pub async fn s_calendar_view_update_extensions(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_extensions_get_count_f_3d_6(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/extensions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances` endpoint.
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
    pub async fn s_calendar_view_list_instance(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/instances?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}` endpoint.
    *
    * The occurrences of a recurring series, if the event is a series master. This property includes occurrences that are part of the recurrence pattern, and exceptions that have been modified, but does not include occurrences that have been cancelled from the series. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `start_date_time: &str` -- The start date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T19:00:00-08:00.
    * * `end_date_time: &str` -- The end date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T20:00:00-08:00.
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_calendar_view_get_instance(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/instances/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/attachments` endpoint.
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
    pub async fn s_calendar_view_instances_list_attachment(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/attachments?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/attachments` endpoint.
    *
    * Use this API to add an attachment to an existing event. This operation limits the size of the attachment you can add to under 3 MB. If an organizer adds an attachment to a meeting event, the organizer can subsequently update the event to send the attachment and update the event for each attendee as well.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-post-attachments?view=graph-rest-1.0>
    */
    pub async fn s_calendar_view_instances_create_attachments(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphAttachmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/attachments",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/attachments/{attachment-id}` endpoint.
    *
    * The collection of FileAttachment, ItemAttachment, and referenceAttachment attachments for the event. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendar_view_instances_get_attachment(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/attachments/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/attachments/{attachment-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_instances_delete_attachments(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        attachment_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/attachments/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/attachments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_instances_attachments_get_count_4_8ab(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/attachments/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/calendar` endpoint.
    *
    * The calendar that contains the event. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_calendar_view_instances_get(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/calendar?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/extensions` endpoint.
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
    pub async fn s_calendar_view_instances_list_extension(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/extensions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/extensions` endpoint.
    *
    * Create an open extension (openTypeExtension object) and add custom properties in a new or existing instance of a resource. You can create an open extension in a resource instance and store custom data to it all in the same operation, except for specific resources. See known limitations of open extensions for more information. The table in the Permissions section lists the resources that support open extensions.
    *
    * FROM: <https://docs.microsoft.com/graph/api/opentypeextension-post-opentypeextension?view=graph-rest-1.0>
    */
    pub async fn s_calendar_view_instances_create_extensions(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/extensions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendar_view_instances_get_extension(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/extensions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_instances_delete_extensions(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    */
    pub async fn s_calendar_view_instances_update_extensions(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_instances_extensions_get_count_8140(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/extensions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties` endpoint.
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
    pub async fn s_calendar_view_instances_list_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
&format!("/users/{}/calendars/{}/calendarView/{}/instances/{}/multiValueExtendedProperties?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties` endpoint.
    */
    pub async fn s_calendar_view_instances_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/multiValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendar_view_instances_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
&format!("/users/{}/calendars/{}/calendarView/{}/instances/{}/multiValueExtendedProperties/{}?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),query_), None);
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_instances_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/calendars/{}/calendarView/{}/instances/{}/multiValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_calendar_view_instances_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/calendars/{}/calendarView/{}/instances/{}/multiValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_instances_multi_value_extended_properties_get_count_e_9_6e(
        &self,
        user_id: &str,
        calendar_id: &str,
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
&format!("/users/{}/calendars/{}/calendarView/{}/instances/{}/multiValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties` endpoint.
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
    pub async fn s_calendar_view_instances_list_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
&format!("/users/{}/calendars/{}/calendarView/{}/instances/{}/singleValueExtendedProperties?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties` endpoint.
    */
    pub async fn s_calendar_view_instances_create_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/singleValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendar_view_instances_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
&format!("/users/{}/calendars/{}/calendarView/{}/instances/{}/singleValueExtendedProperties/{}?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),query_), None);
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_instances_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/calendars/{}/calendarView/{}/instances/{}/singleValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_calendar_view_instances_update_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/calendars/{}/calendarView/{}/instances/{}/singleValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_instances_single_value_extended_properties_get_count_6590(
        &self,
        user_id: &str,
        calendar_id: &str,
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
&format!("/users/{}/calendars/{}/calendarView/{}/instances/{}/singleValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_instances_get_count_31_9b(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/instances/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/multiValueExtendedProperties` endpoint.
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
    pub async fn s_calendar_view_list_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/multiValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/multiValueExtendedProperties` endpoint.
    */
    pub async fn s_calendar_view_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/multiValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendar_view_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/multiValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_calendar_view_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_multi_value_extended_properties_get_count_e_7cb(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/multiValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/singleValueExtendedProperties` endpoint.
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
    pub async fn s_calendar_view_list_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/singleValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/singleValueExtendedProperties` endpoint.
    */
    pub async fn s_calendar_view_create_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/singleValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendar_view_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/singleValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_calendar_view_update_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_single_value_extended_properties_get_count_8baa(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/calendarView/{}/singleValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendar_view_get_count_0591(
        &self,
        user_id: &str,
        calendar_id: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events` endpoint.
    *
    * Retrieve a list of events in a calendar. The calendar can be one for a user, or the default calendar of a Microsoft 365 group. The list of events contains single instance meetings and series masters. To get expanded event instances, you can get the calendar view, or
    * get the instances of an event.
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendar-list-events?view=graph-rest-1.0>
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
    pub async fn s_list_event(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * Create event.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events` endpoint.
    *
    * Use this API to create a new event in a calendar. The calendar can be one for a user, or the default calendar of a Microsoft 365 group.
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendar-post-events?view=graph-rest-1.0>
    */
    pub async fn s_create_events(
        &self,
        user_id: &str,
        calendar_id: &str,
        body: &crate::types::MicrosoftGraphEventAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphEventAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}` endpoint.
    *
    * The events in the calendar. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_event(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_events(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}` endpoint.
    */
    pub async fn s_update_events(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphEventAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphEventAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/attachments` endpoint.
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
    pub async fn s_events_list_attachment(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/attachments?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/attachments` endpoint.
    *
    * Use this API to add an attachment to an existing event. This operation limits the size of the attachment you can add to under 3 MB. If an organizer adds an attachment to a meeting event, the organizer can subsequently update the event to send the attachment and update the event for each attendee as well.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-post-attachments?view=graph-rest-1.0>
    */
    pub async fn s_events_create_attachments(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphAttachmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/attachments",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/attachments/{attachment-id}` endpoint.
    *
    * The collection of FileAttachment, ItemAttachment, and referenceAttachment attachments for the event. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_events_get_attachment(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/attachments/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/attachments/{attachment-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_delete_attachments(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        attachment_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/attachments/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/attachments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_attachments_get_count_8147(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/attachments/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/calendar` endpoint.
    *
    * The calendar that contains the event. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_events_get_calendar(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/calendar?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/extensions` endpoint.
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
    pub async fn s_events_list_extension(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/extensions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/extensions` endpoint.
    *
    * Create an open extension (openTypeExtension object) and add custom properties in a new or existing instance of a resource. You can create an open extension in a resource instance and store custom data to it all in the same operation, except for specific resources. See known limitations of open extensions for more information. The table in the Permissions section lists the resources that support open extensions.
    *
    * FROM: <https://docs.microsoft.com/graph/api/opentypeextension-post-opentypeextension?view=graph-rest-1.0>
    */
    pub async fn s_events_create_extensions(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/extensions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_events_get_extension(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/extensions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_delete_extensions(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/extensions/{extension-id}` endpoint.
    */
    pub async fn s_events_update_extensions(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_extensions_get_count_b_4_4d(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/extensions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances` endpoint.
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
    pub async fn s_events_list_instance(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/instances?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}` endpoint.
    *
    * The occurrences of a recurring series, if the event is a series master. This property includes occurrences that are part of the recurrence pattern, and exceptions that have been modified, but does not include occurrences that have been cancelled from the series. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `start_date_time: &str` -- The start date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T19:00:00-08:00.
    * * `end_date_time: &str` -- The end date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T20:00:00-08:00.
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_events_get_instance(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/instances/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/attachments` endpoint.
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
    pub async fn s_events_instances_list_attachment(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/instances/{}/attachments?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/attachments` endpoint.
    *
    * Use this API to add an attachment to an existing event. This operation limits the size of the attachment you can add to under 3 MB. If an organizer adds an attachment to a meeting event, the organizer can subsequently update the event to send the attachment and update the event for each attendee as well.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-post-attachments?view=graph-rest-1.0>
    */
    pub async fn s_events_instances_create_attachments(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphAttachmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/attachments",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/attachments/{attachment-id}` endpoint.
    *
    * The collection of FileAttachment, ItemAttachment, and referenceAttachment attachments for the event. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_events_instances_get_attachment(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/instances/{}/attachments/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/attachments/{attachment-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_instances_delete_attachments(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        attachment_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/attachments/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/attachments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_instances_attachments_get_count_e_3_5a(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/instances/{}/attachments/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/calendar` endpoint.
    *
    * The calendar that contains the event. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_events_instances_get_calendar(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/instances/{}/calendar?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/extensions` endpoint.
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
    pub async fn s_events_instances_list_extension(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/instances/{}/extensions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/extensions` endpoint.
    *
    * Create an open extension (openTypeExtension object) and add custom properties in a new or existing instance of a resource. You can create an open extension in a resource instance and store custom data to it all in the same operation, except for specific resources. See known limitations of open extensions for more information. The table in the Permissions section lists the resources that support open extensions.
    *
    * FROM: <https://docs.microsoft.com/graph/api/opentypeextension-post-opentypeextension?view=graph-rest-1.0>
    */
    pub async fn s_events_instances_create_extensions(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/extensions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_events_instances_get_extension(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/instances/{}/extensions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_instances_delete_extensions(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    */
    pub async fn s_events_instances_update_extensions(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_instances_extensions_get_count_dca_1(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/instances/{}/extensions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties` endpoint.
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
    pub async fn s_events_instances_list_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/instances/{}/multiValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties` endpoint.
    */
    pub async fn s_events_instances_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/multiValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_events_instances_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/instances/{}/multiValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_instances_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_events_instances_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_instances_multi_value_extended_properties_get_count_0_8fb(
        &self,
        user_id: &str,
        calendar_id: &str,
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
&format!("/users/{}/calendars/{}/events/{}/instances/{}/multiValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties` endpoint.
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
    pub async fn s_events_instances_list_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/instances/{}/singleValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties` endpoint.
    */
    pub async fn s_events_instances_create_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/singleValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_events_instances_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/instances/{}/singleValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_instances_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_events_instances_update_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_instances_single_value_extended_properties_get_count_6_6c_4(
        &self,
        user_id: &str,
        calendar_id: &str,
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
&format!("/users/{}/calendars/{}/events/{}/instances/{}/singleValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_instances_get_count_c_0bc(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/instances/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/multiValueExtendedProperties` endpoint.
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
    pub async fn s_events_list_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/multiValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/multiValueExtendedProperties` endpoint.
    */
    pub async fn s_events_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/multiValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_events_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/multiValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_events_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_multi_value_extended_properties_get_count_afcf(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/multiValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/singleValueExtendedProperties` endpoint.
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
    pub async fn s_events_list_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/singleValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/singleValueExtendedProperties` endpoint.
    */
    pub async fn s_events_create_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/singleValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_events_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/singleValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_events_update_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_single_value_extended_properties_get_count_1946(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/events/{}/singleValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/events/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_events_get_count_efc_7(
        &self,
        user_id: &str,
        calendar_id: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/multiValueExtendedProperties` endpoint.
    *
    * The collection of multi-value extended properties defined for the calendar. Read-only. Nullable.
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
        calendar_id: &str,
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
                "/users/{}/calendars/{}/multiValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/multiValueExtendedProperties` endpoint.
    */
    pub async fn s_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/multiValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the calendar. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/multiValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendars/{calendar-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_multi_value_extended_properties_get_count_09_7f(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/multiValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/singleValueExtendedProperties` endpoint.
    *
    * The collection of single-value extended properties defined for the calendar. Read-only. Nullable.
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
        calendar_id: &str,
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
                "/users/{}/calendars/{}/singleValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/singleValueExtendedProperties` endpoint.
    */
    pub async fn s_create_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/singleValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the calendar. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/singleValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendars/{calendar-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendars/{calendar-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_update_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_id: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/{calendar-id}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_single_value_extended_properties_get_count_f_892(
        &self,
        user_id: &str,
        calendar_id: &str,
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
                "/users/{}/calendars/{}/singleValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendars/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_get_count_1b_5(&self, user_id: &str, filter: &str) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/$count?{}",
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
