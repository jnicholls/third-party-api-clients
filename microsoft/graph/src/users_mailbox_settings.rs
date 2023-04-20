use crate::Client;
use crate::ClientResult;

pub struct UsersMailboxSettings {
    pub client: Client,
}

impl UsersMailboxSettings {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersMailboxSettings { client }
    }

    /**
    * Get mailboxSettings property value.
    *
    * This function performs a `GET` to the `/users/{user-id}/mailboxSettings` endpoint.
    *
    * Settings for the primary mailbox of the signed-in user. You can get or update settings for sending automatic replies to incoming messages, locale and time zone. Returned only on $select.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_get_mailbox_setting(
        &self,
        user_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MailboxSettings> {
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
                "/users/{}/mailboxSettings?{}",
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
    * Update property mailboxSettings value.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/mailboxSettings` endpoint.
    */
    pub async fn users_update_mailbox_settings(
        &self,
        user_id: &str,
        body: &crate::types::MailboxSettings,
    ) -> ClientResult<crate::types::MailboxSettings> {
        let url = self.client.url(
            &format!(
                "/users/{}/mailboxSettings",
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
}
