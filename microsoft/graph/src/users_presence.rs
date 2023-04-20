use crate::Client;
use crate::ClientResult;

pub struct UsersPresence {
    pub client: Client,
}

impl UsersPresence {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersPresence { client }
    }

    /**
    * Get presence.
    *
    * This function performs a `GET` to the `/users/{user-id}/presence` endpoint.
    *
    * Get a user's presence information.
    *
    * FROM: <https://docs.microsoft.com/graph/api/presence-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_get_presence(
        &self,
        user_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPresenceAllOf> {
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
                "/users/{}/presence?{}",
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
    * Delete navigation property presence for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/presence` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_delete_presence(&self, user_id: &str, if_match: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/presence",
                crate::progenitor_support::encode_path(&user_id.to_string()),
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
    * Update the navigation property presence in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/presence` endpoint.
    */
    pub async fn users_update_presence(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphPresenceAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPresenceAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/presence",
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
