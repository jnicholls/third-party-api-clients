use crate::Client;
use crate::ClientResult;

pub struct UsersOnlineMeeting {
    pub client: Client,
}

impl UsersOnlineMeeting {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersOnlineMeeting { client }
    }

    /**
    * Get onlineMeeting.
    *
    * This function performs a `GET` to the `/users/{user-id}/onlineMeetings` endpoint.
    *
    * Retrieve the properties and relationships of an onlineMeeting object. For example, you can: Teams live event attendee report is an online meeting artifact. For details, see Online meeting artifacts and permissions.
    *
    * FROM: <https://docs.microsoft.com/graph/api/onlinemeeting-get?view=graph-rest-1.0>
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
    pub async fn users_list_online_meeting(
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
                "/users/{}/onlineMeetings?{}",
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
    * Create onlineMeeting.
    *
    * This function performs a `POST` to the `/users/{user-id}/onlineMeetings` endpoint.
    *
    * Create an online meeting on behalf of a user.
    *
    * FROM: <https://docs.microsoft.com/graph/api/application-post-onlinemeetings?view=graph-rest-1.0>
    */
    pub async fn users_create_online_meetings(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphOnlineMeetingAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnlineMeetingAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onlineMeetings",
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
    * Get onlineMeetings from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onlineMeetings/{onlineMeeting-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_get_online_meeting(
        &self,
        user_id: &str,
        online_meeting_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnlineMeetingAllOf> {
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
                "/users/{}/onlineMeetings/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&online_meeting_id.to_string()),
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
    * Delete navigation property onlineMeetings for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/onlineMeetings/{onlineMeeting-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_delete_online_meetings(
        &self,
        user_id: &str,
        online_meeting_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onlineMeetings/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&online_meeting_id.to_string()),
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
    * Update the navigation property onlineMeetings in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/onlineMeetings/{onlineMeeting-id}` endpoint.
    */
    pub async fn users_update_online_meetings(
        &self,
        user_id: &str,
        online_meeting_id: &str,
        body: &crate::types::MicrosoftGraphOnlineMeetingAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnlineMeetingAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onlineMeetings/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&online_meeting_id.to_string()),
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
    * List meetingAttendanceReports.
    *
    * This function performs a `GET` to the `/users/{user-id}/onlineMeetings/{onlineMeeting-id}/attendanceReports` endpoint.
    *
    * Get a list of meetingAttendanceReport objects for an onlineMeeting. Each time an online meeting ends, an attendance report is generated for that session.
    *
    * FROM: <https://docs.microsoft.com/graph/api/meetingattendancereport-list?view=graph-rest-1.0>
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
    pub async fn s_list_attendance_report(
        &self,
        user_id: &str,
        online_meeting_id: &str,
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
                "/users/{}/onlineMeetings/{}/attendanceReports?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&online_meeting_id.to_string()),
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
    * Create new navigation property to attendanceReports for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/onlineMeetings/{onlineMeeting-id}/attendanceReports` endpoint.
    */
    pub async fn s_create_attendance_reports(
        &self,
        user_id: &str,
        online_meeting_id: &str,
        body: &crate::types::MicrosoftGraphMeetingAttendanceReportAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMeetingAttendanceReportAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onlineMeetings/{}/attendanceReports",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&online_meeting_id.to_string()),
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
    * Get attendanceReports from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onlineMeetings/{onlineMeeting-id}/attendanceReports/{meetingAttendanceReport-id}` endpoint.
    *
    * The attendance reports of an online meeting. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_attendance_report(
        &self,
        user_id: &str,
        online_meeting_id: &str,
        meeting_attendance_report_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphMeetingAttendanceReportAllOf> {
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
                "/users/{}/onlineMeetings/{}/attendanceReports/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&online_meeting_id.to_string()),
                crate::progenitor_support::encode_path(&meeting_attendance_report_id.to_string()),
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
    * Delete navigation property attendanceReports for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/onlineMeetings/{onlineMeeting-id}/attendanceReports/{meetingAttendanceReport-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_attendance_reports(
        &self,
        user_id: &str,
        online_meeting_id: &str,
        meeting_attendance_report_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onlineMeetings/{}/attendanceReports/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&online_meeting_id.to_string()),
                crate::progenitor_support::encode_path(&meeting_attendance_report_id.to_string()),
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
    * Update the navigation property attendanceReports in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/onlineMeetings/{onlineMeeting-id}/attendanceReports/{meetingAttendanceReport-id}` endpoint.
    */
    pub async fn s_update_attendance_reports(
        &self,
        user_id: &str,
        online_meeting_id: &str,
        meeting_attendance_report_id: &str,
        body: &crate::types::MicrosoftGraphMeetingAttendanceReportAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMeetingAttendanceReportAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onlineMeetings/{}/attendanceReports/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&online_meeting_id.to_string()),
                crate::progenitor_support::encode_path(&meeting_attendance_report_id.to_string()),
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
    * List attendanceRecords.
    *
    * This function performs a `GET` to the `/users/{user-id}/onlineMeetings/{onlineMeeting-id}/attendanceReports/{meetingAttendanceReport-id}/attendanceRecords` endpoint.
    *
    * Get a list of attendanceRecord objects and their properties.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attendancerecord-list?view=graph-rest-1.0>
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
    pub async fn s_attendance_reports_list_record(
        &self,
        user_id: &str,
        online_meeting_id: &str,
        meeting_attendance_report_id: &str,
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
                "/users/{}/onlineMeetings/{}/attendanceReports/{}/attendanceRecords?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&online_meeting_id.to_string()),
                crate::progenitor_support::encode_path(&meeting_attendance_report_id.to_string()),
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
    * Create new navigation property to attendanceRecords for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/onlineMeetings/{onlineMeeting-id}/attendanceReports/{meetingAttendanceReport-id}/attendanceRecords` endpoint.
    */
    pub async fn s_attendance_reports_create_records(
        &self,
        user_id: &str,
        online_meeting_id: &str,
        meeting_attendance_report_id: &str,
        body: &crate::types::MicrosoftGraphAttendanceRecordAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttendanceRecordAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onlineMeetings/{}/attendanceReports/{}/attendanceRecords",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&online_meeting_id.to_string()),
                crate::progenitor_support::encode_path(&meeting_attendance_report_id.to_string()),
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
    * Get attendanceRecords from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onlineMeetings/{onlineMeeting-id}/attendanceReports/{meetingAttendanceReport-id}/attendanceRecords/{attendanceRecord-id}` endpoint.
    *
    * List of attendance records of an attendance report. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_attendance_reports_get_record(
        &self,
        user_id: &str,
        online_meeting_id: &str,
        meeting_attendance_report_id: &str,
        attendance_record_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphAttendanceRecordAllOf> {
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
                "/users/{}/onlineMeetings/{}/attendanceReports/{}/attendanceRecords/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&online_meeting_id.to_string()),
                crate::progenitor_support::encode_path(&meeting_attendance_report_id.to_string()),
                crate::progenitor_support::encode_path(&attendance_record_id.to_string()),
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
    * Delete navigation property attendanceRecords for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/onlineMeetings/{onlineMeeting-id}/attendanceReports/{meetingAttendanceReport-id}/attendanceRecords/{attendanceRecord-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_attendance_reports_delete_records(
        &self,
        user_id: &str,
        online_meeting_id: &str,
        meeting_attendance_report_id: &str,
        attendance_record_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onlineMeetings/{}/attendanceReports/{}/attendanceRecords/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&online_meeting_id.to_string()),
                crate::progenitor_support::encode_path(&meeting_attendance_report_id.to_string()),
                crate::progenitor_support::encode_path(&attendance_record_id.to_string()),
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
    * Update the navigation property attendanceRecords in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/onlineMeetings/{onlineMeeting-id}/attendanceReports/{meetingAttendanceReport-id}/attendanceRecords/{attendanceRecord-id}` endpoint.
    */
    pub async fn s_attendance_reports_update_records(
        &self,
        user_id: &str,
        online_meeting_id: &str,
        meeting_attendance_report_id: &str,
        attendance_record_id: &str,
        body: &crate::types::MicrosoftGraphAttendanceRecordAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttendanceRecordAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onlineMeetings/{}/attendanceReports/{}/attendanceRecords/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&online_meeting_id.to_string()),
                crate::progenitor_support::encode_path(&meeting_attendance_report_id.to_string()),
                crate::progenitor_support::encode_path(&attendance_record_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/onlineMeetings/{onlineMeeting-id}/attendanceReports/{meetingAttendanceReport-id}/attendanceRecords/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_attendance_reports_records_get_count_3340(
        &self,
        user_id: &str,
        online_meeting_id: &str,
        meeting_attendance_report_id: &str,
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
                "/users/{}/onlineMeetings/{}/attendanceReports/{}/attendanceRecords/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&online_meeting_id.to_string()),
                crate::progenitor_support::encode_path(&meeting_attendance_report_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/onlineMeetings/{onlineMeeting-id}/attendanceReports/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_attendance_reports_get_count_e_849(
        &self,
        user_id: &str,
        online_meeting_id: &str,
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
                "/users/{}/onlineMeetings/{}/attendanceReports/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&online_meeting_id.to_string()),
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
    * Get attendeeReport for the navigation property onlineMeetings from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onlineMeetings/{onlineMeeting-id}/attendeeReport` endpoint.
    *
    * The content stream of the attendee report of a Microsoft Teams live event. Read-only.
    *
    * FROM: <https://docs.microsoft.com/graph/api/onlinemeeting-get?view=graph-rest-1.0>
    */
    pub async fn users_get_online_meetings_attendee_report(
        &self,
        user_id: &str,
        online_meeting_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onlineMeetings/{}/attendeeReport",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&online_meeting_id.to_string()),
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
    * Update attendeeReport for the navigation property onlineMeetings in users.
    *
    * This function performs a `PUT` to the `/users/{user-id}/onlineMeetings/{onlineMeeting-id}/attendeeReport` endpoint.
    *
    * The content stream of the attendee report of a Microsoft Teams live event. Read-only.
    */
    pub async fn users_update_online_meetings_attendee_report<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        online_meeting_id: &str,
        body: B,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onlineMeetings/{}/attendeeReport",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&online_meeting_id.to_string()),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(body.into()),
                    content_type: Some("application/octet-stream".to_string()),
                },
            )
            .await
    }
    /**
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/onlineMeetings/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_get_count_1fc_7(
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
                "/users/{}/onlineMeetings/$count?{}",
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
