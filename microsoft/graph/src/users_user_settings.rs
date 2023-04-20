use crate::Client;
use crate::ClientResult;

pub struct UsersUserSettings {
    pub client: Client,
}

impl UsersUserSettings {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersUserSettings { client }
    }

    /**
    * Get settings from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/settings` endpoint.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_get_setting(
        &self,
        user_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphUserSettingsAllOf> {
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
                "/users/{}/settings?{}",
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
    * Delete navigation property settings for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/settings` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_delete_settings(&self, user_id: &str, if_match: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/settings",
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
    * Update the navigation property settings in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/settings` endpoint.
    */
    pub async fn users_update_settings(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphUserSettingsAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphUserSettingsAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/settings",
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
    * Get shiftPreferences.
    *
    * This function performs a `GET` to the `/users/{user-id}/settings/shiftPreferences` endpoint.
    *
    * Retrieve the properties and relationships of a shiftPreferences object by ID.
    *
    * FROM: <https://docs.microsoft.com/graph/api/shiftpreferences-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_settings_get_shift_preference(
        &self,
        user_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphShiftPreferencesAllOf> {
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
                "/users/{}/settings/shiftPreferences?{}",
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
    * Delete navigation property shiftPreferences for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/settings/shiftPreferences` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_settings_delete_shift_preferences(
        &self,
        user_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/settings/shiftPreferences",
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
    * Update shiftPreferences.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/settings/shiftPreferences` endpoint.
    *
    * Update the properties and relationships of a shiftPreferences object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/shiftpreferences-put?view=graph-rest-1.0>
    */
    pub async fn users_settings_update_shift_preferences(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphShiftPreferencesAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphShiftPreferencesAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/settings/shiftPreferences",
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
