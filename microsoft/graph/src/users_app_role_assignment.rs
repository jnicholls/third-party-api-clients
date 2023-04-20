use crate::Client;
use crate::ClientResult;

pub struct UsersAppRoleAssignment {
    pub client: Client,
}

impl UsersAppRoleAssignment {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersAppRoleAssignment { client }
    }

    /**
    * Get appRoleAssignments from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/appRoleAssignments` endpoint.
    *
    * Represents the app roles a user has been granted for an application. Supports $expand.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-list-approleassignments?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `consistency_level: &str` -- Indicates the requested consistency level. Documentation URL: https://docs.microsoft.com/graph/aad-advanced-queries.
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_list_app_role_assignment(
        &self,
        user_id: &str,
        consistency_level: &str,
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
                "/users/{}/appRoleAssignments?{}",
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
    * Grant an appRoleAssignment to a user.
    *
    * This function performs a `POST` to the `/users/{user-id}/appRoleAssignments` endpoint.
    *
    * Use this API to assign an app role to a user. To grant an app role assignment to a user, you need three identifiers:
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-post-approleassignments?view=graph-rest-1.0>
    */
    pub async fn users_create_app_role_assignments(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphAppRoleAssignmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAppRoleAssignmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/appRoleAssignments",
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
    * Get appRoleAssignments from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/appRoleAssignments/{appRoleAssignment-id}` endpoint.
    *
    * Represents the app roles a user has been granted for an application. Supports $expand.
    *
    * **Parameters:**
    *
    * * `consistency_level: &str` -- Indicates the requested consistency level. Documentation URL: https://docs.microsoft.com/graph/aad-advanced-queries.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_get_app_role_assignment(
        &self,
        user_id: &str,
        app_role_assignment_id: &str,
        consistency_level: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphAppRoleAssignmentAllOf> {
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
                "/users/{}/appRoleAssignments/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&app_role_assignment_id.to_string()),
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
    * Delete navigation property appRoleAssignments for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/appRoleAssignments/{appRoleAssignment-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_delete_app_role_assignments(
        &self,
        user_id: &str,
        app_role_assignment_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/appRoleAssignments/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&app_role_assignment_id.to_string()),
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
    * Update the navigation property appRoleAssignments in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/appRoleAssignments/{appRoleAssignment-id}` endpoint.
    */
    pub async fn users_update_app_role_assignments(
        &self,
        user_id: &str,
        app_role_assignment_id: &str,
        body: &crate::types::MicrosoftGraphAppRoleAssignmentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAppRoleAssignmentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/appRoleAssignments/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&app_role_assignment_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/appRoleAssignments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `consistency_level: &str` -- Indicates the requested consistency level. Documentation URL: https://docs.microsoft.com/graph/aad-advanced-queries.
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_get_count_d_180(
        &self,
        user_id: &str,
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
        let url = self.client.url(
            &format!(
                "/users/{}/appRoleAssignments/$count?{}",
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
