use crate::Client;
use crate::ClientResult;

pub struct UsersCalendarGroup {
    pub client: Client,
}

impl UsersCalendarGroup {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersCalendarGroup { client }
    }

    /**
    * List calendarGroups.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups` endpoint.
    *
    * Get the user's calendar groups.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-list-calendargroups?view=graph-rest-1.0>
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
    pub async fn users_list_calendar_group(
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
                "/users/{}/calendarGroups?{}",
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
    * Create CalendarGroup.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups` endpoint.
    *
    * Use this API to create a new CalendarGroup.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-post-calendargroups?view=graph-rest-1.0>
    */
    pub async fn users_create_calendar_groups(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphCalendarGroupAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarGroupAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups",
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
    * Get calendarGroups from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}` endpoint.
    *
    * The user's calendar groups. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn users_get_calendar_group(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarGroupAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * Delete navigation property calendarGroups for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_delete_calendar_groups(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * Update the navigation property calendarGroups in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}` endpoint.
    */
    pub async fn users_update_calendar_groups(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        body: &crate::types::MicrosoftGraphCalendarGroupAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarGroupAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * List calendars.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars` endpoint.
    *
    * Retrieve a list of calendars belonging to a calendar group.
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendargroup-list-calendars?view=graph-rest-1.0>
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
    pub async fn s_list_calendar(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * Create Calendar.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars` endpoint.
    *
    * Use this API to create a new calendar in a calendar group for a user.
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendargroup-post-calendars?view=graph-rest-1.0>
    */
    pub async fn s_create_calendars(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        body: &crate::types::MicrosoftGraphCalendarAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}` endpoint.
    *
    * The calendars in the calendar group. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_get_calendar(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_calendars(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}` endpoint.
    */
    pub async fn s_update_calendars(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        body: &crate::types::MicrosoftGraphCalendarAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarPermissions` endpoint.
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
    pub async fn s_calendars_list_permission(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/calendarPermissions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarPermissions` endpoint.
    *
    * Create a calendarPermission resource to specify the identity and role of the user with whom the specified calendar is being shared or delegated.
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendar-post-calendarpermissions?view=graph-rest-1.0>
    */
    pub async fn s_calendars_create_permissions(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        body: &crate::types::MicrosoftGraphCalendarPermissionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarPermissionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarPermissions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarPermissions/{calendarPermission-id}` endpoint.
    *
    * The permissions of the users with whom the calendar is shared.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_calendars_get_permission(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/calendarPermissions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarPermissions/{calendarPermission-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_delete_permissions(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        calendar_permission_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarPermissions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarPermissions/{calendarPermission-id}` endpoint.
    */
    pub async fn s_calendars_update_permissions(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        calendar_permission_id: &str,
        body: &crate::types::MicrosoftGraphCalendarPermissionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphCalendarPermissionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarPermissions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarPermissions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_permissions_get_count_9_8a_8(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/calendarPermissions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * List calendarView.
    *
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView` endpoint.
    *
    * Get the occurrences, exceptions and single instances of events in a calendar view defined by a time range,
    * from a user's default calendar `(../me/calendarView)` or some other calendar of the user's.
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendar-list-calendarview?view=graph-rest-1.0>
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
    pub async fn s_calendars_list_view(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}` endpoint.
    *
    * The calendar view for the calendar. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_calendars_get_view(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphEventAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/attachments` endpoint.
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
    pub async fn s_calendars_view_list_attachment(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/attachments?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/attachments` endpoint.
    *
    * Use this API to add an attachment to an existing event. This operation limits the size of the attachment you can add to under 3 MB. If an organizer adds an attachment to a meeting event, the organizer can subsequently update the event to send the attachment and update the event for each attendee as well.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-post-attachments?view=graph-rest-1.0>
    */
    pub async fn s_calendars_view_create_attachments(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphAttachmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/attachments",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/attachments/{attachment-id}` endpoint.
    *
    * The collection of FileAttachment, ItemAttachment, and referenceAttachment attachments for the event. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_view_get_attachment(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/attachments/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/attachments/{attachment-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_delete_attachments(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        attachment_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/attachments/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/attachments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_attachments_get_count_1276(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/attachments/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/calendar` endpoint.
    *
    * The calendar that contains the event. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_calendars_view_get(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/calendar?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/extensions` endpoint.
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
    pub async fn s_calendars_view_list_extension(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/extensions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/extensions` endpoint.
    *
    * Create an open extension (openTypeExtension object) and add custom properties in a new or existing instance of a resource. You can create an open extension in a resource instance and store custom data to it all in the same operation, except for specific resources. See known limitations of open extensions for more information. The table in the Permissions section lists the resources that support open extensions.
    *
    * FROM: <https://docs.microsoft.com/graph/api/opentypeextension-post-opentypeextension?view=graph-rest-1.0>
    */
    pub async fn s_calendars_view_create_extensions(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/extensions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_view_get_extension(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/extensions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_delete_extensions(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/extensions/{extension-id}` endpoint.
    */
    pub async fn s_calendars_view_update_extensions(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_extensions_get_count_dc_55(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/extensions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances` endpoint.
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
    pub async fn s_calendars_view_list_instance(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}` endpoint.
    *
    * The occurrences of a recurring series, if the event is a series master. This property includes occurrences that are part of the recurrence pattern, and exceptions that have been modified, but does not include occurrences that have been cancelled from the series. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `start_date_time: &str` -- The start date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T19:00:00-08:00.
    * * `end_date_time: &str` -- The end date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T20:00:00-08:00.
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_calendars_view_get_instance(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/attachments` endpoint.
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
    pub async fn s_calendars_view_instances_list_attachment(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/attachments?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/attachments` endpoint.
    *
    * Use this API to add an attachment to an existing event. This operation limits the size of the attachment you can add to under 3 MB. If an organizer adds an attachment to a meeting event, the organizer can subsequently update the event to send the attachment and update the event for each attendee as well.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-post-attachments?view=graph-rest-1.0>
    */
    pub async fn s_calendars_view_instances_create_attachments(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphAttachmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/attachments",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/attachments/{attachment-id}` endpoint.
    *
    * The collection of FileAttachment, ItemAttachment, and referenceAttachment attachments for the event. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_view_instances_get_attachment(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/attachments/{}?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&attachment_id.to_string()),query_), None);
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/attachments/{attachment-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_instances_delete_attachments(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        attachment_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/attachments/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&attachment_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/attachments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_instances_attachments_get_count_7469(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/attachments/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/calendar` endpoint.
    *
    * The calendar that contains the event. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_calendars_view_instances_get(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/calendar?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/extensions` endpoint.
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
    pub async fn s_calendars_view_instances_list_extension(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/extensions?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/extensions` endpoint.
    *
    * Create an open extension (openTypeExtension object) and add custom properties in a new or existing instance of a resource. You can create an open extension in a resource instance and store custom data to it all in the same operation, except for specific resources. See known limitations of open extensions for more information. The table in the Permissions section lists the resources that support open extensions.
    *
    * FROM: <https://docs.microsoft.com/graph/api/opentypeextension-post-opentypeextension?view=graph-rest-1.0>
    */
    pub async fn s_calendars_view_instances_create_extensions(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/extensions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_view_instances_get_extension(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/extensions/{}?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&extension_id.to_string()),query_), None);
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_instances_delete_extensions(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/extensions/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&extension_id.to_string()),), None);
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    */
    pub async fn s_calendars_view_instances_update_extensions(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/extensions/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&extension_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_instances_extensions_get_count_8068(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/extensions/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties` endpoint.
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
    pub async fn s_calendars_view_instances_list_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/multiValueExtendedProperties?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties` endpoint.
    */
    pub async fn s_calendars_view_instances_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/multiValueExtendedProperties",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_view_instances_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/multiValueExtendedProperties/{}?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),query_), None);
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_instances_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/multiValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_calendars_view_instances_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/multiValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_instances_multi_value_extended_properties_get_count_0234(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/multiValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties` endpoint.
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
    pub async fn s_calendars_view_instances_list_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/singleValueExtendedProperties?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties` endpoint.
    */
    pub async fn s_calendars_view_instances_create_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/singleValueExtendedProperties",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_view_instances_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/singleValueExtendedProperties/{}?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),query_), None);
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_instances_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/singleValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_calendars_view_instances_update_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/singleValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_instances_single_value_extended_properties_get_count_1dff(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/singleValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_instances_get_count_6a_91(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/multiValueExtendedProperties` endpoint.
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
    pub async fn s_calendars_view_list_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/multiValueExtendedProperties?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),query_), None);
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/multiValueExtendedProperties` endpoint.
    */
    pub async fn s_calendars_view_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/multiValueExtendedProperties",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_view_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/multiValueExtendedProperties/{}?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),query_), None);
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/multiValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_calendars_view_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/multiValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_multi_value_extended_properties_get_count_5_8f_7(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/multiValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/singleValueExtendedProperties` endpoint.
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
    pub async fn s_calendars_view_list_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/singleValueExtendedProperties?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),query_), None);
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/singleValueExtendedProperties` endpoint.
    */
    pub async fn s_calendars_view_create_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/singleValueExtendedProperties",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_view_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/singleValueExtendedProperties/{}?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),query_), None);
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/singleValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_calendars_view_update_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/singleValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_single_value_extended_properties_get_count_d_882(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/singleValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_view_get_count_ff_1a(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events` endpoint.
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
    pub async fn s_calendars_list_event(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events` endpoint.
    *
    * Use this API to create a new event in a calendar. The calendar can be one for a user, or the default calendar of a Microsoft 365 group.
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendar-post-events?view=graph-rest-1.0>
    */
    pub async fn s_calendars_create_events(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        body: &crate::types::MicrosoftGraphEventAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphEventAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}` endpoint.
    *
    * The events in the calendar. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_get_event(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_delete_events(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}` endpoint.
    */
    pub async fn s_calendars_update_events(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphEventAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphEventAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/attachments` endpoint.
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
    pub async fn s_calendars_events_list_attachment(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/attachments?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/attachments` endpoint.
    *
    * Use this API to add an attachment to an existing event. This operation limits the size of the attachment you can add to under 3 MB. If an organizer adds an attachment to a meeting event, the organizer can subsequently update the event to send the attachment and update the event for each attendee as well.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-post-attachments?view=graph-rest-1.0>
    */
    pub async fn s_calendars_events_create_attachments(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphAttachmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/attachments",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/attachments/{attachment-id}` endpoint.
    *
    * The collection of FileAttachment, ItemAttachment, and referenceAttachment attachments for the event. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_events_get_attachment(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/attachments/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/attachments/{attachment-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_delete_attachments(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        attachment_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/attachments/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/attachments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_attachments_get_count_e_742(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/attachments/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/calendar` endpoint.
    *
    * The calendar that contains the event. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_calendars_events_get(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/calendar?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/extensions` endpoint.
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
    pub async fn s_calendars_events_list_extension(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/extensions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/extensions` endpoint.
    *
    * Create an open extension (openTypeExtension object) and add custom properties in a new or existing instance of a resource. You can create an open extension in a resource instance and store custom data to it all in the same operation, except for specific resources. See known limitations of open extensions for more information. The table in the Permissions section lists the resources that support open extensions.
    *
    * FROM: <https://docs.microsoft.com/graph/api/opentypeextension-post-opentypeextension?view=graph-rest-1.0>
    */
    pub async fn s_calendars_events_create_extensions(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/extensions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_events_get_extension(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/extensions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_delete_extensions(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/extensions/{extension-id}` endpoint.
    */
    pub async fn s_calendars_events_update_extensions(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_extensions_get_count_4cca(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/extensions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances` endpoint.
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
    pub async fn s_calendars_events_list_instance(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}` endpoint.
    *
    * The occurrences of a recurring series, if the event is a series master. This property includes occurrences that are part of the recurrence pattern, and exceptions that have been modified, but does not include occurrences that have been cancelled from the series. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `start_date_time: &str` -- The start date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T19:00:00-08:00.
    * * `end_date_time: &str` -- The end date and time of the time range, represented in ISO 8601 format. For example, 2019-11-08T20:00:00-08:00.
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_calendars_events_get_instance(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/attachments` endpoint.
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
    pub async fn s_calendars_events_instances_list_attachment(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/attachments?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/attachments` endpoint.
    *
    * Use this API to add an attachment to an existing event. This operation limits the size of the attachment you can add to under 3 MB. If an organizer adds an attachment to a meeting event, the organizer can subsequently update the event to send the attachment and update the event for each attendee as well.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-post-attachments?view=graph-rest-1.0>
    */
    pub async fn s_calendars_events_instances_create_attachments(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphAttachmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/attachments",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/attachments/{attachment-id}` endpoint.
    *
    * The collection of FileAttachment, ItemAttachment, and referenceAttachment attachments for the event. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_events_instances_get_attachment(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/attachments/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/attachments/{attachment-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_instances_delete_attachments(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        attachment_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/attachments/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/attachments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_instances_attachments_get_count_e_572(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/attachments/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/calendar` endpoint.
    *
    * The calendar that contains the event. Navigation property. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_calendars_events_instances_get(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/calendar?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/extensions` endpoint.
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
    pub async fn s_calendars_events_instances_list_extension(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/extensions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/extensions` endpoint.
    *
    * Create an open extension (openTypeExtension object) and add custom properties in a new or existing instance of a resource. You can create an open extension in a resource instance and store custom data to it all in the same operation, except for specific resources. See known limitations of open extensions for more information. The table in the Permissions section lists the resources that support open extensions.
    *
    * FROM: <https://docs.microsoft.com/graph/api/opentypeextension-post-opentypeextension?view=graph-rest-1.0>
    */
    pub async fn s_calendars_events_instances_create_extensions(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/extensions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the event. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_events_instances_get_extension(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/extensions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_instances_delete_extensions(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/extensions/{extension-id}` endpoint.
    */
    pub async fn s_calendars_events_instances_update_extensions(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_instances_extensions_get_count_7_5cb(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/extensions/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties` endpoint.
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
    pub async fn s_calendars_events_instances_list_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/multiValueExtendedProperties?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties` endpoint.
    */
    pub async fn s_calendars_events_instances_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/multiValueExtendedProperties",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_events_instances_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/multiValueExtendedProperties/{}?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),query_), None);
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_instances_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/multiValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_calendars_events_instances_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/multiValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_instances_multi_value_extended_properties_get_count_6fb_3(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/multiValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties` endpoint.
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
    pub async fn s_calendars_events_instances_list_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/singleValueExtendedProperties?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties` endpoint.
    */
    pub async fn s_calendars_events_instances_create_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/singleValueExtendedProperties",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_events_instances_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/singleValueExtendedProperties/{}?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),query_), None);
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_instances_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/singleValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_calendars_events_instances_update_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/singleValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_instances_single_value_extended_properties_get_count_ac_17(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/singleValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_instances_get_count_ee_5f(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/multiValueExtendedProperties` endpoint.
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
    pub async fn s_calendars_events_list_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/multiValueExtendedProperties?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),query_), None);
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/multiValueExtendedProperties` endpoint.
    */
    pub async fn s_calendars_events_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/multiValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_events_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/multiValueExtendedProperties/{}?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),query_), None);
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/multiValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_calendars_events_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/multiValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_multi_value_extended_properties_get_count_2db_5(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/multiValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/singleValueExtendedProperties` endpoint.
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
    pub async fn s_calendars_events_list_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/singleValueExtendedProperties?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),query_), None);
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/singleValueExtendedProperties` endpoint.
    */
    pub async fn s_calendars_events_create_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/singleValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the event. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_events_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/singleValueExtendedProperties/{}?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),query_), None);
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/singleValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_calendars_events_update_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/singleValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_single_value_extended_properties_get_count_9ef_7(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/singleValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_events_get_count_f_3ad(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/events/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/multiValueExtendedProperties` endpoint.
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
    pub async fn s_calendars_list_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/multiValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/multiValueExtendedProperties` endpoint.
    */
    pub async fn s_calendars_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/multiValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the calendar. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/multiValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_calendars_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_multi_value_extended_properties_get_count_52_8f(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/multiValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/singleValueExtendedProperties` endpoint.
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
    pub async fn s_calendars_list_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/singleValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/singleValueExtendedProperties` endpoint.
    */
    pub async fn s_calendars_create_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/singleValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the calendar. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_calendars_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/singleValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_calendars_update_single_value_extended_properties(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_single_value_extended_properties_get_count_2299(
        &self,
        user_id: &str,
        calendar_group_id: &str,
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
                "/users/{}/calendarGroups/{}/calendars/{}/singleValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_calendars_get_count_8e_45(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&calendar_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/calendarGroups/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_get_count_ee_80(&self, user_id: &str, filter: &str) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/$count?{}",
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
