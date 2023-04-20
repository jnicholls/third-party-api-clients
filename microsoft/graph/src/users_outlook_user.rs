use crate::Client;
use crate::ClientResult;

pub struct UsersOutlookUser {
    pub client: Client,
}

impl UsersOutlookUser {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersOutlookUser { client }
    }

    /**
    * Get outlook from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/outlook` endpoint.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn users_get_outlook(
        &self,
        user_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOutlookUserAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/outlook?{}",
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
    * List Outlook categories.
    *
    * This function performs a `GET` to the `/users/{user-id}/outlook/masterCategories` endpoint.
    *
    * Get all the categories that have been defined for the user.
    *
    * FROM: <https://docs.microsoft.com/graph/api/outlookuser-list-mastercategories?view=graph-rest-1.0>
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
    pub async fn users_outlook_list_master_categorie(
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
                "/users/{}/outlook/masterCategories?{}",
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
    * Create Outlook category.
    *
    * This function performs a `POST` to the `/users/{user-id}/outlook/masterCategories` endpoint.
    *
    * Create an outlookCategory object in the user's master list of categories.
    *
    * FROM: <https://docs.microsoft.com/graph/api/outlookuser-post-mastercategories?view=graph-rest-1.0>
    */
    pub async fn users_outlook_create_master_categories(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphOutlookCategoryAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOutlookCategoryAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/outlook/masterCategories",
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
    * Get masterCategories from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/outlook/masterCategories/{outlookCategory-id}` endpoint.
    *
    * A list of categories defined for the user.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn users_outlook_get_master_categorie(
        &self,
        user_id: &str,
        outlook_category_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOutlookCategoryAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/outlook/masterCategories/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&outlook_category_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property masterCategories for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/outlook/masterCategories/{outlookCategory-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_outlook_delete_master_categories(
        &self,
        user_id: &str,
        outlook_category_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/outlook/masterCategories/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&outlook_category_id.to_string()),
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
    * Update the navigation property masterCategories in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/outlook/masterCategories/{outlookCategory-id}` endpoint.
    */
    pub async fn users_outlook_update_master_categories(
        &self,
        user_id: &str,
        outlook_category_id: &str,
        body: &crate::types::MicrosoftGraphOutlookCategoryAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOutlookCategoryAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/outlook/masterCategories/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&outlook_category_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/outlook/masterCategories/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_outlook_master_categories_get_count_8560(
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
                "/users/{}/outlook/masterCategories/$count?{}",
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
