use crate::Client;
use crate::ClientResult;

pub struct UsersUser {
    pub client: Client,
}

impl UsersUser {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersUser { client }
    }

    /**
    * List users.
    *
    * This function performs a `GET` to the `/users` endpoint.
    *
    * Retrieve a list of user objects.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-list?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `consistency_level: &str` -- Indicates the requested consistency level. Documentation URL: https://docs.microsoft.com/graph/aad-advanced-queries.
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn list(
        &self,
        consistency_level: &str,
        top: u64,
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
        if !top.to_string().is_empty() {
            query_args.push(("$top".to_string(), top.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/users?{}", query_), None);
        self.client
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
    * Create User.
    *
    * This function performs a `POST` to the `/users` endpoint.
    *
    * Create a new user.
    * The request body contains the user to create. At a minimum, you must specify the required properties for the user. You can optionally specify any other writable properties.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-post-users?view=graph-rest-1.0>
    */
    pub async fn create(
        &self,
        body: &crate::types::MicrosoftGraphUserAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphUserAllOf> {
        let url = self.client.url(&"/users".to_string(), None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get a user.
    *
    * This function performs a `GET` to the `/users/{user-id}` endpoint.
    *
    * Retrieve the properties and relationships of user object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn get(
        &self,
        user_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphUserAllOf> {
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
                "/users/{}?{}",
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
    * Delete a user.
    *
    * This function performs a `DELETE` to the `/users/{user-id}` endpoint.
    *
    * Delete user.   When deleted, user resources are moved to a temporary container and can be restored within 30 days.  After that time, they are permanently deleted.  To learn more, see deletedItems.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-delete?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn delete(&self, user_id: &str, if_match: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}",
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
    * Update user.
    *
    * This function performs a `PATCH` to the `/users/{user-id}` endpoint.
    *
    * Update the properties of a user object. Not all properties can be updated by Member or Guest users with their default permissions without Administrator roles. Compare member and guest default permissions to see properties they can manage.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-update?view=graph-rest-1.0>
    */
    pub async fn update(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphUserAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphUserAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}",
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
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `consistency_level: &str` -- Indicates the requested consistency level. Documentation URL: https://docs.microsoft.com/graph/aad-advanced-queries.
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_get_count_ee_47(
        &self,
        consistency_level: &str,
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
        let url = self.client.url(&format!("/users/$count?{}", query_), None);
        self.client
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
