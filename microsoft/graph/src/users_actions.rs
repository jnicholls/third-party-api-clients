use crate::Client;
use crate::ClientResult;

pub struct UsersActions {
    pub client: Client,
}

impl UsersActions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersActions { client }
    }

    /**
    * Invoke action resetPassword.
    *
    * This function performs a `POST` to the `/users/{user-id}/authentication/methods/{authenticationMethod-id}/resetPassword` endpoint.
    *
    * Reset a user's password, represented by a password authentication method object. This can only be done by an administrator with appropriate permissions and cannot be performed on a user's own account. This flow writes the new password to Azure Active Directory and pushes it to on-premises Active Directory if configured using password writeback. The admin can either provide a new password or have the system generate one. The user is prompted to change their password on their next sign in. This reset is a long-running operation and will return a **Location** header with a link where the caller can periodically check for the status of the reset operation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/authenticationmethod-resetpassword?view=graph-rest-1.0>
    */
    pub async fn users_user_authentication_methods_method_reset_password(
        &self,
        user_id: &str,
        authentication_method_id: &str,
        body: &crate::types::UsersUserAuthenticationMethodsMethodResetPasswordRequest,
    ) -> ClientResult<crate::types::UsersUserAuthenticationMethodsMethodResetPasswordResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/authentication/methods/{}/resetPassword",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&authentication_method_id.to_string()),
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
    * Invoke action disableSmsSignIn.
    *
    * This function performs a `POST` to the `/users/{user-id}/authentication/phoneMethods/{phoneAuthenticationMethod-id}/disableSmsSignIn` endpoint.
    *
    * Disable SMS sign-in for an existing `mobile` phone number registered to a user. The number will no longer be available for SMS sign-in, which can prevent your user from signing in.
    *
    * FROM: <https://docs.microsoft.com/graph/api/phoneauthenticationmethod-disablesmssignin?view=graph-rest-1.0>
    */
    pub async fn users_user_authentication_phone_methods_method_disable_sms_sign(
        &self,
        user_id: &str,
        phone_authentication_method_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/authentication/phoneMethods/{}/disableSmsSignIn",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&phone_authentication_method_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action enableSmsSignIn.
    *
    * This function performs a `POST` to the `/users/{user-id}/authentication/phoneMethods/{phoneAuthenticationMethod-id}/enableSmsSignIn` endpoint.
    *
    * Enable SMS sign-in for an existing `mobile` phone number registered to a user. To be successfully enabled:
    *
    * FROM: <https://docs.microsoft.com/graph/api/phoneauthenticationmethod-enablesmssignin?view=graph-rest-1.0>
    */
    pub async fn users_user_authentication_phone_methods_method_enable_sms_sign(
        &self,
        user_id: &str,
        phone_authentication_method_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/authentication/phoneMethods/{}/enableSmsSignIn",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&phone_authentication_method_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_attachments_create_upload_session(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/attachments/createUploadSession",
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_instances_attachments_create_upload_session(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/attachments/createUploadSession",
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
    * Invoke action accept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/accept` endpoint.
    *
    * Accept the specified event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-accept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_instances_accept(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventAcceptRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/accept",
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
    * Invoke action cancel.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/cancel` endpoint.
    *
    * This action allows the organizer of a meeting to send a cancellation message and cancel the event.  The action moves the event to the Deleted Items folder. The organizer can also cancel an occurrence of a recurring meeting
    * by providing the occurrence event ID. An attendee calling this action gets an error (HTTP 400 Bad Request), with the following
    * error message: 'Your request can't be completed. You need to be an organizer to cancel a meeting.' This action differs from Delete in that **Cancel** is available to only the organizer, and lets
    * the organizer send a custom message to the attendees about the cancellation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-cancel?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_instances_cancel(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventCancelRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/cancel",
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
    * Invoke action decline.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/decline` endpoint.
    *
    * Decline invitation to the specified event in a user calendar. If the event allows proposals for new times, on declining the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-decline?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_instances_decline(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/decline",
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
    * Invoke action dismissReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/dismissReminder` endpoint.
    *
    * Dismiss a reminder that has been triggered for an event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-dismissreminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_instances_dismiss_reminder(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/dismissReminder",
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
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/forward` endpoint.
    *
    * This action allows the organizer or attendee of a meeting event to forward the
    * meeting request to a new recipient.  If the meeting event is forwarded from an attendee's Microsoft 365 mailbox to another recipient, this action
    * also sends a message to notify the organizer of the forwarding, and adds the recipient to the organizer's
    * copy of the meeting event. This convenience is not available when forwarding from an Outlook.com account.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_instances_forward(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/forward",
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
    * Invoke action snoozeReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/snoozeReminder` endpoint.
    *
    * Postpone a reminder for an event in a user calendar until a new time.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-snoozereminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_instances_snooze_reminder(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventSnoozeReminderRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/snoozeReminder",
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
    * Invoke action tentativelyAccept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/instances/{event-id1}/tentativelyAccept` endpoint.
    *
    * Tentatively accept the specified event in a user calendar. If the event allows proposals for new times, on responding tentative to the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-tentativelyaccept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_instances_tentatively_accept(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/instances/{}/tentativelyAccept",
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
    * Invoke action accept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/accept` endpoint.
    *
    * Accept the specified event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-accept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_accept(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventAcceptRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/accept",
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
    * Invoke action cancel.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/cancel` endpoint.
    *
    * This action allows the organizer of a meeting to send a cancellation message and cancel the event.  The action moves the event to the Deleted Items folder. The organizer can also cancel an occurrence of a recurring meeting
    * by providing the occurrence event ID. An attendee calling this action gets an error (HTTP 400 Bad Request), with the following
    * error message: 'Your request can't be completed. You need to be an organizer to cancel a meeting.' This action differs from Delete in that **Cancel** is available to only the organizer, and lets
    * the organizer send a custom message to the attendees about the cancellation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-cancel?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_cancel(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventCancelRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/cancel",
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
    * Invoke action decline.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/decline` endpoint.
    *
    * Decline invitation to the specified event in a user calendar. If the event allows proposals for new times, on declining the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-decline?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_decline(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/decline",
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
    * Invoke action dismissReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/dismissReminder` endpoint.
    *
    * Dismiss a reminder that has been triggered for an event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-dismissreminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_dismiss_reminder(
        &self,
        user_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/dismissReminder",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/forward` endpoint.
    *
    * This action allows the organizer or attendee of a meeting event to forward the
    * meeting request to a new recipient.  If the meeting event is forwarded from an attendee's Microsoft 365 mailbox to another recipient, this action
    * also sends a message to notify the organizer of the forwarding, and adds the recipient to the organizer's
    * copy of the meeting event. This convenience is not available when forwarding from an Outlook.com account.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_forward(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/forward",
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
    * Invoke action snoozeReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/snoozeReminder` endpoint.
    *
    * Postpone a reminder for an event in a user calendar until a new time.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-snoozereminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_snooze_reminder(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventSnoozeReminderRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/snoozeReminder",
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
    * Invoke action tentativelyAccept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/calendarView/{event-id}/tentativelyAccept` endpoint.
    *
    * Tentatively accept the specified event in a user calendar. If the event allows proposals for new times, on responding tentative to the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-tentativelyaccept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_tentatively_accept(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/calendarView/{}/tentativelyAccept",
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_events_event_attachments_create_upload_session(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/attachments/createUploadSession",
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_events_event_instances_attachments_create_upload_session(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/attachments/createUploadSession",
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
    * Invoke action accept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/accept` endpoint.
    *
    * Accept the specified event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-accept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_events_event_instances_accept(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventAcceptRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/accept",
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
    * Invoke action cancel.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/cancel` endpoint.
    *
    * This action allows the organizer of a meeting to send a cancellation message and cancel the event.  The action moves the event to the Deleted Items folder. The organizer can also cancel an occurrence of a recurring meeting
    * by providing the occurrence event ID. An attendee calling this action gets an error (HTTP 400 Bad Request), with the following
    * error message: 'Your request can't be completed. You need to be an organizer to cancel a meeting.' This action differs from Delete in that **Cancel** is available to only the organizer, and lets
    * the organizer send a custom message to the attendees about the cancellation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-cancel?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_events_event_instances_cancel(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventCancelRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/cancel",
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
    * Invoke action decline.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/decline` endpoint.
    *
    * Decline invitation to the specified event in a user calendar. If the event allows proposals for new times, on declining the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-decline?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_events_event_instances_decline(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/decline",
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
    * Invoke action dismissReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/dismissReminder` endpoint.
    *
    * Dismiss a reminder that has been triggered for an event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-dismissreminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_events_event_instances_dismiss_reminder(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/dismissReminder",
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
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/forward` endpoint.
    *
    * This action allows the organizer or attendee of a meeting event to forward the
    * meeting request to a new recipient.  If the meeting event is forwarded from an attendee's Microsoft 365 mailbox to another recipient, this action
    * also sends a message to notify the organizer of the forwarding, and adds the recipient to the organizer's
    * copy of the meeting event. This convenience is not available when forwarding from an Outlook.com account.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_events_event_instances_forward(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/forward",
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
    * Invoke action snoozeReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/snoozeReminder` endpoint.
    *
    * Postpone a reminder for an event in a user calendar until a new time.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-snoozereminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_events_event_instances_snooze_reminder(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventSnoozeReminderRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/snoozeReminder",
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
    * Invoke action tentativelyAccept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/instances/{event-id1}/tentativelyAccept` endpoint.
    *
    * Tentatively accept the specified event in a user calendar. If the event allows proposals for new times, on responding tentative to the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-tentativelyaccept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_events_event_instances_tentatively_accept(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/instances/{}/tentativelyAccept",
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
    * Invoke action accept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/accept` endpoint.
    *
    * Accept the specified event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-accept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_events_event_accept(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventAcceptRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/accept",
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
    * Invoke action cancel.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/cancel` endpoint.
    *
    * This action allows the organizer of a meeting to send a cancellation message and cancel the event.  The action moves the event to the Deleted Items folder. The organizer can also cancel an occurrence of a recurring meeting
    * by providing the occurrence event ID. An attendee calling this action gets an error (HTTP 400 Bad Request), with the following
    * error message: 'Your request can't be completed. You need to be an organizer to cancel a meeting.' This action differs from Delete in that **Cancel** is available to only the organizer, and lets
    * the organizer send a custom message to the attendees about the cancellation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-cancel?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_events_event_cancel(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventCancelRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/cancel",
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
    * Invoke action decline.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/decline` endpoint.
    *
    * Decline invitation to the specified event in a user calendar. If the event allows proposals for new times, on declining the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-decline?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_events_event_decline(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/decline",
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
    * Invoke action dismissReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/dismissReminder` endpoint.
    *
    * Dismiss a reminder that has been triggered for an event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-dismissreminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_events_event_dismiss_reminder(
        &self,
        user_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/dismissReminder",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/forward` endpoint.
    *
    * This action allows the organizer or attendee of a meeting event to forward the
    * meeting request to a new recipient.  If the meeting event is forwarded from an attendee's Microsoft 365 mailbox to another recipient, this action
    * also sends a message to notify the organizer of the forwarding, and adds the recipient to the organizer's
    * copy of the meeting event. This convenience is not available when forwarding from an Outlook.com account.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_events_event_forward(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/forward",
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
    * Invoke action snoozeReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/snoozeReminder` endpoint.
    *
    * Postpone a reminder for an event in a user calendar until a new time.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-snoozereminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_events_event_snooze_reminder(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventSnoozeReminderRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/snoozeReminder",
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
    * Invoke action tentativelyAccept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/events/{event-id}/tentativelyAccept` endpoint.
    *
    * Tentatively accept the specified event in a user calendar. If the event allows proposals for new times, on responding tentative to the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-tentativelyaccept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_events_event_tentatively_accept(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/events/{}/tentativelyAccept",
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
    * Invoke action getSchedule.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendar/getSchedule` endpoint.
    *
    * Get the free/busy availability information for a collection of users, distributions lists, or resources (rooms or equipment) for a specified time period.
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendar-getschedule?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_get_schedule(
        &self,
        user_id: &str,
        body: &crate::types::UsersUserCalendarGetScheduleRequest,
    ) -> ClientResult<crate::types::Me> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendar/getSchedule",
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_view_event_attachments_create_upload_session(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/attachments/createUploadSession",
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_view_event_instances_attachments_create_upload_session(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/attachments/createUploadSession",
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
    * Invoke action accept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/accept` endpoint.
    *
    * Accept the specified event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-accept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_view_event_instances_accept(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventAcceptRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/accept",
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
    * Invoke action cancel.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/cancel` endpoint.
    *
    * This action allows the organizer of a meeting to send a cancellation message and cancel the event.  The action moves the event to the Deleted Items folder. The organizer can also cancel an occurrence of a recurring meeting
    * by providing the occurrence event ID. An attendee calling this action gets an error (HTTP 400 Bad Request), with the following
    * error message: 'Your request can't be completed. You need to be an organizer to cancel a meeting.' This action differs from Delete in that **Cancel** is available to only the organizer, and lets
    * the organizer send a custom message to the attendees about the cancellation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-cancel?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_view_event_instances_cancel(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventCancelRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/cancel",
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
    * Invoke action decline.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/decline` endpoint.
    *
    * Decline invitation to the specified event in a user calendar. If the event allows proposals for new times, on declining the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-decline?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_view_event_instances_decline(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/decline",
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
    * Invoke action dismissReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/dismissReminder` endpoint.
    *
    * Dismiss a reminder that has been triggered for an event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-dismissreminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_view_event_instances_dismiss_reminder(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/dismissReminder",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_group_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),), None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/forward` endpoint.
    *
    * This action allows the organizer or attendee of a meeting event to forward the
    * meeting request to a new recipient.  If the meeting event is forwarded from an attendee's Microsoft 365 mailbox to another recipient, this action
    * also sends a message to notify the organizer of the forwarding, and adds the recipient to the organizer's
    * copy of the meeting event. This convenience is not available when forwarding from an Outlook.com account.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_view_event_instances_forward(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/forward",
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
    * Invoke action snoozeReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/snoozeReminder` endpoint.
    *
    * Postpone a reminder for an event in a user calendar until a new time.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-snoozereminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_view_event_instances_snooze_reminder(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventSnoozeReminderRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/snoozeReminder",
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
    * Invoke action tentativelyAccept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/tentativelyAccept` endpoint.
    *
    * Tentatively accept the specified event in a user calendar. If the event allows proposals for new times, on responding tentative to the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-tentativelyaccept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_view_event_instances_tentatively_accept(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/instances/{}/tentativelyAccept",
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
    * Invoke action accept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/accept` endpoint.
    *
    * Accept the specified event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-accept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_view_event_accept(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventAcceptRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/accept",
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
    * Invoke action cancel.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/cancel` endpoint.
    *
    * This action allows the organizer of a meeting to send a cancellation message and cancel the event.  The action moves the event to the Deleted Items folder. The organizer can also cancel an occurrence of a recurring meeting
    * by providing the occurrence event ID. An attendee calling this action gets an error (HTTP 400 Bad Request), with the following
    * error message: 'Your request can't be completed. You need to be an organizer to cancel a meeting.' This action differs from Delete in that **Cancel** is available to only the organizer, and lets
    * the organizer send a custom message to the attendees about the cancellation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-cancel?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_view_event_cancel(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventCancelRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/cancel",
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
    * Invoke action decline.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/decline` endpoint.
    *
    * Decline invitation to the specified event in a user calendar. If the event allows proposals for new times, on declining the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-decline?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_view_event_decline(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/decline",
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
    * Invoke action dismissReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/dismissReminder` endpoint.
    *
    * Dismiss a reminder that has been triggered for an event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-dismissreminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_view_event_dismiss_reminder(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/dismissReminder",
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
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/forward` endpoint.
    *
    * This action allows the organizer or attendee of a meeting event to forward the
    * meeting request to a new recipient.  If the meeting event is forwarded from an attendee's Microsoft 365 mailbox to another recipient, this action
    * also sends a message to notify the organizer of the forwarding, and adds the recipient to the organizer's
    * copy of the meeting event. This convenience is not available when forwarding from an Outlook.com account.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_view_event_forward(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/forward",
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
    * Invoke action snoozeReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/snoozeReminder` endpoint.
    *
    * Postpone a reminder for an event in a user calendar until a new time.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-snoozereminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_view_event_snooze_reminder(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventSnoozeReminderRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/snoozeReminder",
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
    * Invoke action tentativelyAccept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/calendarView/{event-id}/tentativelyAccept` endpoint.
    *
    * Tentatively accept the specified event in a user calendar. If the event allows proposals for new times, on responding tentative to the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-tentativelyaccept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_view_event_tentatively_accept(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/calendarView/{}/tentativelyAccept",
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_events_event_attachments_create_upload_session(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/attachments/createUploadSession",
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_events_event_instances_attachments_create_upload_session(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
&format!("/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/attachments/createUploadSession",
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
    * Invoke action accept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/accept` endpoint.
    *
    * Accept the specified event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-accept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_events_event_instances_accept(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventAcceptRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/accept",
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
    * Invoke action cancel.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/cancel` endpoint.
    *
    * This action allows the organizer of a meeting to send a cancellation message and cancel the event.  The action moves the event to the Deleted Items folder. The organizer can also cancel an occurrence of a recurring meeting
    * by providing the occurrence event ID. An attendee calling this action gets an error (HTTP 400 Bad Request), with the following
    * error message: 'Your request can't be completed. You need to be an organizer to cancel a meeting.' This action differs from Delete in that **Cancel** is available to only the organizer, and lets
    * the organizer send a custom message to the attendees about the cancellation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-cancel?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_events_event_instances_cancel(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventCancelRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/cancel",
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
    * Invoke action decline.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/decline` endpoint.
    *
    * Decline invitation to the specified event in a user calendar. If the event allows proposals for new times, on declining the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-decline?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_events_event_instances_decline(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/decline",
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
    * Invoke action dismissReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/dismissReminder` endpoint.
    *
    * Dismiss a reminder that has been triggered for an event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-dismissreminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_events_event_instances_dismiss_reminder(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/dismissReminder",
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
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/forward` endpoint.
    *
    * This action allows the organizer or attendee of a meeting event to forward the
    * meeting request to a new recipient.  If the meeting event is forwarded from an attendee's Microsoft 365 mailbox to another recipient, this action
    * also sends a message to notify the organizer of the forwarding, and adds the recipient to the organizer's
    * copy of the meeting event. This convenience is not available when forwarding from an Outlook.com account.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_events_event_instances_forward(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/forward",
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
    * Invoke action snoozeReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/snoozeReminder` endpoint.
    *
    * Postpone a reminder for an event in a user calendar until a new time.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-snoozereminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_events_event_instances_snooze_reminder(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventSnoozeReminderRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/snoozeReminder",
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
    * Invoke action tentativelyAccept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/tentativelyAccept` endpoint.
    *
    * Tentatively accept the specified event in a user calendar. If the event allows proposals for new times, on responding tentative to the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-tentativelyaccept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_events_event_instances_tentatively_accept(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/instances/{}/tentativelyAccept",
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
    * Invoke action accept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/accept` endpoint.
    *
    * Accept the specified event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-accept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_events_event_accept(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventAcceptRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/accept",
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
    * Invoke action cancel.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/cancel` endpoint.
    *
    * This action allows the organizer of a meeting to send a cancellation message and cancel the event.  The action moves the event to the Deleted Items folder. The organizer can also cancel an occurrence of a recurring meeting
    * by providing the occurrence event ID. An attendee calling this action gets an error (HTTP 400 Bad Request), with the following
    * error message: 'Your request can't be completed. You need to be an organizer to cancel a meeting.' This action differs from Delete in that **Cancel** is available to only the organizer, and lets
    * the organizer send a custom message to the attendees about the cancellation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-cancel?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_events_event_cancel(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventCancelRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/cancel",
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
    * Invoke action decline.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/decline` endpoint.
    *
    * Decline invitation to the specified event in a user calendar. If the event allows proposals for new times, on declining the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-decline?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_events_event_decline(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/decline",
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
    * Invoke action dismissReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/dismissReminder` endpoint.
    *
    * Dismiss a reminder that has been triggered for an event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-dismissreminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_events_event_dismiss_reminder(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/dismissReminder",
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
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/forward` endpoint.
    *
    * This action allows the organizer or attendee of a meeting event to forward the
    * meeting request to a new recipient.  If the meeting event is forwarded from an attendee's Microsoft 365 mailbox to another recipient, this action
    * also sends a message to notify the organizer of the forwarding, and adds the recipient to the organizer's
    * copy of the meeting event. This convenience is not available when forwarding from an Outlook.com account.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_events_event_forward(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/forward",
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
    * Invoke action snoozeReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/snoozeReminder` endpoint.
    *
    * Postpone a reminder for an event in a user calendar until a new time.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-snoozereminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_events_event_snooze_reminder(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventSnoozeReminderRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/snoozeReminder",
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
    * Invoke action tentativelyAccept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/events/{event-id}/tentativelyAccept` endpoint.
    *
    * Tentatively accept the specified event in a user calendar. If the event allows proposals for new times, on responding tentative to the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-tentativelyaccept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_events_event_tentatively_accept(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/events/{}/tentativelyAccept",
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
    * Invoke action getSchedule.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarGroups/{calendarGroup-id}/calendars/{calendar-id}/getSchedule` endpoint.
    *
    * Get the free/busy availability information for a collection of users, distributions lists, or resources (rooms or equipment) for a specified time period.
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendar-getschedule?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_groups_group_calendars_get_schedule(
        &self,
        user_id: &str,
        calendar_group_id: &str,
        calendar_id: &str,
        body: &crate::types::UsersUserCalendarGetScheduleRequest,
    ) -> ClientResult<crate::types::Me> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarGroups/{}/calendars/{}/getSchedule",
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_view_event_attachments_create_upload_session(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/attachments/createUploadSession",
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_view_event_instances_attachments_create_upload_session(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
&format!("/users/{}/calendars/{}/calendarView/{}/instances/{}/attachments/createUploadSession",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&event_id.to_string()),crate::progenitor_support::encode_path(&event_id_1.to_string()),), None);
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
    * Invoke action accept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/accept` endpoint.
    *
    * Accept the specified event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-accept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_view_event_instances_accept(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventAcceptRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/accept",
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
    * Invoke action cancel.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/cancel` endpoint.
    *
    * This action allows the organizer of a meeting to send a cancellation message and cancel the event.  The action moves the event to the Deleted Items folder. The organizer can also cancel an occurrence of a recurring meeting
    * by providing the occurrence event ID. An attendee calling this action gets an error (HTTP 400 Bad Request), with the following
    * error message: 'Your request can't be completed. You need to be an organizer to cancel a meeting.' This action differs from Delete in that **Cancel** is available to only the organizer, and lets
    * the organizer send a custom message to the attendees about the cancellation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-cancel?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_view_event_instances_cancel(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventCancelRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/cancel",
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
    * Invoke action decline.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/decline` endpoint.
    *
    * Decline invitation to the specified event in a user calendar. If the event allows proposals for new times, on declining the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-decline?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_view_event_instances_decline(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/decline",
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
    * Invoke action dismissReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/dismissReminder` endpoint.
    *
    * Dismiss a reminder that has been triggered for an event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-dismissreminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_view_event_instances_dismiss_reminder(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/dismissReminder",
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
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/forward` endpoint.
    *
    * This action allows the organizer or attendee of a meeting event to forward the
    * meeting request to a new recipient.  If the meeting event is forwarded from an attendee's Microsoft 365 mailbox to another recipient, this action
    * also sends a message to notify the organizer of the forwarding, and adds the recipient to the organizer's
    * copy of the meeting event. This convenience is not available when forwarding from an Outlook.com account.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_view_event_instances_forward(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/forward",
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
    * Invoke action snoozeReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/snoozeReminder` endpoint.
    *
    * Postpone a reminder for an event in a user calendar until a new time.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-snoozereminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_view_event_instances_snooze_reminder(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventSnoozeReminderRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/snoozeReminder",
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
    * Invoke action tentativelyAccept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/instances/{event-id1}/tentativelyAccept` endpoint.
    *
    * Tentatively accept the specified event in a user calendar. If the event allows proposals for new times, on responding tentative to the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-tentativelyaccept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_view_event_instances_tentatively_accept(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/instances/{}/tentativelyAccept",
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
    * Invoke action accept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/accept` endpoint.
    *
    * Accept the specified event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-accept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_view_event_accept(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventAcceptRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/accept",
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
    * Invoke action cancel.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/cancel` endpoint.
    *
    * This action allows the organizer of a meeting to send a cancellation message and cancel the event.  The action moves the event to the Deleted Items folder. The organizer can also cancel an occurrence of a recurring meeting
    * by providing the occurrence event ID. An attendee calling this action gets an error (HTTP 400 Bad Request), with the following
    * error message: 'Your request can't be completed. You need to be an organizer to cancel a meeting.' This action differs from Delete in that **Cancel** is available to only the organizer, and lets
    * the organizer send a custom message to the attendees about the cancellation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-cancel?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_view_event_cancel(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventCancelRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/cancel",
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
    * Invoke action decline.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/decline` endpoint.
    *
    * Decline invitation to the specified event in a user calendar. If the event allows proposals for new times, on declining the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-decline?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_view_event_decline(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/decline",
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
    * Invoke action dismissReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/dismissReminder` endpoint.
    *
    * Dismiss a reminder that has been triggered for an event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-dismissreminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_view_event_dismiss_reminder(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/dismissReminder",
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
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/forward` endpoint.
    *
    * This action allows the organizer or attendee of a meeting event to forward the
    * meeting request to a new recipient.  If the meeting event is forwarded from an attendee's Microsoft 365 mailbox to another recipient, this action
    * also sends a message to notify the organizer of the forwarding, and adds the recipient to the organizer's
    * copy of the meeting event. This convenience is not available when forwarding from an Outlook.com account.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_view_event_forward(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/forward",
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
    * Invoke action snoozeReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/snoozeReminder` endpoint.
    *
    * Postpone a reminder for an event in a user calendar until a new time.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-snoozereminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_view_event_snooze_reminder(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventSnoozeReminderRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/snoozeReminder",
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
    * Invoke action tentativelyAccept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/calendarView/{event-id}/tentativelyAccept` endpoint.
    *
    * Tentatively accept the specified event in a user calendar. If the event allows proposals for new times, on responding tentative to the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-tentativelyaccept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_view_event_tentatively_accept(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/calendarView/{}/tentativelyAccept",
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_events_event_attachments_create_upload_session(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/attachments/createUploadSession",
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_events_event_instances_attachments_create_upload_session(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/attachments/createUploadSession",
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
    * Invoke action accept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/accept` endpoint.
    *
    * Accept the specified event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-accept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_events_event_instances_accept(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventAcceptRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/accept",
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
    * Invoke action cancel.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/cancel` endpoint.
    *
    * This action allows the organizer of a meeting to send a cancellation message and cancel the event.  The action moves the event to the Deleted Items folder. The organizer can also cancel an occurrence of a recurring meeting
    * by providing the occurrence event ID. An attendee calling this action gets an error (HTTP 400 Bad Request), with the following
    * error message: 'Your request can't be completed. You need to be an organizer to cancel a meeting.' This action differs from Delete in that **Cancel** is available to only the organizer, and lets
    * the organizer send a custom message to the attendees about the cancellation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-cancel?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_events_event_instances_cancel(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventCancelRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/cancel",
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
    * Invoke action decline.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/decline` endpoint.
    *
    * Decline invitation to the specified event in a user calendar. If the event allows proposals for new times, on declining the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-decline?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_events_event_instances_decline(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/decline",
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
    * Invoke action dismissReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/dismissReminder` endpoint.
    *
    * Dismiss a reminder that has been triggered for an event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-dismissreminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_events_event_instances_dismiss_reminder(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/dismissReminder",
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
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/forward` endpoint.
    *
    * This action allows the organizer or attendee of a meeting event to forward the
    * meeting request to a new recipient.  If the meeting event is forwarded from an attendee's Microsoft 365 mailbox to another recipient, this action
    * also sends a message to notify the organizer of the forwarding, and adds the recipient to the organizer's
    * copy of the meeting event. This convenience is not available when forwarding from an Outlook.com account.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_events_event_instances_forward(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/forward",
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
    * Invoke action snoozeReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/snoozeReminder` endpoint.
    *
    * Postpone a reminder for an event in a user calendar until a new time.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-snoozereminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_events_event_instances_snooze_reminder(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventSnoozeReminderRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/snoozeReminder",
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
    * Invoke action tentativelyAccept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/instances/{event-id1}/tentativelyAccept` endpoint.
    *
    * Tentatively accept the specified event in a user calendar. If the event allows proposals for new times, on responding tentative to the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-tentativelyaccept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_events_event_instances_tentatively_accept(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/instances/{}/tentativelyAccept",
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
    * Invoke action accept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/accept` endpoint.
    *
    * Accept the specified event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-accept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_events_event_accept(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventAcceptRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/accept",
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
    * Invoke action cancel.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/cancel` endpoint.
    *
    * This action allows the organizer of a meeting to send a cancellation message and cancel the event.  The action moves the event to the Deleted Items folder. The organizer can also cancel an occurrence of a recurring meeting
    * by providing the occurrence event ID. An attendee calling this action gets an error (HTTP 400 Bad Request), with the following
    * error message: 'Your request can't be completed. You need to be an organizer to cancel a meeting.' This action differs from Delete in that **Cancel** is available to only the organizer, and lets
    * the organizer send a custom message to the attendees about the cancellation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-cancel?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_events_event_cancel(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventCancelRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/cancel",
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
    * Invoke action decline.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/decline` endpoint.
    *
    * Decline invitation to the specified event in a user calendar. If the event allows proposals for new times, on declining the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-decline?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_events_event_decline(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/decline",
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
    * Invoke action dismissReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/dismissReminder` endpoint.
    *
    * Dismiss a reminder that has been triggered for an event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-dismissreminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_events_event_dismiss_reminder(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/dismissReminder",
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
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/forward` endpoint.
    *
    * This action allows the organizer or attendee of a meeting event to forward the
    * meeting request to a new recipient.  If the meeting event is forwarded from an attendee's Microsoft 365 mailbox to another recipient, this action
    * also sends a message to notify the organizer of the forwarding, and adds the recipient to the organizer's
    * copy of the meeting event. This convenience is not available when forwarding from an Outlook.com account.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_events_event_forward(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/forward",
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
    * Invoke action snoozeReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/snoozeReminder` endpoint.
    *
    * Postpone a reminder for an event in a user calendar until a new time.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-snoozereminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_events_event_snooze_reminder(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventSnoozeReminderRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/snoozeReminder",
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
    * Invoke action tentativelyAccept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/events/{event-id}/tentativelyAccept` endpoint.
    *
    * Tentatively accept the specified event in a user calendar. If the event allows proposals for new times, on responding tentative to the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-tentativelyaccept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_events_event_tentatively_accept(
        &self,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/events/{}/tentativelyAccept",
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
    * Invoke action getSchedule.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendars/{calendar-id}/getSchedule` endpoint.
    *
    * Get the free/busy availability information for a collection of users, distributions lists, or resources (rooms or equipment) for a specified time period.
    *
    * FROM: <https://docs.microsoft.com/graph/api/calendar-getschedule?view=graph-rest-1.0>
    */
    pub async fn users_user_calendars_calendar_get_schedule(
        &self,
        user_id: &str,
        calendar_id: &str,
        body: &crate::types::UsersUserCalendarGetScheduleRequest,
    ) -> ClientResult<crate::types::Me> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendars/{}/getSchedule",
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_attachments_create_upload_session_users_actions(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/attachments/createUploadSession",
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_instances_attachments_create_upload_session_users_actions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/attachments/createUploadSession",
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
    * Invoke action accept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/accept` endpoint.
    *
    * Accept the specified event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-accept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_instances_accept_users_actions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventAcceptRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/accept",
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
    * Invoke action cancel.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/cancel` endpoint.
    *
    * This action allows the organizer of a meeting to send a cancellation message and cancel the event.  The action moves the event to the Deleted Items folder. The organizer can also cancel an occurrence of a recurring meeting
    * by providing the occurrence event ID. An attendee calling this action gets an error (HTTP 400 Bad Request), with the following
    * error message: 'Your request can't be completed. You need to be an organizer to cancel a meeting.' This action differs from Delete in that **Cancel** is available to only the organizer, and lets
    * the organizer send a custom message to the attendees about the cancellation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-cancel?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_instances_cancel_users_actions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventCancelRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/cancel",
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
    * Invoke action decline.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/decline` endpoint.
    *
    * Decline invitation to the specified event in a user calendar. If the event allows proposals for new times, on declining the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-decline?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_instances_decline_users_actions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/decline",
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
    * Invoke action dismissReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/dismissReminder` endpoint.
    *
    * Dismiss a reminder that has been triggered for an event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-dismissreminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_instances_dismiss_reminder_users_actions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/dismissReminder",
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
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/forward` endpoint.
    *
    * This action allows the organizer or attendee of a meeting event to forward the
    * meeting request to a new recipient.  If the meeting event is forwarded from an attendee's Microsoft 365 mailbox to another recipient, this action
    * also sends a message to notify the organizer of the forwarding, and adds the recipient to the organizer's
    * copy of the meeting event. This convenience is not available when forwarding from an Outlook.com account.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_instances_forward_users_actions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/forward",
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
    * Invoke action snoozeReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/snoozeReminder` endpoint.
    *
    * Postpone a reminder for an event in a user calendar until a new time.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-snoozereminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_instances_snooze_reminder_users_actions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventSnoozeReminderRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/snoozeReminder",
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
    * Invoke action tentativelyAccept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/instances/{event-id1}/tentativelyAccept` endpoint.
    *
    * Tentatively accept the specified event in a user calendar. If the event allows proposals for new times, on responding tentative to the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-tentativelyaccept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_instances_tentatively_accept_users_actions(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/instances/{}/tentativelyAccept",
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
    * Invoke action accept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/accept` endpoint.
    *
    * Accept the specified event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-accept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_accept_users_actions(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventAcceptRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/accept",
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
    * Invoke action cancel.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/cancel` endpoint.
    *
    * This action allows the organizer of a meeting to send a cancellation message and cancel the event.  The action moves the event to the Deleted Items folder. The organizer can also cancel an occurrence of a recurring meeting
    * by providing the occurrence event ID. An attendee calling this action gets an error (HTTP 400 Bad Request), with the following
    * error message: 'Your request can't be completed. You need to be an organizer to cancel a meeting.' This action differs from Delete in that **Cancel** is available to only the organizer, and lets
    * the organizer send a custom message to the attendees about the cancellation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-cancel?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_cancel_users_actions(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventCancelRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/cancel",
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
    * Invoke action decline.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/decline` endpoint.
    *
    * Decline invitation to the specified event in a user calendar. If the event allows proposals for new times, on declining the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-decline?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_decline_users_actions(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/decline",
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
    * Invoke action dismissReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/dismissReminder` endpoint.
    *
    * Dismiss a reminder that has been triggered for an event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-dismissreminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_dismiss_reminder_users_actions(
        &self,
        user_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/dismissReminder",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/forward` endpoint.
    *
    * This action allows the organizer or attendee of a meeting event to forward the
    * meeting request to a new recipient.  If the meeting event is forwarded from an attendee's Microsoft 365 mailbox to another recipient, this action
    * also sends a message to notify the organizer of the forwarding, and adds the recipient to the organizer's
    * copy of the meeting event. This convenience is not available when forwarding from an Outlook.com account.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_forward_users_actions(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/forward",
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
    * Invoke action snoozeReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/snoozeReminder` endpoint.
    *
    * Postpone a reminder for an event in a user calendar until a new time.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-snoozereminder?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_snooze_reminder_users_actions(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventSnoozeReminderRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/snoozeReminder",
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
    * Invoke action tentativelyAccept.
    *
    * This function performs a `POST` to the `/users/{user-id}/calendarView/{event-id}/tentativelyAccept` endpoint.
    *
    * Tentatively accept the specified event in a user calendar. If the event allows proposals for new times, on responding tentative to the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-tentativelyaccept?view=graph-rest-1.0>
    */
    pub async fn users_user_calendar_view_event_tentatively_accept_users_actions(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/calendarView/{}/tentativelyAccept",
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
    * Invoke action upgrade.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/installedApps/{teamsAppInstallation-id}/upgrade` endpoint.
    *
    * Upgrade an app installation within a chat.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-teamsappinstallation-upgrade?view=graph-rest-1.0>
    */
    pub async fn users_user_chats_chat_installed_apps_teams_app_installation_upgrade(
        &self,
        user_id: &str,
        chat_id: &str,
        teams_app_installation_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/installedApps/{}/upgrade",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&teams_app_installation_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action add.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/members/add` endpoint.
    *
    * Add multiple members in a single request to a team. The response provides details about which memberships could and couldn't be created.
    *
    * FROM: <https://docs.microsoft.com/graph/api/conversationmembers-add?view=graph-rest-1.0>
    */
    pub async fn users_user_chats_chat_members_add(
        &self,
        user_id: &str,
        chat_id: &str,
        body: &crate::types::UsersUserChatsChatMembersAddRequest,
    ) -> ClientResult<crate::types::Me> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/members/add",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * Invoke action softDelete.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/softDelete` endpoint.
    */
    pub async fn users_user_chats_chat_messages_message_soft_delete(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}/softDelete",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action undoSoftDelete.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/undoSoftDelete` endpoint.
    */
    pub async fn users_user_chats_chat_messages_message_undo_soft_delete(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}/undoSoftDelete",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action softDelete.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/softDelete` endpoint.
    */
    pub async fn users_user_chats_chat_messages_message_replies_soft_delete(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}/replies/{}/softDelete",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action undoSoftDelete.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/undoSoftDelete` endpoint.
    */
    pub async fn users_user_chats_chat_messages_message_replies_undo_soft_delete(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}/replies/{}/undoSoftDelete",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action hideForUser.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/hideForUser` endpoint.
    *
    * Hide a chat for a user.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-hideforuser?view=graph-rest-1.0>
    */
    pub async fn users_user_chats_chat_hide_for(
        &self,
        user_id: &str,
        chat_id: &str,
        body: &crate::types::UsersUserChatsChatHideRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/hideForUser",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * Invoke action markChatReadForUser.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/markChatReadForUser` endpoint.
    *
    * Mark a chat as read for a user.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-markchatreadforuser?view=graph-rest-1.0>
    */
    pub async fn users_user_chats_chat_mark_read_for(
        &self,
        user_id: &str,
        chat_id: &str,
        body: &crate::types::UsersUserChatsChatHideRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/markChatReadForUser",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * Invoke action markChatUnreadForUser.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/markChatUnreadForUser` endpoint.
    *
    * Mark a chat as unread for a user.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-markchatunreadforuser?view=graph-rest-1.0>
    */
    pub async fn users_user_chats_chat_mark_unread_for(
        &self,
        user_id: &str,
        chat_id: &str,
        body: &crate::types::UsersUserChatsChatMarkUnreadRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/markChatUnreadForUser",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * Invoke action sendActivityNotification.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/sendActivityNotification` endpoint.
    *
    * Send an activity feed notification in scope of a chat. For more details about sending notifications and the requirements for doing so, see sending Teams activity notifications.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-sendactivitynotification?view=graph-rest-1.0>
    */
    pub async fn users_user_chats_chat_send_activity_notification(
        &self,
        user_id: &str,
        chat_id: &str,
        body: &crate::types::UsersUserChatsChatSendActivityNotificationRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/sendActivityNotification",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * Invoke action unhideForUser.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/unhideForUser` endpoint.
    *
    * Unhide a chat for a user.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-unhideforuser?view=graph-rest-1.0>
    */
    pub async fn users_user_chats_chat_unhide_for(
        &self,
        user_id: &str,
        chat_id: &str,
        body: &crate::types::UsersUserChatsChatHideRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/unhideForUser",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_events_event_attachments_create_upload_session(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/attachments/createUploadSession",
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_events_event_instances_attachments_create_upload_session(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/attachments/createUploadSession",
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
    * Invoke action accept.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/accept` endpoint.
    *
    * Accept the specified event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-accept?view=graph-rest-1.0>
    */
    pub async fn users_user_events_event_instances_accept(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventAcceptRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/accept",
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
    * Invoke action cancel.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/cancel` endpoint.
    *
    * This action allows the organizer of a meeting to send a cancellation message and cancel the event.  The action moves the event to the Deleted Items folder. The organizer can also cancel an occurrence of a recurring meeting
    * by providing the occurrence event ID. An attendee calling this action gets an error (HTTP 400 Bad Request), with the following
    * error message: 'Your request can't be completed. You need to be an organizer to cancel a meeting.' This action differs from Delete in that **Cancel** is available to only the organizer, and lets
    * the organizer send a custom message to the attendees about the cancellation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-cancel?view=graph-rest-1.0>
    */
    pub async fn users_user_events_event_instances_cancel(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventCancelRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/cancel",
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
    * Invoke action decline.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/decline` endpoint.
    *
    * Decline invitation to the specified event in a user calendar. If the event allows proposals for new times, on declining the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-decline?view=graph-rest-1.0>
    */
    pub async fn users_user_events_event_instances_decline(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/decline",
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
    * Invoke action dismissReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/dismissReminder` endpoint.
    *
    * Dismiss a reminder that has been triggered for an event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-dismissreminder?view=graph-rest-1.0>
    */
    pub async fn users_user_events_event_instances_dismiss_reminder(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/dismissReminder",
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
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/forward` endpoint.
    *
    * This action allows the organizer or attendee of a meeting event to forward the
    * meeting request to a new recipient.  If the meeting event is forwarded from an attendee's Microsoft 365 mailbox to another recipient, this action
    * also sends a message to notify the organizer of the forwarding, and adds the recipient to the organizer's
    * copy of the meeting event. This convenience is not available when forwarding from an Outlook.com account.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_events_event_instances_forward(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/forward",
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
    * Invoke action snoozeReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/snoozeReminder` endpoint.
    *
    * Postpone a reminder for an event in a user calendar until a new time.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-snoozereminder?view=graph-rest-1.0>
    */
    pub async fn users_user_events_event_instances_snooze_reminder(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventSnoozeReminderRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/snoozeReminder",
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
    * Invoke action tentativelyAccept.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/instances/{event-id1}/tentativelyAccept` endpoint.
    *
    * Tentatively accept the specified event in a user calendar. If the event allows proposals for new times, on responding tentative to the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-tentativelyaccept?view=graph-rest-1.0>
    */
    pub async fn users_user_events_event_instances_tentatively_accept(
        &self,
        user_id: &str,
        event_id: &str,
        event_id_1: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/instances/{}/tentativelyAccept",
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
    * Invoke action accept.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/accept` endpoint.
    *
    * Accept the specified event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-accept?view=graph-rest-1.0>
    */
    pub async fn users_user_events_event_accept(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventAcceptRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/accept",
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
    * Invoke action cancel.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/cancel` endpoint.
    *
    * This action allows the organizer of a meeting to send a cancellation message and cancel the event.  The action moves the event to the Deleted Items folder. The organizer can also cancel an occurrence of a recurring meeting
    * by providing the occurrence event ID. An attendee calling this action gets an error (HTTP 400 Bad Request), with the following
    * error message: 'Your request can't be completed. You need to be an organizer to cancel a meeting.' This action differs from Delete in that **Cancel** is available to only the organizer, and lets
    * the organizer send a custom message to the attendees about the cancellation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-cancel?view=graph-rest-1.0>
    */
    pub async fn users_user_events_event_cancel(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventCancelRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/cancel",
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
    * Invoke action decline.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/decline` endpoint.
    *
    * Decline invitation to the specified event in a user calendar. If the event allows proposals for new times, on declining the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-decline?view=graph-rest-1.0>
    */
    pub async fn users_user_events_event_decline(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/decline",
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
    * Invoke action dismissReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/dismissReminder` endpoint.
    *
    * Dismiss a reminder that has been triggered for an event in a user calendar.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-dismissreminder?view=graph-rest-1.0>
    */
    pub async fn users_user_events_event_dismiss_reminder(
        &self,
        user_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/dismissReminder",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&event_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/forward` endpoint.
    *
    * This action allows the organizer or attendee of a meeting event to forward the
    * meeting request to a new recipient.  If the meeting event is forwarded from an attendee's Microsoft 365 mailbox to another recipient, this action
    * also sends a message to notify the organizer of the forwarding, and adds the recipient to the organizer's
    * copy of the meeting event. This convenience is not available when forwarding from an Outlook.com account.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_events_event_forward(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/forward",
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
    * Invoke action snoozeReminder.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/snoozeReminder` endpoint.
    *
    * Postpone a reminder for an event in a user calendar until a new time.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-snoozereminder?view=graph-rest-1.0>
    */
    pub async fn users_user_events_event_snooze_reminder(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventSnoozeReminderRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/snoozeReminder",
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
    * Invoke action tentativelyAccept.
    *
    * This function performs a `POST` to the `/users/{user-id}/events/{event-id}/tentativelyAccept` endpoint.
    *
    * Tentatively accept the specified event in a user calendar. If the event allows proposals for new times, on responding tentative to the event, an invitee can choose to suggest an alternative time by including the **proposedNewTime** parameter. For more information on how to propose a time, and how to receive and accept a new time proposal, see Propose new meeting times.
    *
    * FROM: <https://docs.microsoft.com/graph/api/event-tentativelyaccept?view=graph-rest-1.0>
    */
    pub async fn users_user_events_event_tentatively_accept(
        &self,
        user_id: &str,
        event_id: &str,
        body: &crate::types::UsersUserEventsEventDeclineRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/events/{}/tentativelyAccept",
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
    * Invoke action add.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/members/add` endpoint.
    *
    * Add multiple members in a single request to a team. The response provides details about which memberships could and couldn't be created.
    *
    * FROM: <https://docs.microsoft.com/graph/api/conversationmembers-add?view=graph-rest-1.0>
    */
    pub async fn users_user_joined_teams_team_channels_channel_members_add(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        body: &crate::types::UsersUserChatsChatMembersAddRequest,
    ) -> ClientResult<crate::types::Me> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/members/add",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * Invoke action softDelete.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/softDelete` endpoint.
    */
    pub async fn users_user_joined_teams_team_channels_channel_messages_chat_message_soft_delete(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/softDelete",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action undoSoftDelete.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/undoSoftDelete` endpoint.
    */
    pub async fn users_user_joined_teams_team_channels_channel_messages_chat_message_undo_soft_delete(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/undoSoftDelete",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action softDelete.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/softDelete` endpoint.
    */
    pub async fn users_user_joined_teams_team_channels_channel_messages_chat_message_replies_soft_delete(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/replies/{}/softDelete",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action undoSoftDelete.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/undoSoftDelete` endpoint.
    */
    pub async fn users_user_joined_teams_team_channels_channel_messages_chat_message_replies_undo_soft_delete(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/replies/{}/undoSoftDelete",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action completeMigration.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/completeMigration` endpoint.
    *
    * Complete the message migration process by removing `migration mode` from a channel in a team. `Migration mode` is a special state that prevents certain operations, like sending messages and adding members, during the data migration process. After a **completeMigration** request is made, you cannot import additional messages into the team. You can add members to the team after the request returns a successful response.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-completemigration?view=graph-rest-1.0>
    */
    pub async fn users_user_joined_teams_team_channels_channel_complete_migration(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/completeMigration",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action provisionEmail.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/provisionEmail` endpoint.
    *
    * Provision an email address for a channel. Microsoft Teams doesn't automatically provision an email address for a **channel** by default. To have Teams provision an email address, you can call **provisionEmail**, or through the Teams user interface, select **Get email address**, which triggers Teams to generate an email address if it has not already provisioned one. To remove the email address of a **channel**, use the removeEmail method.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-provisionemail?view=graph-rest-1.0>
    */
    pub async fn users_user_joined_teams_team_channels_channel_provision_email(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
    ) -> ClientResult<crate::types::UsersUserJoinedTeamsTeamChannelProvisionEmailResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/provisionEmail",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action removeEmail.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/removeEmail` endpoint.
    *
    * Remove the email address of a channel. You can remove an email address only if it was provisioned using the provisionEmail method or through the Microsoft Teams client.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-removeemail?view=graph-rest-1.0>
    */
    pub async fn users_user_joined_teams_team_channels_channel_remove_email(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/removeEmail",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action upgrade.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/installedApps/{teamsAppInstallation-id}/upgrade` endpoint.
    *
    * Upgrade an app installation within a chat.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-teamsappinstallation-upgrade?view=graph-rest-1.0>
    */
    pub async fn users_user_joined_teams_team_installed_app_installation_upgrade(
        &self,
        user_id: &str,
        team_id: &str,
        teams_app_installation_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/installedApps/{}/upgrade",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&teams_app_installation_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action add.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/members/add` endpoint.
    *
    * Add multiple members in a single request to a team. The response provides details about which memberships could and couldn't be created.
    *
    * FROM: <https://docs.microsoft.com/graph/api/conversationmembers-add?view=graph-rest-1.0>
    */
    pub async fn users_user_joined_teams_team_members_add(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::UsersUserChatsChatMembersAddRequest,
    ) -> ClientResult<crate::types::Me> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/members/add",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Invoke action archive.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/archive` endpoint.
    *
    * Archive the specified team.
    * When a team is archived, users can no longer send or like messages on any channel in the team, edit the team's name, description, or other settings, or in general make most changes to the team.
    * Membership changes to the team continue to be allowed. Archiving is an async operation. A team is archived once the async operation completes successfully, which may occur subsequent to a response from this API. To archive a team, the team and group must have an owner. To restore a team from its archived state, use the API to unarchive.
    *
    * FROM: <https://docs.microsoft.com/graph/api/team-archive?view=graph-rest-1.0>
    */
    pub async fn users_user_joined_teams_team_archive(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::UsersUserJoinedTeamsTeamArchiveRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/archive",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Invoke action clone.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/clone` endpoint.
    *
    * Create a copy of a team. This operation also creates a copy of the corresponding group.
    * You can specify which parts of the team to clone: When tabs are cloned, they are put into an unconfigured state
    * -- they are displayed on the tab bar in Microsoft Teams, and the first time you open them, you'll go through the configuration screen.
    * (If the person opening the tab does not have permission to configure apps, they will see a message explaining that the tab hasn't been configured.) Cloning is a long-running operation.
    * After the POST clone returns, you need to GET the operation to see if it's 'running' or 'succeeded' or 'failed'.
    * You should continue to GET until the status is not 'running'.
    * The recommended delay between GETs is 5 seconds.
    *
    * FROM: <https://docs.microsoft.com/graph/api/team-clone?view=graph-rest-1.0>
    */
    pub async fn users_user_joined_teams_team_clone(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::UsersUserJoinedTeamsTeamCloneRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/clone",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Invoke action completeMigration.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/completeMigration` endpoint.
    *
    * Complete the message migration process by removing `migration mode` from a team. `Migration mode` is a special state where certain operations are barred, like message POST and membership operations during the data migration process. After a **completeMigration** request is made, you cannot import additional messages into the team. You can add members to the team after the request returns a successful response.
    *
    * FROM: <https://docs.microsoft.com/graph/api/team-completemigration?view=graph-rest-1.0>
    */
    pub async fn users_user_joined_teams_team_complete_migration(
        &self,
        user_id: &str,
        team_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/completeMigration",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action sendActivityNotification.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/sendActivityNotification` endpoint.
    *
    * Send an activity feed notification in the scope of a team. For more details about sending notifications and the requirements for doing so, see
    * sending Teams activity notifications.
    *
    * FROM: <https://docs.microsoft.com/graph/api/team-sendactivitynotification?view=graph-rest-1.0>
    */
    pub async fn users_user_joined_teams_team_send_activity_notification(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::UsersUserChatsChatSendActivityNotificationRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/sendActivityNotification",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Invoke action unarchive.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/unarchive` endpoint.
    *
    * Restore an archived team. This restores users' ability to send messages and edit the team, abiding by tenant and team settings. Teams are archived using the archive API. Unarchiving is an async operation. A team is unarchived once the async operation completes successfully, which may occur subsequent to a response from this API.
    *
    * FROM: <https://docs.microsoft.com/graph/api/team-unarchive?view=graph-rest-1.0>
    */
    pub async fn users_user_joined_teams_team_unarchive(
        &self,
        user_id: &str,
        team_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/unarchive",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action add.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/members/add` endpoint.
    *
    * Add multiple members in a single request to a team. The response provides details about which memberships could and couldn't be created.
    *
    * FROM: <https://docs.microsoft.com/graph/api/conversationmembers-add?view=graph-rest-1.0>
    */
    pub async fn users_user_joined_teams_team_primary_channel_members_add(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::UsersUserChatsChatMembersAddRequest,
    ) -> ClientResult<crate::types::Me> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/members/add",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Invoke action softDelete.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/softDelete` endpoint.
    */
    pub async fn users_user_joined_teams_team_primary_channel_messages_chat_message_soft_delete(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/softDelete",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action undoSoftDelete.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/undoSoftDelete` endpoint.
    */
    pub async fn users_user_joined_teams_team_primary_channel_messages_chat_message_undo_soft_delete(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/undoSoftDelete",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action softDelete.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/replies/{chatMessage-id1}/softDelete` endpoint.
    */
    pub async fn users_user_joined_teams_team_primary_channel_messages_chat_message_replies_soft_delete(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/replies/{}/softDelete",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action undoSoftDelete.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/replies/{chatMessage-id1}/undoSoftDelete` endpoint.
    */
    pub async fn users_user_joined_teams_team_primary_channel_messages_chat_message_replies_undo_soft_delete(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/replies/{}/undoSoftDelete",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action completeMigration.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/completeMigration` endpoint.
    *
    * Complete the message migration process by removing `migration mode` from a channel in a team. `Migration mode` is a special state that prevents certain operations, like sending messages and adding members, during the data migration process. After a **completeMigration** request is made, you cannot import additional messages into the team. You can add members to the team after the request returns a successful response.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-completemigration?view=graph-rest-1.0>
    */
    pub async fn users_user_joined_teams_team_primary_channel_complete_migration(
        &self,
        user_id: &str,
        team_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/completeMigration",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action provisionEmail.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/provisionEmail` endpoint.
    *
    * Provision an email address for a channel. Microsoft Teams doesn't automatically provision an email address for a **channel** by default. To have Teams provision an email address, you can call **provisionEmail**, or through the Teams user interface, select **Get email address**, which triggers Teams to generate an email address if it has not already provisioned one. To remove the email address of a **channel**, use the removeEmail method.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-provisionemail?view=graph-rest-1.0>
    */
    pub async fn users_user_joined_teams_team_primary_channel_provision_email(
        &self,
        user_id: &str,
        team_id: &str,
    ) -> ClientResult<crate::types::UsersUserJoinedTeamsTeamChannelProvisionEmailResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/provisionEmail",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action removeEmail.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/removeEmail` endpoint.
    *
    * Remove the email address of a channel. You can remove an email address only if it was provisioned using the provisionEmail method or through the Microsoft Teams client.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-removeemail?view=graph-rest-1.0>
    */
    pub async fn users_user_joined_teams_team_primary_channel_remove_email(
        &self,
        user_id: &str,
        team_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/removeEmail",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action share.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/share` endpoint.
    *
    * Share a schedule time range with schedule members.
    * Make the collections of shift, openshift and timeOff items in the specified time range of the schedule viewable by the specified team members, including employees and managers.
    * Each shift, openshift and timeOff instance in a schedule supports a draft version and a shared version of the item. The draft version is viewable by only managers, and the shared version is viewable by employees and managers. For each shift, openshift and timeOff instance in the specified time range, the share action updates the shared version from the draft version, so that in addition to managers, employees can also view the most current information about the item. The **notifyTeam** parameter further specifies which employees can view the item.
    *
    * FROM: <https://docs.microsoft.com/graph/api/schedule-share?view=graph-rest-1.0>
    */
    pub async fn users_user_joined_teams_team_schedule_share(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::UsersUserJoinedTeamsTeamScheduleShareRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/share",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/childFolders/{mailFolder-id1}/messages/{message-id}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_child_messages_message_attachments_create_upload_session(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        mail_folder_id_1: &str,
        message_id: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
&format!("/users/{}/mailFolders/{}/childFolders/{}/messages/{}/attachments/createUploadSession",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&mail_folder_id.to_string()),crate::progenitor_support::encode_path(&mail_folder_id_1.to_string()),crate::progenitor_support::encode_path(&message_id.to_string()),), None);
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
    * Invoke action copy.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/childFolders/{mailFolder-id1}/messages/{message-id}/copy` endpoint.
    *
    * Copy a message to a folder within the user's mailbox.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-copy?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_child_messages_message_copy(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        mail_folder_id_1: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageCopyRequest,
    ) -> ClientResult<crate::types::MessageAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/childFolders/{}/messages/{}/copy",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id_1.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action createForward.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/childFolders/{mailFolder-id1}/messages/{message-id}/createForward` endpoint.
    *
    * Create a draft to forward an existing message, in either JSON or MIME format. When using JSON format, you can:
    * - Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP 400 Bad Request error.
    * - Specify either the `toRecipients` parameter or the **toRecipients** property of the `message` parameter. Specifying both or specifying neither will return an HTTP 400 Bad Request error.
    * - Update the draft later to add content to the **body** or change other message properties. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. Send the draft message in a subsequent operation. Alternatively, forward a message in a single operation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-createforward?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_child_messages_message_create_forward(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        mail_folder_id_1: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageForwardRequest,
    ) -> ClientResult<crate::types::MessageAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/childFolders/{}/messages/{}/createForward",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id_1.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action createReply.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/childFolders/{mailFolder-id1}/messages/{message-id}/createReply` endpoint.
    *
    * Create a draft to reply to the sender of a message in either JSON or MIME format. When using JSON format:
    * - Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP 400 Bad Request error.
    * - If **replyTo** is specified in the original message, per Internet Message Format (RFC 2822), you should send the reply to the recipients in **replyTo**, and not the recipients in **from**.
    * - You can update the draft later to add reply content to the **body** or change other message properties. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. Send the draft message in a subsequent operation. Alternatively, reply to a message in a single operation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-createreply?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_child_messages_message_create_reply(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        mail_folder_id_1: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageReplyRequest,
    ) -> ClientResult<crate::types::MessageAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/childFolders/{}/messages/{}/createReply",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id_1.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action createReplyAll.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/childFolders/{mailFolder-id1}/messages/{message-id}/createReplyAll` endpoint.
    *
    * Create a draft to reply to the sender and all recipients of a message in either JSON or MIME format.  When using JSON format:
    * - Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP 400 Bad Request error.
    * - If the original message specifies a recipient in the **replyTo** property, per Internet Message Format (RFC 2822), you should send the reply to the recipients in the **replyTo** and **toRecipients** properties, and not the recipients in the **from** and **toRecipients** properties.
    * - You can update the draft later to add reply content to the **body** or change other message properties. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. Send the draft message in a subsequent operation. Alternatively, reply-all to a message in a single action.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-createreplyall?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_child_messages_message_create_reply_all(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        mail_folder_id_1: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageReplyRequest,
    ) -> ClientResult<crate::types::MessageAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/childFolders/{}/messages/{}/createReplyAll",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id_1.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/childFolders/{mailFolder-id1}/messages/{message-id}/forward` endpoint.
    *
    * Forward a message using either JSON or MIME format. When using JSON format, you can:
    * - Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP 400 Bad Request error.
    * - Specify either the `toRecipients` parameter or the **toRecipients** property of the `message` parameter. Specifying both or specifying neither will return an HTTP 400 Bad Request error. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. This method saves the message in the **Sent Items** folder. Alternatively, create a draft to forward a message, and send it later.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_child_messages_message_forward(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        mail_folder_id_1: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/childFolders/{}/messages/{}/forward",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id_1.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action move.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/childFolders/{mailFolder-id1}/messages/{message-id}/move` endpoint.
    *
    * Move a message to another folder within the specified user's mailbox. This creates a new copy of the message in the destination folder and removes the original message.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-move?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_child_messages_message_move(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        mail_folder_id_1: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageCopyRequest,
    ) -> ClientResult<crate::types::MessageAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/childFolders/{}/messages/{}/move",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id_1.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action reply.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/childFolders/{mailFolder-id1}/messages/{message-id}/reply` endpoint.
    *
    * Reply to the sender of a message using either JSON or MIME format. When using JSON format:
    * * Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP `400 Bad Request` error.
    * * If the original message specifies a recipient in the **replyTo** property, per Internet Message Format (RFC 2822), send the reply to the recipients in **replyTo** and not the recipient in the **from** property. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. This method saves the message in the **Sent Items** folder. Alternatively, create a draft to reply to an existing message and send it later.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-reply?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_child_messages_message_reply(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        mail_folder_id_1: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageReplyRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/childFolders/{}/messages/{}/reply",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id_1.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action replyAll.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/childFolders/{mailFolder-id1}/messages/{message-id}/replyAll` endpoint.
    *
    * Reply to all recipients of a message using either JSON or MIME format. When using JSON format:
    * - Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP 400 Bad Request error.
    * - If the original message specifies a recipient in the **replyTo** property, per Internet Message Format (RFC 2822), send the reply to the recipients in **replyTo** and not the recipient in the **from** property. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. This method saves the message in the **Sent Items** folder. Alternatively, create a draft to reply-all to a message and send it later.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-replyall?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_child_messages_message_reply_all(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        mail_folder_id_1: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageReplyRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/childFolders/{}/messages/{}/replyAll",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id_1.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action send.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/childFolders/{mailFolder-id1}/messages/{message-id}/send` endpoint.
    *
    * Send an existing draft message.  The draft message can be a new message draft, reply draft, reply-all draft, or a forward draft. This method saves the message in the **Sent Items** folder. Alternatively, send a new message in a single operation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-send?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_child_messages_message_send(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        mail_folder_id_1: &str,
        message_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/childFolders/{}/messages/{}/send",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id_1.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action copy.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/childFolders/{mailFolder-id1}/copy` endpoint.
    *
    * Copy a mailfolder and its contents to another mailfolder.
    *
    * FROM: <https://docs.microsoft.com/graph/api/mailfolder-copy?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_child_copy(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        mail_folder_id_1: &str,
        body: &crate::types::UsersUserMessagesMessageCopyRequest,
    ) -> ClientResult<crate::types::UsersUserMailFoldersFolderCopyResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/childFolders/{}/copy",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id_1.to_string()),
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
    * Invoke action move.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/childFolders/{mailFolder-id1}/move` endpoint.
    *
    * Move a mailfolder and its contents to another mailfolder.
    *
    * FROM: <https://docs.microsoft.com/graph/api/mailfolder-move?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_child_move(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        mail_folder_id_1: &str,
        body: &crate::types::UsersUserMessagesMessageCopyRequest,
    ) -> ClientResult<crate::types::UsersUserMailFoldersFolderCopyResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/childFolders/{}/move",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id_1.to_string()),
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/messages/{message-id}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_messages_message_attachments_create_upload_session(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/messages/{}/attachments/createUploadSession",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action copy.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/messages/{message-id}/copy` endpoint.
    *
    * Copy a message to a folder within the user's mailbox.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-copy?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_messages_message_copy(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageCopyRequest,
    ) -> ClientResult<crate::types::MessageAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/messages/{}/copy",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action createForward.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/messages/{message-id}/createForward` endpoint.
    *
    * Create a draft to forward an existing message, in either JSON or MIME format. When using JSON format, you can:
    * - Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP 400 Bad Request error.
    * - Specify either the `toRecipients` parameter or the **toRecipients** property of the `message` parameter. Specifying both or specifying neither will return an HTTP 400 Bad Request error.
    * - Update the draft later to add content to the **body** or change other message properties. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. Send the draft message in a subsequent operation. Alternatively, forward a message in a single operation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-createforward?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_messages_message_create_forward(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageForwardRequest,
    ) -> ClientResult<crate::types::MessageAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/messages/{}/createForward",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action createReply.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/messages/{message-id}/createReply` endpoint.
    *
    * Create a draft to reply to the sender of a message in either JSON or MIME format. When using JSON format:
    * - Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP 400 Bad Request error.
    * - If **replyTo** is specified in the original message, per Internet Message Format (RFC 2822), you should send the reply to the recipients in **replyTo**, and not the recipients in **from**.
    * - You can update the draft later to add reply content to the **body** or change other message properties. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. Send the draft message in a subsequent operation. Alternatively, reply to a message in a single operation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-createreply?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_messages_message_create_reply(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageReplyRequest,
    ) -> ClientResult<crate::types::MessageAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/messages/{}/createReply",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action createReplyAll.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/messages/{message-id}/createReplyAll` endpoint.
    *
    * Create a draft to reply to the sender and all recipients of a message in either JSON or MIME format.  When using JSON format:
    * - Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP 400 Bad Request error.
    * - If the original message specifies a recipient in the **replyTo** property, per Internet Message Format (RFC 2822), you should send the reply to the recipients in the **replyTo** and **toRecipients** properties, and not the recipients in the **from** and **toRecipients** properties.
    * - You can update the draft later to add reply content to the **body** or change other message properties. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. Send the draft message in a subsequent operation. Alternatively, reply-all to a message in a single action.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-createreplyall?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_messages_message_create_reply_all(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageReplyRequest,
    ) -> ClientResult<crate::types::MessageAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/messages/{}/createReplyAll",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/messages/{message-id}/forward` endpoint.
    *
    * Forward a message using either JSON or MIME format. When using JSON format, you can:
    * - Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP 400 Bad Request error.
    * - Specify either the `toRecipients` parameter or the **toRecipients** property of the `message` parameter. Specifying both or specifying neither will return an HTTP 400 Bad Request error. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. This method saves the message in the **Sent Items** folder. Alternatively, create a draft to forward a message, and send it later.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_messages_message_forward(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/messages/{}/forward",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action move.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/messages/{message-id}/move` endpoint.
    *
    * Move a message to another folder within the specified user's mailbox. This creates a new copy of the message in the destination folder and removes the original message.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-move?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_messages_message_move(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageCopyRequest,
    ) -> ClientResult<crate::types::MessageAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/messages/{}/move",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action reply.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/messages/{message-id}/reply` endpoint.
    *
    * Reply to the sender of a message using either JSON or MIME format. When using JSON format:
    * * Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP `400 Bad Request` error.
    * * If the original message specifies a recipient in the **replyTo** property, per Internet Message Format (RFC 2822), send the reply to the recipients in **replyTo** and not the recipient in the **from** property. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. This method saves the message in the **Sent Items** folder. Alternatively, create a draft to reply to an existing message and send it later.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-reply?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_messages_message_reply(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageReplyRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/messages/{}/reply",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action replyAll.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/messages/{message-id}/replyAll` endpoint.
    *
    * Reply to all recipients of a message using either JSON or MIME format. When using JSON format:
    * - Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP 400 Bad Request error.
    * - If the original message specifies a recipient in the **replyTo** property, per Internet Message Format (RFC 2822), send the reply to the recipients in **replyTo** and not the recipient in the **from** property. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. This method saves the message in the **Sent Items** folder. Alternatively, create a draft to reply-all to a message and send it later.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-replyall?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_messages_message_reply_all(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageReplyRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/messages/{}/replyAll",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action send.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/messages/{message-id}/send` endpoint.
    *
    * Send an existing draft message.  The draft message can be a new message draft, reply draft, reply-all draft, or a forward draft. This method saves the message in the **Sent Items** folder. Alternatively, send a new message in a single operation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-send?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_messages_message_send(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        message_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/messages/{}/send",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action copy.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/copy` endpoint.
    *
    * Copy a mailfolder and its contents to another mailfolder.
    *
    * FROM: <https://docs.microsoft.com/graph/api/mailfolder-copy?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_copy(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        body: &crate::types::UsersUserMessagesMessageCopyRequest,
    ) -> ClientResult<crate::types::UsersUserMailFoldersFolderCopyResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/copy",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
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
    * Invoke action move.
    *
    * This function performs a `POST` to the `/users/{user-id}/mailFolders/{mailFolder-id}/move` endpoint.
    *
    * Move a mailfolder and its contents to another mailfolder.
    *
    * FROM: <https://docs.microsoft.com/graph/api/mailfolder-move?view=graph-rest-1.0>
    */
    pub async fn users_user_mail_folders_folder_move(
        &self,
        user_id: &str,
        mail_folder_id: &str,
        body: &crate::types::UsersUserMessagesMessageCopyRequest,
    ) -> ClientResult<crate::types::UsersUserMailFoldersFolderCopyResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailFolders/{}/move",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&mail_folder_id.to_string()),
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
    * Invoke action bypassActivationLock.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/bypassActivationLock` endpoint.
    *
    * Bypass activation lock
    */
    pub async fn users_user_managed_devices_device_bypass_activation_lock(
        &self,
        user_id: &str,
        managed_device_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/bypassActivationLock",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action cleanWindowsDevice.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/cleanWindowsDevice` endpoint.
    *
    * Clean Windows device
    */
    pub async fn users_user_managed_devices_device_clean_windows(
        &self,
        user_id: &str,
        managed_device_id: &str,
        body: &crate::types::UsersUserManagedDevicesDeviceCleanWindowsRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/cleanWindowsDevice",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
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
    * Invoke action deleteUserFromSharedAppleDevice.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/deleteUserFromSharedAppleDevice` endpoint.
    *
    * Delete user from shared Apple device
    */
    pub async fn users_user_managed_devices_device_delete_from_shared_apple(
        &self,
        user_id: &str,
        managed_device_id: &str,
        body: &crate::types::UsersUserManagedDevicesDeviceDeleteFromSharedAppleRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/deleteUserFromSharedAppleDevice",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
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
    * Invoke action disableLostMode.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/disableLostMode` endpoint.
    *
    * Disable lost mode
    */
    pub async fn users_user_managed_devices_device_disable_lost_mode(
        &self,
        user_id: &str,
        managed_device_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/disableLostMode",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action locateDevice.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/locateDevice` endpoint.
    *
    * Locate a device
    */
    pub async fn users_user_managed_devices_device_locate(
        &self,
        user_id: &str,
        managed_device_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/locateDevice",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action logoutSharedAppleDeviceActiveUser.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/logoutSharedAppleDeviceActiveUser` endpoint.
    *
    * Logout shared Apple device active user
    */
    pub async fn users_user_managed_devices_device_logout_shared_apple_active(
        &self,
        user_id: &str,
        managed_device_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/logoutSharedAppleDeviceActiveUser",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action rebootNow.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/rebootNow` endpoint.
    *
    * Reboot device
    */
    pub async fn users_user_managed_devices_device_reboot_now(
        &self,
        user_id: &str,
        managed_device_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/rebootNow",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action recoverPasscode.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/recoverPasscode` endpoint.
    *
    * Recover passcode
    */
    pub async fn users_user_managed_devices_device_recover_passcode(
        &self,
        user_id: &str,
        managed_device_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/recoverPasscode",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action remoteLock.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/remoteLock` endpoint.
    *
    * Remote lock
    */
    pub async fn users_user_managed_devices_device_remote_lock(
        &self,
        user_id: &str,
        managed_device_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/remoteLock",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action requestRemoteAssistance.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/requestRemoteAssistance` endpoint.
    *
    * Request remote assistance
    */
    pub async fn users_user_managed_devices_device_request_remote_assistance(
        &self,
        user_id: &str,
        managed_device_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/requestRemoteAssistance",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action resetPasscode.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/resetPasscode` endpoint.
    *
    * Reset passcode
    */
    pub async fn users_user_managed_devices_device_reset_passcode(
        &self,
        user_id: &str,
        managed_device_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/resetPasscode",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action retire.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/retire` endpoint.
    *
    * Retire a device
    */
    pub async fn users_user_managed_devices_device_retire(
        &self,
        user_id: &str,
        managed_device_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/retire",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action shutDown.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/shutDown` endpoint.
    *
    * Shut down device
    */
    pub async fn users_user_managed_devices_device_shut_down(
        &self,
        user_id: &str,
        managed_device_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/shutDown",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action syncDevice.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/syncDevice` endpoint.
    */
    pub async fn users_user_managed_devices_device_sync(
        &self,
        user_id: &str,
        managed_device_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/syncDevice",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action updateWindowsDeviceAccount.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/updateWindowsDeviceAccount` endpoint.
    */
    pub async fn users_user_managed_devices_device_update_windows_account(
        &self,
        user_id: &str,
        managed_device_id: &str,
        body: &crate::types::UsersUserManagedDevicesDeviceUpdateWindowsAccountRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/updateWindowsDeviceAccount",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
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
    * Invoke action windowsDefenderScan.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/windowsDefenderScan` endpoint.
    */
    pub async fn users_user_managed_devices_device_windows_defender_scan(
        &self,
        user_id: &str,
        managed_device_id: &str,
        body: &crate::types::UsersUserManagedDevicesDeviceWindowsDefenderScanRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/windowsDefenderScan",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
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
    * Invoke action windowsDefenderUpdateSignatures.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/windowsDefenderUpdateSignatures` endpoint.
    */
    pub async fn users_user_managed_devices_device_windows_defender_update_signatures(
        &self,
        user_id: &str,
        managed_device_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/windowsDefenderUpdateSignatures",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action wipe.
    *
    * This function performs a `POST` to the `/users/{user-id}/managedDevices/{managedDevice-id}/wipe` endpoint.
    *
    * Wipe a device
    */
    pub async fn users_user_managed_devices_device_wipe(
        &self,
        user_id: &str,
        managed_device_id: &str,
        body: &crate::types::UsersUserManagedDevicesDeviceWipeRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/managedDevices/{}/wipe",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&managed_device_id.to_string()),
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/messages/{message-id}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session that allows an app to iteratively upload ranges of a file, so as to attach the file to the specified Outlook item. The item can be a message or event. Use this approach to attach a file if the file size is between 3 MB and 150 MB. To attach a file that's smaller than 3 MB, do a `POST` operation on the **attachments** navigation property of the Outlook item; see how to do this for a message or for an event.  As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. Request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows transfer to be resumed, in case the network connection is dropped during upload.  The following are the steps to attach a file to an Outlook item using an upload session: See attach large files to Outlook messages or events for an example.
    *
    * FROM: <https://docs.microsoft.com/graph/api/attachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_messages_message_attachments_create_upload_session(
        &self,
        user_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/messages/{}/attachments/createUploadSession",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action copy.
    *
    * This function performs a `POST` to the `/users/{user-id}/messages/{message-id}/copy` endpoint.
    *
    * Copy a message to a folder within the user's mailbox.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-copy?view=graph-rest-1.0>
    */
    pub async fn users_user_messages_message_copy(
        &self,
        user_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageCopyRequest,
    ) -> ClientResult<crate::types::MessageAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/messages/{}/copy",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action createForward.
    *
    * This function performs a `POST` to the `/users/{user-id}/messages/{message-id}/createForward` endpoint.
    *
    * Create a draft to forward an existing message, in either JSON or MIME format. When using JSON format, you can:
    * - Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP 400 Bad Request error.
    * - Specify either the `toRecipients` parameter or the **toRecipients** property of the `message` parameter. Specifying both or specifying neither will return an HTTP 400 Bad Request error.
    * - Update the draft later to add content to the **body** or change other message properties. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. Send the draft message in a subsequent operation. Alternatively, forward a message in a single operation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-createforward?view=graph-rest-1.0>
    */
    pub async fn users_user_messages_message_create_forward(
        &self,
        user_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageForwardRequest,
    ) -> ClientResult<crate::types::MessageAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/messages/{}/createForward",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action createReply.
    *
    * This function performs a `POST` to the `/users/{user-id}/messages/{message-id}/createReply` endpoint.
    *
    * Create a draft to reply to the sender of a message in either JSON or MIME format. When using JSON format:
    * - Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP 400 Bad Request error.
    * - If **replyTo** is specified in the original message, per Internet Message Format (RFC 2822), you should send the reply to the recipients in **replyTo**, and not the recipients in **from**.
    * - You can update the draft later to add reply content to the **body** or change other message properties. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. Send the draft message in a subsequent operation. Alternatively, reply to a message in a single operation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-createreply?view=graph-rest-1.0>
    */
    pub async fn users_user_messages_message_create_reply(
        &self,
        user_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageReplyRequest,
    ) -> ClientResult<crate::types::MessageAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/messages/{}/createReply",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action createReplyAll.
    *
    * This function performs a `POST` to the `/users/{user-id}/messages/{message-id}/createReplyAll` endpoint.
    *
    * Create a draft to reply to the sender and all recipients of a message in either JSON or MIME format.  When using JSON format:
    * - Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP 400 Bad Request error.
    * - If the original message specifies a recipient in the **replyTo** property, per Internet Message Format (RFC 2822), you should send the reply to the recipients in the **replyTo** and **toRecipients** properties, and not the recipients in the **from** and **toRecipients** properties.
    * - You can update the draft later to add reply content to the **body** or change other message properties. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. Send the draft message in a subsequent operation. Alternatively, reply-all to a message in a single action.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-createreplyall?view=graph-rest-1.0>
    */
    pub async fn users_user_messages_message_create_reply_all(
        &self,
        user_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageReplyRequest,
    ) -> ClientResult<crate::types::MessageAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/messages/{}/createReplyAll",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action forward.
    *
    * This function performs a `POST` to the `/users/{user-id}/messages/{message-id}/forward` endpoint.
    *
    * Forward a message using either JSON or MIME format. When using JSON format, you can:
    * - Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP 400 Bad Request error.
    * - Specify either the `toRecipients` parameter or the **toRecipients** property of the `message` parameter. Specifying both or specifying neither will return an HTTP 400 Bad Request error. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. This method saves the message in the **Sent Items** folder. Alternatively, create a draft to forward a message, and send it later.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-forward?view=graph-rest-1.0>
    */
    pub async fn users_user_messages_message_forward(
        &self,
        user_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageForwardRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/messages/{}/forward",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action move.
    *
    * This function performs a `POST` to the `/users/{user-id}/messages/{message-id}/move` endpoint.
    *
    * Move a message to another folder within the specified user's mailbox. This creates a new copy of the message in the destination folder and removes the original message.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-move?view=graph-rest-1.0>
    */
    pub async fn users_user_messages_message_move(
        &self,
        user_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageCopyRequest,
    ) -> ClientResult<crate::types::MessageAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/messages/{}/move",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action reply.
    *
    * This function performs a `POST` to the `/users/{user-id}/messages/{message-id}/reply` endpoint.
    *
    * Reply to the sender of a message using either JSON or MIME format. When using JSON format:
    * * Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP `400 Bad Request` error.
    * * If the original message specifies a recipient in the **replyTo** property, per Internet Message Format (RFC 2822), send the reply to the recipients in **replyTo** and not the recipient in the **from** property. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. This method saves the message in the **Sent Items** folder. Alternatively, create a draft to reply to an existing message and send it later.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-reply?view=graph-rest-1.0>
    */
    pub async fn users_user_messages_message_reply(
        &self,
        user_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageReplyRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/messages/{}/reply",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action replyAll.
    *
    * This function performs a `POST` to the `/users/{user-id}/messages/{message-id}/replyAll` endpoint.
    *
    * Reply to all recipients of a message using either JSON or MIME format. When using JSON format:
    * - Specify either a comment or the **body** property of the `message` parameter. Specifying both will return an HTTP 400 Bad Request error.
    * - If the original message specifies a recipient in the **replyTo** property, per Internet Message Format (RFC 2822), send the reply to the recipients in **replyTo** and not the recipient in the **from** property. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. This method saves the message in the **Sent Items** folder. Alternatively, create a draft to reply-all to a message and send it later.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-replyall?view=graph-rest-1.0>
    */
    pub async fn users_user_messages_message_reply_all(
        &self,
        user_id: &str,
        message_id: &str,
        body: &crate::types::UsersUserMessagesMessageReplyRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/messages/{}/replyAll",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
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
    * Invoke action send.
    *
    * This function performs a `POST` to the `/users/{user-id}/messages/{message-id}/send` endpoint.
    *
    * Send an existing draft message.  The draft message can be a new message draft, reply draft, reply-all draft, or a forward draft. This method saves the message in the **Sent Items** folder. Alternatively, send a new message in a single operation.
    *
    * FROM: <https://docs.microsoft.com/graph/api/message-send?view=graph-rest-1.0>
    */
    pub async fn users_user_messages_message_send(
        &self,
        user_id: &str,
        message_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/messages/{}/send",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&message_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action assignLicense.
    *
    * This function performs a `POST` to the `/users/{user-id}/assignLicense` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-assignlicense?view=graph-rest-1.0>
    */
    pub async fn users_user_assign_license(
        &self,
        user_id: &str,
        body: &crate::types::AssignLicenseRequestBody,
    ) -> ClientResult<crate::types::UserAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/assignLicense",
                crate::progenitor_support::encode_path(&user_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action changePassword.
    *
    * This function performs a `POST` to the `/users/{user-id}/changePassword` endpoint.
    *
    * Enable the user to update their password. Any user can update their password without belonging to any administrator role.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-changepassword?view=graph-rest-1.0>
    */
    pub async fn users_user_change_password(
        &self,
        user_id: &str,
        body: &crate::types::ChangePasswordRequestBody,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/changePassword",
                crate::progenitor_support::encode_path(&user_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action checkMemberGroups.
    *
    * This function performs a `POST` to the `/users/{user-id}/checkMemberGroups` endpoint.
    *
    * Check for membership in a specified list of group IDs, and return from that list those groups (identified by IDs) of which the specified user, group, service principal, organizational contact, device, or directory object is a member. This function is transitive. You can check up to a maximum of 20 groups per request. This function supports all groups provisioned in Azure AD. Because Microsoft 365 groups cannot contain other groups, membership in a Microsoft 365 group is always direct.
    *
    * FROM: <https://docs.microsoft.com/graph/api/directoryobject-checkmembergroups?view=graph-rest-1.0>
    */
    pub async fn users_user_check_member_groups(
        &self,
        user_id: &str,
        body: &crate::types::UsersUserCheckMemberGroupsRequest,
    ) -> ClientResult<crate::types::Me> {
        let url = self.client.url(
            &format!(
                "/users/{}/checkMemberGroups",
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
    * Invoke action checkMemberObjects.
    *
    * This function performs a `POST` to the `/users/{user-id}/checkMemberObjects` endpoint.
    */
    pub async fn users_user_check_member_objects(
        &self,
        user_id: &str,
        body: &crate::types::UsersUserCheckMemberObjectsRequest,
    ) -> ClientResult<crate::types::Me> {
        let url = self.client.url(
            &format!(
                "/users/{}/checkMemberObjects",
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
    * Invoke action exportPersonalData.
    *
    * This function performs a `POST` to the `/users/{user-id}/exportPersonalData` endpoint.
    *
    * Submit a data policy operation request from a company administrator or an application to export an organizational user's data. This data includes the user's data stored in OneDrive and their activity reports. For more guidance about exporting data while complying with regulations, see Data Subject Requests and the GDPR and CCPA.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-exportpersonaldata?view=graph-rest-1.0>
    */
    pub async fn users_user_export_personal_data(
        &self,
        user_id: &str,
        body: &crate::types::ExportPersonalDataRequestBody,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/exportPersonalData",
                crate::progenitor_support::encode_path(&user_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action findMeetingTimes.
    *
    * This function performs a `POST` to the `/users/{user-id}/findMeetingTimes` endpoint.
    *
    * Suggest meeting times and locations based on organizer and attendee availability, and time or location constraints specified as parameters. If **findMeetingTimes** cannot return any meeting suggestions, the response would indicate a reason in the **emptySuggestionsReason** property.
    * Based on this value, you can better adjust the parameters and call **findMeetingTimes** again. The algorithm used to suggest meeting times and locations undergoes fine-tuning from time to time. In scenarios like test environments where the input parameters and calendar data remain static, expect that the suggested results may differ over time.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-findmeetingtimes?view=graph-rest-1.0>
    */
    pub async fn users_user_find_meeting_times(
        &self,
        user_id: &str,
        body: &crate::types::FindMeetingTimesRequestBody,
    ) -> ClientResult<crate::types::FindMeetingTimesResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/findMeetingTimes",
                crate::progenitor_support::encode_path(&user_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action getMailTips.
    *
    * This function performs a `POST` to the `/users/{user-id}/getMailTips` endpoint.
    *
    * Get the MailTips of one or more recipients as available to the signed-in user. Note that by making a `POST` call to the `getMailTips` action, you can request specific types of MailTips to
    * be returned for more than one recipient at one time. The requested MailTips are returned in a mailTips collection.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-getmailtips?view=graph-rest-1.0>
    */
    pub async fn users_user_get_mail_tips(
        &self,
        user_id: &str,
        body: &crate::types::GetMailTipsRequestBody,
    ) -> ClientResult<crate::types::Me> {
        let url = self.client.url(
            &format!(
                "/users/{}/getMailTips",
                crate::progenitor_support::encode_path(&user_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action getMemberGroups.
    *
    * This function performs a `POST` to the `/users/{user-id}/getMemberGroups` endpoint.
    *
    * Return all the group IDs for the groups that the specified user, group, service principal, organizational contact, device, or directory object is a member of. This function is transitive.
    *
    * FROM: <https://docs.microsoft.com/graph/api/directoryobject-getmembergroups?view=graph-rest-1.0>
    */
    pub async fn users_user_get_member_groups(
        &self,
        user_id: &str,
        body: &crate::types::UsersUserGetMemberGroupsRequest,
    ) -> ClientResult<crate::types::Me> {
        let url = self.client.url(
            &format!(
                "/users/{}/getMemberGroups",
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
    * Invoke action getMemberObjects.
    *
    * This function performs a `POST` to the `/users/{user-id}/getMemberObjects` endpoint.
    *
    * Return all IDs for the groups, administrative units, and directory roles that a user, group, service principal, organizational contact, device, or directory object is a member of. This function is transitive. **Note:** Only users and role-enabled groups can be members of directory roles.
    *
    * FROM: <https://docs.microsoft.com/graph/api/directoryobject-getmemberobjects?view=graph-rest-1.0>
    */
    pub async fn users_user_get_member_objects(
        &self,
        user_id: &str,
        body: &crate::types::UsersUserGetMemberGroupsRequest,
    ) -> ClientResult<crate::types::Me> {
        let url = self.client.url(
            &format!(
                "/users/{}/getMemberObjects",
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
    * Invoke action removeAllDevicesFromManagement.
    *
    * This function performs a `POST` to the `/users/{user-id}/removeAllDevicesFromManagement` endpoint.
    *
    * Retire all devices from management for this user
    */
    pub async fn users_user_remove_all_devices_from_management(
        &self,
        user_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/removeAllDevicesFromManagement",
                crate::progenitor_support::encode_path(&user_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action reprocessLicenseAssignment.
    *
    * This function performs a `POST` to the `/users/{user-id}/reprocessLicenseAssignment` endpoint.
    *
    * Reprocess all group-based license assignments for the user. To learn more about group-based licensing, see What is group-based licensing in Azure Active Directory. Also see Identify and resolve license assignment problems for a group in Azure Active Directory for more details.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-reprocesslicenseassignment?view=graph-rest-1.0>
    */
    pub async fn users_user_reprocess_license_assignment(
        &self,
        user_id: &str,
    ) -> ClientResult<crate::types::UserAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/reprocessLicenseAssignment",
                crate::progenitor_support::encode_path(&user_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action restore.
    *
    * This function performs a `POST` to the `/users/{user-id}/restore` endpoint.
    *
    * Restore a recently deleted application, group, servicePrincipal, administrative unit, or user object from deleted items. If an item was accidentally deleted, you can fully restore the item. This is not applicable to security groups, which are deleted permanently. A recently deleted item will remain available for up to 30 days. After 30 days, the item is permanently deleted.
    *
    * FROM: <https://docs.microsoft.com/graph/api/directory-deleteditems-restore?view=graph-rest-1.0>
    */
    pub async fn users_user_restore(
        &self,
        user_id: &str,
    ) -> ClientResult<crate::types::ManagerAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/restore",
                crate::progenitor_support::encode_path(&user_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action revokeSignInSessions.
    *
    * This function performs a `POST` to the `/users/{user-id}/revokeSignInSessions` endpoint.
    */
    pub async fn users_user_revoke_sign_sessions(
        &self,
        user_id: &str,
    ) -> ClientResult<crate::types::RevokeSignInSessionsResponse> {
        let url = self.client.url(
            &format!(
                "/users/{}/revokeSignInSessions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action sendMail.
    *
    * This function performs a `POST` to the `/users/{user-id}/sendMail` endpoint.
    *
    * Send the message specified in the request body using either JSON or MIME format. When using JSON format you can include a file attachment in the same **sendMail** action call. When using MIME format:
    * - Provide the applicable Internet message headers and the MIME content, all encoded in **base64** format in the request body.
    * - Add any attachments and S/MIME properties to the MIME content. This method saves the message in the **Sent Items** folder. Alternatively, create a draft message to send later. To learn more about the steps involved in the backend before a mail is delivered to recipients, see here.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-sendmail?view=graph-rest-1.0>
    */
    pub async fn users_user_send_mail(
        &self,
        user_id: &str,
        body: &crate::types::SendMailRequestBody,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/sendMail",
                crate::progenitor_support::encode_path(&user_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action translateExchangeIds.
    *
    * This function performs a `POST` to the `/users/{user-id}/translateExchangeIds` endpoint.
    *
    * Translate identifiers of Outlook-related resources between formats.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-translateexchangeids?view=graph-rest-1.0>
    */
    pub async fn users_user_translate_exchange_ids(
        &self,
        user_id: &str,
        body: &crate::types::TranslateExchangeIdsRequestBody,
    ) -> ClientResult<crate::types::Me> {
        let url = self.client.url(
            &format!(
                "/users/{}/translateExchangeIds",
                crate::progenitor_support::encode_path(&user_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action wipeManagedAppRegistrationsByDeviceTag.
    *
    * This function performs a `POST` to the `/users/{user-id}/wipeManagedAppRegistrationsByDeviceTag` endpoint.
    *
    * Issues a wipe operation on an app registration with specified device tag.
    */
    pub async fn users_user_wipe_managed_app_registrations_device_tag(
        &self,
        user_id: &str,
        body: &crate::types::WipeManagedAppRegistrationsByDeviceTagRequestBody,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/wipeManagedAppRegistrationsByDeviceTag",
                crate::progenitor_support::encode_path(&user_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action copyNotebook.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/copyNotebook` endpoint.
    *
    * For Copy operations, you follow an asynchronous calling pattern:  First call the Copy action, and then poll the operation endpoint for the result.
    *
    * FROM: <https://docs.microsoft.com/graph/api/notebook-copynotebook?view=graph-rest-1.0>
    */
    pub async fn users_user_onenote_notebooks_notebook_copy(
        &self,
        user_id: &str,
        notebook_id: &str,
        body: &crate::types::UsersUserOnenoteNotebooksNotebookCopyRequest,
    ) -> ClientResult<crate::types::UsersUserOnenoteNotebooksNotebookCopyResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/copyNotebook",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
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
    * Invoke action copyToNotebook.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/copyToNotebook` endpoint.
    *
    * For Copy operations, you follow an asynchronous calling pattern:  First call the Copy action, and then poll the operation endpoint for the result.
    *
    * FROM: <https://docs.microsoft.com/graph/api/section-copytonotebook?view=graph-rest-1.0>
    */
    pub async fn users_user_onenote_notebooks_notebook_section_groups_group_sections_copy(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        body: &crate::types::UsersUserOnenoteSectionsSectionCopyGroupRequest,
    ) -> ClientResult<crate::types::UsersUserOnenoteNotebooksNotebookCopyResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}/copyToNotebook",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Invoke action copyToSectionGroup.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/copyToSectionGroup` endpoint.
    *
    * For Copy operations, you follow an asynchronous calling pattern:  First call the Copy action, and then poll the operation endpoint for the result.
    *
    * FROM: <https://docs.microsoft.com/graph/api/section-copytosectiongroup?view=graph-rest-1.0>
    */
    pub async fn users_user_onenote_notebooks_notebook_section_groups_group_sections_copy_users_actions(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        body: &crate::types::UsersUserOnenoteSectionsSectionCopyGroupRequest,
    ) -> ClientResult<crate::types::UsersUserOnenoteNotebooksNotebookCopyResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}/copyToSectionGroup",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Invoke action copyToSection.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/copyToSection` endpoint.
    *
    * Copy a page to a specific section. For copy operations, you follow an asynchronous calling pattern:  First call the Copy action, and then poll the operation endpoint for the result.
    *
    * FROM: <https://docs.microsoft.com/graph/api/page-copytosection?view=graph-rest-1.0>
    */
    pub async fn users_user_onenote_notebooks_notebook_section_groups_group_sections_pages_page_copy(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        body: &crate::types::UsersUserOnenotePagesPageCopySectionRequest,
    ) -> ClientResult<crate::types::UsersUserOnenoteNotebooksNotebookCopyResponseAnyOf> {
        let url = self.client.url(
&format!("/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}/pages/{}/copyToSection",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&notebook_id.to_string()),crate::progenitor_support::encode_path(&section_group_id.to_string()),crate::progenitor_support::encode_path(&onenote_section_id.to_string()),crate::progenitor_support::encode_path(&onenote_page_id.to_string()),), None);
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
    * Invoke action onenotePatchContent.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/onenotePatchContent` endpoint.
    */
    pub async fn users_user_onenote_notebooks_notebook_section_groups_group_sections_pages_page_patch_content(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        body: &crate::types::UsersUserOnenotePagesPagePatchContentRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}/pages/{}/onenotePatchContent",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&notebook_id.to_string()),crate::progenitor_support::encode_path(&section_group_id.to_string()),crate::progenitor_support::encode_path(&onenote_section_id.to_string()),crate::progenitor_support::encode_path(&onenote_page_id.to_string()),), None);
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
    * Invoke action copyToNotebook.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}/copyToNotebook` endpoint.
    *
    * For Copy operations, you follow an asynchronous calling pattern:  First call the Copy action, and then poll the operation endpoint for the result.
    *
    * FROM: <https://docs.microsoft.com/graph/api/section-copytonotebook?view=graph-rest-1.0>
    */
    pub async fn users_user_onenote_notebooks_notebook_sections_section_copy(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
        body: &crate::types::UsersUserOnenoteSectionsSectionCopyGroupRequest,
    ) -> ClientResult<crate::types::UsersUserOnenoteNotebooksNotebookCopyResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sections/{}/copyToNotebook",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Invoke action copyToSectionGroup.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}/copyToSectionGroup` endpoint.
    *
    * For Copy operations, you follow an asynchronous calling pattern:  First call the Copy action, and then poll the operation endpoint for the result.
    *
    * FROM: <https://docs.microsoft.com/graph/api/section-copytosectiongroup?view=graph-rest-1.0>
    */
    pub async fn users_user_onenote_notebooks_notebook_sections_section_copy_group(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
        body: &crate::types::UsersUserOnenoteSectionsSectionCopyGroupRequest,
    ) -> ClientResult<crate::types::UsersUserOnenoteNotebooksNotebookCopyResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sections/{}/copyToSectionGroup",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Invoke action copyToSection.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/copyToSection` endpoint.
    *
    * Copy a page to a specific section. For copy operations, you follow an asynchronous calling pattern:  First call the Copy action, and then poll the operation endpoint for the result.
    *
    * FROM: <https://docs.microsoft.com/graph/api/page-copytosection?view=graph-rest-1.0>
    */
    pub async fn users_user_onenote_notebooks_notebook_sections_section_pages_page_copy(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        body: &crate::types::UsersUserOnenotePagesPageCopySectionRequest,
    ) -> ClientResult<crate::types::UsersUserOnenoteNotebooksNotebookCopyResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sections/{}/pages/{}/copyToSection",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Invoke action onenotePatchContent.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/onenotePatchContent` endpoint.
    */
    pub async fn users_user_onenote_notebooks_notebook_sections_section_pages_page_patch_content(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        body: &crate::types::UsersUserOnenotePagesPagePatchContentRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sections/{}/pages/{}/onenotePatchContent",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Invoke action getNotebookFromWebUrl.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/notebooks/getNotebookFromWebUrl` endpoint.
    *
    * Retrieve the properties and relationships of a notebook object by using its URL path. The location can be user notebooks on Microsoft 365, group notebooks, or SharePoint site-hosted team notebooks on Microsoft 365.
    *
    * FROM: <https://docs.microsoft.com/graph/api/notebook-getnotebookfromweburl?view=graph-rest-1.0>
    */
    pub async fn users_user_onenote_notebooks_get_notebook_from_web_url(
        &self,
        user_id: &str,
        body: &crate::types::UsersUserOnenoteNotebooksGetNotebookFromWebUrlRequest,
    ) -> ClientResult<crate::types::UsersUserOnenoteNotebooksGetNotebookFromWebUrlResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/getNotebookFromWebUrl",
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
    * Invoke action copyToSection.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/pages/{onenotePage-id}/copyToSection` endpoint.
    *
    * Copy a page to a specific section. For copy operations, you follow an asynchronous calling pattern:  First call the Copy action, and then poll the operation endpoint for the result.
    *
    * FROM: <https://docs.microsoft.com/graph/api/page-copytosection?view=graph-rest-1.0>
    */
    pub async fn users_user_onenote_pages_page_copy_section(
        &self,
        user_id: &str,
        onenote_page_id: &str,
        body: &crate::types::UsersUserOnenotePagesPageCopySectionRequest,
    ) -> ClientResult<crate::types::UsersUserOnenoteNotebooksNotebookCopyResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/pages/{}/copyToSection",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Invoke action onenotePatchContent.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/pages/{onenotePage-id}/onenotePatchContent` endpoint.
    */
    pub async fn users_user_onenote_pages_page_patch_content(
        &self,
        user_id: &str,
        onenote_page_id: &str,
        body: &crate::types::UsersUserOnenotePagesPagePatchContentRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/pages/{}/onenotePatchContent",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Invoke action copyToNotebook.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/copyToNotebook` endpoint.
    *
    * For Copy operations, you follow an asynchronous calling pattern:  First call the Copy action, and then poll the operation endpoint for the result.
    *
    * FROM: <https://docs.microsoft.com/graph/api/section-copytonotebook?view=graph-rest-1.0>
    */
    pub async fn users_user_onenote_section_groups_group_sections_copy_notebook(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        body: &crate::types::UsersUserOnenoteSectionsSectionCopyGroupRequest,
    ) -> ClientResult<crate::types::UsersUserOnenoteNotebooksNotebookCopyResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sectionGroups/{}/sections/{}/copyToNotebook",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Invoke action copyToSectionGroup.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/copyToSectionGroup` endpoint.
    *
    * For Copy operations, you follow an asynchronous calling pattern:  First call the Copy action, and then poll the operation endpoint for the result.
    *
    * FROM: <https://docs.microsoft.com/graph/api/section-copytosectiongroup?view=graph-rest-1.0>
    */
    pub async fn users_user_onenote_section_groups_group_sections_copy(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        body: &crate::types::UsersUserOnenoteSectionsSectionCopyGroupRequest,
    ) -> ClientResult<crate::types::UsersUserOnenoteNotebooksNotebookCopyResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sectionGroups/{}/sections/{}/copyToSectionGroup",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Invoke action copyToSection.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/copyToSection` endpoint.
    *
    * Copy a page to a specific section. For copy operations, you follow an asynchronous calling pattern:  First call the Copy action, and then poll the operation endpoint for the result.
    *
    * FROM: <https://docs.microsoft.com/graph/api/page-copytosection?view=graph-rest-1.0>
    */
    pub async fn users_user_onenote_section_groups_group_sections_pages_page_copy(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        body: &crate::types::UsersUserOnenotePagesPageCopySectionRequest,
    ) -> ClientResult<crate::types::UsersUserOnenoteNotebooksNotebookCopyResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sectionGroups/{}/sections/{}/pages/{}/copyToSection",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Invoke action onenotePatchContent.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/onenotePatchContent` endpoint.
    */
    pub async fn users_user_onenote_section_groups_group_sections_pages_page_patch_content(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        body: &crate::types::UsersUserOnenotePagesPagePatchContentRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sectionGroups/{}/sections/{}/pages/{}/onenotePatchContent",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Invoke action copyToNotebook.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}/copyToNotebook` endpoint.
    *
    * For Copy operations, you follow an asynchronous calling pattern:  First call the Copy action, and then poll the operation endpoint for the result.
    *
    * FROM: <https://docs.microsoft.com/graph/api/section-copytonotebook?view=graph-rest-1.0>
    */
    pub async fn users_user_onenote_sections_section_copy_notebook(
        &self,
        user_id: &str,
        onenote_section_id: &str,
        body: &crate::types::UsersUserOnenoteSectionsSectionCopyGroupRequest,
    ) -> ClientResult<crate::types::UsersUserOnenoteNotebooksNotebookCopyResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sections/{}/copyToNotebook",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Invoke action copyToSectionGroup.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}/copyToSectionGroup` endpoint.
    *
    * For Copy operations, you follow an asynchronous calling pattern:  First call the Copy action, and then poll the operation endpoint for the result.
    *
    * FROM: <https://docs.microsoft.com/graph/api/section-copytosectiongroup?view=graph-rest-1.0>
    */
    pub async fn users_user_onenote_sections_section_copy_group(
        &self,
        user_id: &str,
        onenote_section_id: &str,
        body: &crate::types::UsersUserOnenoteSectionsSectionCopyGroupRequest,
    ) -> ClientResult<crate::types::UsersUserOnenoteNotebooksNotebookCopyResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sections/{}/copyToSectionGroup",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Invoke action copyToSection.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}/pages/{onenotePage-id}/copyToSection` endpoint.
    *
    * Copy a page to a specific section. For copy operations, you follow an asynchronous calling pattern:  First call the Copy action, and then poll the operation endpoint for the result.
    *
    * FROM: <https://docs.microsoft.com/graph/api/page-copytosection?view=graph-rest-1.0>
    */
    pub async fn users_user_onenote_sections_section_pages_page_copy(
        &self,
        user_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        body: &crate::types::UsersUserOnenotePagesPageCopySectionRequest,
    ) -> ClientResult<crate::types::UsersUserOnenoteNotebooksNotebookCopyResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sections/{}/pages/{}/copyToSection",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Invoke action onenotePatchContent.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}/pages/{onenotePage-id}/onenotePatchContent` endpoint.
    */
    pub async fn users_user_onenote_sections_section_pages_page_patch_content(
        &self,
        user_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        body: &crate::types::UsersUserOnenotePagesPagePatchContentRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sections/{}/pages/{}/onenotePatchContent",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Invoke action createOrGet.
    *
    * This function performs a `POST` to the `/users/{user-id}/onlineMeetings/createOrGet` endpoint.
    *
    * Create an onlineMeeting object with a custom specified external ID. If the external ID already exists, this API will return the onlineMeeting object with that external ID.
    *
    * FROM: <https://docs.microsoft.com/graph/api/onlinemeeting-createorget?view=graph-rest-1.0>
    */
    pub async fn users_user_online_meetings_create_or_get(
        &self,
        user_id: &str,
        body: &crate::types::UsersUserOnlineMeetingsCreateGetRequest,
    ) -> ClientResult<crate::types::UsersUserOnlineMeetingsCreateGetResponseAnyOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onlineMeetings/createOrGet",
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
    * Invoke action clearPresence.
    *
    * This function performs a `POST` to the `/users/{user-id}/presence/clearPresence` endpoint.
    *
    * Clear the application's presence session for a user. If it is the user's only presence session, the user's presence will change to `Offline/Offline`. For details about presences sessions, see presence: setPresence.
    *
    * FROM: <https://docs.microsoft.com/graph/api/presence-clearpresence?view=graph-rest-1.0>
    */
    pub async fn users_user_presence_clear(
        &self,
        user_id: &str,
        body: &crate::types::UsersUserPresenceClearRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/presence/clearPresence",
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
    * Invoke action clearUserPreferredPresence.
    *
    * This function performs a `POST` to the `/users/{user-id}/presence/clearUserPreferredPresence` endpoint.
    *
    * Clear the preferred availability and activity status for a user.
    *
    * FROM: <https://docs.microsoft.com/graph/api/presence-clearuserpreferredpresence?view=graph-rest-1.0>
    */
    pub async fn users_user_presence_clear_preferred(&self, user_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/presence/clearUserPreferredPresence",
                crate::progenitor_support::encode_path(&user_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Invoke action setPresence.
    *
    * This function performs a `POST` to the `/users/{user-id}/presence/setPresence` endpoint.
    *
    * Set the state of a user's presence session as an application.
    *
    * FROM: <https://docs.microsoft.com/graph/api/presence-setpresence?view=graph-rest-1.0>
    */
    pub async fn users_user_presence_set(
        &self,
        user_id: &str,
        body: &crate::types::UsersUserPresenceSetRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/presence/setPresence",
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
    * Invoke action setUserPreferredPresence.
    *
    * This function performs a `POST` to the `/users/{user-id}/presence/setUserPreferredPresence` endpoint.
    *
    * Set the preferred availability and activity status for a user. If the preferred presence of a user is set, the user's presence shows as the preferred status. Preferred presence takes effect only when at least one presence session exists for the user. Otherwise, the user's presence shows as `Offline`. A presence session is created as a result of a successful setPresence operation, or if the user is signed in on a Microsoft Teams client. For more details, see presence sessions and time-out and expiration.
    *
    * FROM: <https://docs.microsoft.com/graph/api/presence-setuserpreferredpresence?view=graph-rest-1.0>
    */
    pub async fn users_user_presence_set_preferred(
        &self,
        user_id: &str,
        body: &crate::types::UsersUserPresenceSetPreferredRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/presence/setUserPreferredPresence",
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
    * Invoke action sendActivityNotification.
    *
    * This function performs a `POST` to the `/users/{user-id}/teamwork/sendActivityNotification` endpoint.
    *
    * Send an activity feed notification to a user. For more details about sending notifications and the requirements for doing so, see sending Teams activity notifications.
    *
    * FROM: <https://docs.microsoft.com/graph/api/userteamwork-sendactivitynotification?view=graph-rest-1.0>
    */
    pub async fn users_user_teamwork_send_activity_notification(
        &self,
        user_id: &str,
        body: &crate::types::UsersUserTeamworkSendActivityNotificationRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/teamwork/sendActivityNotification",
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
    * Invoke action createUploadSession.
    *
    * This function performs a `POST` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/attachments/createUploadSession` endpoint.
    *
    * Create an upload session to iteratively upload ranges of a file as an attachment to a todoTask. As part of the response, this action returns an upload URL that you can use in subsequent sequential `PUT` queries. The request headers for each `PUT` operation let you specify the exact range of bytes to be uploaded. This allows the transfer to be resumed, in case the network connection is dropped during the upload. The following are the steps to attach a file to a Microsoft To Do task using an upload session: For an example that describes the end-to-end attachment process, see attach files to a To Do task.
    *
    * FROM: <https://docs.microsoft.com/graph/api/taskfileattachment-createuploadsession?view=graph-rest-1.0>
    */
    pub async fn users_user_todo_lists_task_list_tasks_attachments_create_upload_session(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        body: &crate::types::UsersUserTodoListsTaskListTasksAttachmentsCreateUploadSessionRequest,
    ) -> ClientResult<crate::types::UsersUserEventsEventAttachmentsCreateUploadSessionResponseAnyOf>
    {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/attachments/createUploadSession",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * Invoke action getAvailableExtensionProperties.
    *
    * This function performs a `POST` to the `/users/getAvailableExtensionProperties` endpoint.
    *
    * Return all directory extension definitions that have been registered in a directory, including through multi-tenant apps. The following entities support extension properties:
    * + user
    * + group
    * + administrativeUnit
    * + application
    * + device
    * + organization
    *
    * FROM: <https://docs.microsoft.com/graph/api/directoryobject-getavailableextensionproperties?view=graph-rest-1.0>
    */
    pub async fn users_get_available_extension_properties(
        &self,
        body: &crate::types::UsersGetAvailableExtensionPropertiesRequest,
    ) -> ClientResult<crate::types::Me> {
        let url = self
            .client
            .url(&"/users/getAvailableExtensionProperties".to_string(), None);
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
    * Invoke action getByIds.
    *
    * This function performs a `POST` to the `/users/getByIds` endpoint.
    *
    * Return the directory objects specified in a list of IDs. Some common uses for this function are to:
    *
    * FROM: <https://docs.microsoft.com/graph/api/directoryobject-getbyids?view=graph-rest-1.0>
    */
    pub async fn users_get_ids(
        &self,
        body: &crate::types::UsersGetByIdsRequest,
    ) -> ClientResult<crate::types::Me> {
        let url = self.client.url(&"/users/getByIds".to_string(), None);
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
    * Invoke action validateProperties.
    *
    * This function performs a `POST` to the `/users/validateProperties` endpoint.
    *
    * Validate that a Microsoft 365 group's display name or mail nickname complies with naming policies.  Clients can use this API to determine whether a display name or mail nickname is valid before trying to create a Microsoft 365 group. To validate the properties of an existing group, use the group: validateProperties function. The following policy validations are performed for the display name and mail nickname properties:
    * 1. Validate the prefix and suffix naming policy
    * 2. Validate the custom banned words policy
    * 3. Validate that the mail nickname is unique This API only returns the first validation failure that is encountered. If the properties fail multiple validations, only the first validation failure is returned. However, you can validate both the mail nickname and the display name and receive a collection of validation errors if you are only validating the prefix and suffix naming policy. To learn more about configuring naming policies, see Configure naming policy.
    *
    * FROM: <https://docs.microsoft.com/graph/api/directoryobject-validateproperties?view=graph-rest-1.0>
    */
    pub async fn users_validate_properties(
        &self,
        body: &crate::types::UsersValidatePropertiesRequest,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url(&"/users/validateProperties".to_string(), None);
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
}
