use crate::Client;
use crate::ClientResult;

pub struct UsersProfilePhoto {
    pub client: Client,
}

impl UsersProfilePhoto {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersProfilePhoto { client }
    }

    /**
    * Get photo from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/photo` endpoint.
    *
    * The user's profile photo. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn users_get_photo(
        &self,
        user_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphProfilePhotoAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/photo?{}",
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
    * Delete navigation property photo for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/photo` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_delete_photo(&self, user_id: &str, if_match: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/photo",
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
    * Update the navigation property photo in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/photo` endpoint.
    */
    pub async fn users_update_photo(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphProfilePhotoAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphProfilePhotoAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/photo",
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
    * Get media content for the navigation property photo from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/photo/$value` endpoint.
    */
    pub async fn users_get_photo_content(&self, user_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/photo/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Update media content for the navigation property photo in users.
    *
    * This function performs a `PUT` to the `/users/{user-id}/photo/$value` endpoint.
    */
    pub async fn users_update_photo_content<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        body: B,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/photo/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
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
    * Get photos from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/photos` endpoint.
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
    pub async fn users_list_photo(
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
                "/users/{}/photos?{}",
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
    * Get photos from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/photos/{profilePhoto-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn users_get_photo_users_profile_photo(
        &self,
        user_id: &str,
        profile_photo_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphProfilePhotoAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/photos/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&profile_photo_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get media content for the navigation property photos from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/photos/{profilePhoto-id}/$value` endpoint.
    */
    pub async fn users_get_photos_content(
        &self,
        user_id: &str,
        profile_photo_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/photos/{}/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&profile_photo_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Update media content for the navigation property photos in users.
    *
    * This function performs a `PUT` to the `/users/{user-id}/photos/{profilePhoto-id}/$value` endpoint.
    */
    pub async fn users_update_photos_content<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        profile_photo_id: &str,
        body: B,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/photos/{}/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&profile_photo_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/photos/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_photos_get_count_5dad(
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
                "/users/{}/photos/$count?{}",
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
