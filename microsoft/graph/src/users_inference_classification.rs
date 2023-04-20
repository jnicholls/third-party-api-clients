use crate::Client;
use crate::ClientResult;

pub struct UsersInferenceClassification {
    pub client: Client,
}

impl UsersInferenceClassification {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersInferenceClassification { client }
    }

    /**
    * Get inferenceClassification from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/inferenceClassification` endpoint.
    *
    * Relevance classification of the user's messages based on explicit designations which override inferred relevance or importance.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn users_get_inference_classification(
        &self,
        user_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphInferenceClassificationAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/inferenceClassification?{}",
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
    * Update the navigation property inferenceClassification in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/inferenceClassification` endpoint.
    */
    pub async fn users_update_inference_classification(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphInferenceClassificationAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphInferenceClassificationAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/inferenceClassification",
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
    * List overrides.
    *
    * This function performs a `GET` to the `/users/{user-id}/inferenceClassification/overrides` endpoint.
    *
    * Get the overrides that a user has set up to always classify messages from certain senders in specific ways. Each override corresponds to an SMTP address of a sender. Initially, a user does not have any overrides.
    *
    * FROM: <https://docs.microsoft.com/graph/api/inferenceclassification-list-overrides?view=graph-rest-1.0>
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
    pub async fn list_override(
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
                "/users/{}/inferenceClassification/overrides?{}",
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
    * Create inferenceClassificationOverride.
    *
    * This function performs a `POST` to the `/users/{user-id}/inferenceClassification/overrides` endpoint.
    *
    * Create an override for a sender identified by an SMTP address. Future messages from that SMTP address will be consistently classified
    * as specified in the override. **Note**
    *
    * FROM: <https://docs.microsoft.com/graph/api/inferenceclassification-post-overrides?view=graph-rest-1.0>
    */
    pub async fn create_overrides(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphInferenceClassificationOverrideAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphInferenceClassificationOverrideAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/inferenceClassification/overrides",
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
    * Get overrides from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/inferenceClassification/overrides/{inferenceClassificationOverride-id}` endpoint.
    *
    * A set of overrides for a user to always classify messages from specific senders in certain ways: focused, or other. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn get_override(
        &self,
        user_id: &str,
        inference_classification_override_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphInferenceClassificationOverrideAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/inferenceClassification/overrides/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(
                    &inference_classification_override_id.to_string()
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
    * Delete navigation property overrides for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/inferenceClassification/overrides/{inferenceClassificationOverride-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn delete_overrides(
        &self,
        user_id: &str,
        inference_classification_override_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/inferenceClassification/overrides/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(
                    &inference_classification_override_id.to_string()
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
    * Update the navigation property overrides in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/inferenceClassification/overrides/{inferenceClassificationOverride-id}` endpoint.
    */
    pub async fn update_overrides(
        &self,
        user_id: &str,
        inference_classification_override_id: &str,
        body: &crate::types::MicrosoftGraphInferenceClassificationOverrideAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphInferenceClassificationOverrideAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/inferenceClassification/overrides/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(
                    &inference_classification_override_id.to_string()
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
    * This function performs a `GET` to the `/users/{user-id}/inferenceClassification/overrides/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn overrides_get_count_f_355(
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
                "/users/{}/inferenceClassification/overrides/$count?{}",
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
