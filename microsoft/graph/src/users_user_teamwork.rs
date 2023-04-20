use crate::Client;
use crate::ClientResult;

pub struct UsersUserTeamwork {
    pub client: Client,
}

impl UsersUserTeamwork {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersUserTeamwork { client }
    }

    /**
    * Get teamwork from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/teamwork` endpoint.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_get_teamwork(
        &self,
        user_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphUserTeamworkAllOf> {
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
                "/users/{}/teamwork?{}",
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
    * Delete navigation property teamwork for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/teamwork` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_delete_teamwork(&self, user_id: &str, if_match: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/teamwork",
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
    * Update the navigation property teamwork in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/teamwork` endpoint.
    */
    pub async fn users_update_teamwork(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphUserTeamworkAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphUserTeamworkAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/teamwork",
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
    * List associatedTeamInfo.
    *
    * This function performs a `GET` to the `/users/{user-id}/teamwork/associatedTeams` endpoint.
    *
    * Get the list of teams in Microsoft Teams that a user is associated with.
    * Currently, a user can be associated with a team in two different ways:
    * * A user can be a direct member of a team.
    * * A user can be a member of a shared channel that is hosted inside a team.
    *
    * FROM: <https://docs.microsoft.com/graph/api/associatedteaminfo-list?view=graph-rest-1.0>
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
    pub async fn users_teamwork_list_associated_team(
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
                "/users/{}/teamwork/associatedTeams?{}",
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
    * Create new navigation property to associatedTeams for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/teamwork/associatedTeams` endpoint.
    */
    pub async fn users_teamwork_create_associated_teams(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphAssociatedTeamInfoAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAssociatedTeamInfoAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/teamwork/associatedTeams",
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
    * Get associatedTeams from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/teamwork/associatedTeams/{associatedTeamInfo-id}` endpoint.
    *
    * The list of associatedTeamInfo objects that a user is associated with.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_teamwork_get_associated_team(
        &self,
        user_id: &str,
        associated_team_info_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphAssociatedTeamInfoAllOf> {
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
                "/users/{}/teamwork/associatedTeams/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&associated_team_info_id.to_string()),
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
    * Delete navigation property associatedTeams for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/teamwork/associatedTeams/{associatedTeamInfo-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_teamwork_delete_associated_teams(
        &self,
        user_id: &str,
        associated_team_info_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/teamwork/associatedTeams/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&associated_team_info_id.to_string()),
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
    * Update the navigation property associatedTeams in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/teamwork/associatedTeams/{associatedTeamInfo-id}` endpoint.
    */
    pub async fn users_teamwork_update_associated_teams(
        &self,
        user_id: &str,
        associated_team_info_id: &str,
        body: &crate::types::MicrosoftGraphAssociatedTeamInfoAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAssociatedTeamInfoAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/teamwork/associatedTeams/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&associated_team_info_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/teamwork/associatedTeams/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_teamwork_associated_teams_get_count_9_7ef(
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
                "/users/{}/teamwork/associatedTeams/$count?{}",
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
    * List apps installed for user.
    *
    * This function performs a `GET` to the `/users/{user-id}/teamwork/installedApps` endpoint.
    *
    * Retrieve the list of apps installed in the personal scope of the specified user.
    *
    * FROM: <https://docs.microsoft.com/graph/api/userteamwork-list-installedapps?view=graph-rest-1.0>
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
    pub async fn users_teamwork_list_installed_app(
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
                "/users/{}/teamwork/installedApps?{}",
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
    * Install app for user.
    *
    * This function performs a `POST` to the `/users/{user-id}/teamwork/installedApps` endpoint.
    *
    * Install an app in the personal scope of the specified user.
    *
    * FROM: <https://docs.microsoft.com/graph/api/userteamwork-post-installedapps?view=graph-rest-1.0>
    */
    pub async fn users_teamwork_create_installed_apps(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphUserScopeTeamsAppInstallationAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphUserScopeTeamsAppInstallationAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/teamwork/installedApps",
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
    * Get installedApps from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/teamwork/installedApps/{userScopeTeamsAppInstallation-id}` endpoint.
    *
    * The apps installed in the personal scope of this user.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_teamwork_get_installed_app(
        &self,
        user_id: &str,
        user_scope_teams_app_installation_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphUserScopeTeamsAppInstallationAllOf> {
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
                "/users/{}/teamwork/installedApps/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(
                    &user_scope_teams_app_installation_id.to_string()
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
    * Delete navigation property installedApps for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/teamwork/installedApps/{userScopeTeamsAppInstallation-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_teamwork_delete_installed_apps(
        &self,
        user_id: &str,
        user_scope_teams_app_installation_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/teamwork/installedApps/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(
                    &user_scope_teams_app_installation_id.to_string()
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
    * Update the navigation property installedApps in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/teamwork/installedApps/{userScopeTeamsAppInstallation-id}` endpoint.
    */
    pub async fn users_teamwork_update_installed_apps(
        &self,
        user_id: &str,
        user_scope_teams_app_installation_id: &str,
        body: &crate::types::MicrosoftGraphUserScopeTeamsAppInstallationAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphUserScopeTeamsAppInstallationAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/teamwork/installedApps/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(
                    &user_scope_teams_app_installation_id.to_string()
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
    * Get chat between user and teamsApp.
    *
    * This function performs a `GET` to the `/users/{user-id}/teamwork/installedApps/{userScopeTeamsAppInstallation-id}/chat` endpoint.
    *
    * Retrieve the chat of the specified user and Teams app.
    *
    * FROM: <https://docs.microsoft.com/graph/api/userscopeteamsappinstallation-get-chat?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_teamwork_installed_apps_get_chat(
        &self,
        user_id: &str,
        user_scope_teams_app_installation_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphChatAllOf> {
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
                "/users/{}/teamwork/installedApps/{}/chat?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(
                    &user_scope_teams_app_installation_id.to_string()
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
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/teamwork/installedApps/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_teamwork_installed_apps_get_count_0292(
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
                "/users/{}/teamwork/installedApps/$count?{}",
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
