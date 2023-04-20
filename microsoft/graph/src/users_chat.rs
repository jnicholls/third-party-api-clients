use crate::Client;
use crate::ClientResult;

pub struct UsersChat {
    pub client: Client,
}

impl UsersChat {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersChat { client }
    }

    /**
    * List chats.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats` endpoint.
    *
    * Retrieve the list of chats that the user is part of. This method supports federation. When a user ID is provided, the calling application must belong to the same tenant that the user belongs to.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-list?view=graph-rest-1.0>
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
    pub async fn users_list_chat(
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
                "/users/{}/chats?{}",
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
    * Create new navigation property to chats for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats` endpoint.
    */
    pub async fn users_create_chats(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphChatAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats",
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
    * Get chats from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_get_chat(
        &self,
        user_id: &str,
        chat_id: &str,
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
                "/users/{}/chats/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * Delete navigation property chats for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/chats/{chat-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_delete_chats(
        &self,
        user_id: &str,
        chat_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * Update the navigation property chats in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/chats/{chat-id}` endpoint.
    */
    pub async fn users_update_chats(
        &self,
        user_id: &str,
        chat_id: &str,
        body: &crate::types::MicrosoftGraphChatAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * List apps in chat.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/installedApps` endpoint.
    *
    * List all app installations within a chat.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-list-installedapps?view=graph-rest-1.0>
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
    pub async fn s_list_installed_app(
        &self,
        user_id: &str,
        chat_id: &str,
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
                "/users/{}/chats/{}/installedApps?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * Add app to chat.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/installedApps` endpoint.
    *
    * Install a teamsApp to the specified chat.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-post-installedapps?view=graph-rest-1.0>
    */
    pub async fn s_create_installed_apps(
        &self,
        user_id: &str,
        chat_id: &str,
        body: &crate::types::MicrosoftGraphTeamsAppInstallationAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsAppInstallationAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/installedApps",
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
    * Get installedApps from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/installedApps/{teamsAppInstallation-id}` endpoint.
    *
    * A collection of all the apps in the chat. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_installed_app(
        &self,
        user_id: &str,
        chat_id: &str,
        teams_app_installation_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsAppInstallationAllOf> {
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
                "/users/{}/chats/{}/installedApps/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&teams_app_installation_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/chats/{chat-id}/installedApps/{teamsAppInstallation-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_installed_apps(
        &self,
        user_id: &str,
        chat_id: &str,
        teams_app_installation_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/installedApps/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&teams_app_installation_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/chats/{chat-id}/installedApps/{teamsAppInstallation-id}` endpoint.
    */
    pub async fn s_update_installed_apps(
        &self,
        user_id: &str,
        chat_id: &str,
        teams_app_installation_id: &str,
        body: &crate::types::MicrosoftGraphTeamsAppInstallationAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsAppInstallationAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/installedApps/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&teams_app_installation_id.to_string()),
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
    * Get teamsApp from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/installedApps/{teamsAppInstallation-id}/teamsApp` endpoint.
    *
    * The app that is installed.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_installed_apps_get_teams_app(
        &self,
        user_id: &str,
        chat_id: &str,
        teams_app_installation_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsAppAllOf> {
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
                "/users/{}/chats/{}/installedApps/{}/teamsApp?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&teams_app_installation_id.to_string()),
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
    * Get teamsAppDefinition from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/installedApps/{teamsAppInstallation-id}/teamsAppDefinition` endpoint.
    *
    * The details of this version of the app.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_installed_apps_get_teams_app_definition(
        &self,
        user_id: &str,
        chat_id: &str,
        teams_app_installation_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsAppDefinitionAllOf> {
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
                "/users/{}/chats/{}/installedApps/{}/teamsAppDefinition?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&teams_app_installation_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/installedApps/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_installed_apps_get_count_0d_67(
        &self,
        user_id: &str,
        chat_id: &str,
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
                "/users/{}/chats/{}/installedApps/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * Get lastMessagePreview from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/lastMessagePreview` endpoint.
    *
    * Preview of the last message sent in the chat. Null if no messages have been sent in the chat. Currently, only the list chats operation supports this property.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_last_message_preview(
        &self,
        user_id: &str,
        chat_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageInfoAllOf> {
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
                "/users/{}/chats/{}/lastMessagePreview?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * Delete navigation property lastMessagePreview for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/chats/{chat-id}/lastMessagePreview` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_last_message_preview(
        &self,
        user_id: &str,
        chat_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/lastMessagePreview",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * Update the navigation property lastMessagePreview in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/chats/{chat-id}/lastMessagePreview` endpoint.
    */
    pub async fn s_update_last_message_preview(
        &self,
        user_id: &str,
        chat_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageInfoAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageInfoAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/lastMessagePreview",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * List members of a chat.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/members` endpoint.
    *
    * List all conversation members in a chat. This method supports federation. For one-on-one chats, at least one chat member must belong to the tenant the request initiates from. For group chats, the chat must be initiated by a user in the tenant the request initiates from.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-list-members?view=graph-rest-1.0>
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
    pub async fn s_list_member(
        &self,
        user_id: &str,
        chat_id: &str,
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
                "/users/{}/chats/{}/members?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * Add member to a chat.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/members` endpoint.
    *
    * Add a conversationMember to a chat.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-post-members?view=graph-rest-1.0>
    */
    pub async fn s_create_members(
        &self,
        user_id: &str,
        chat_id: &str,
        body: &crate::types::MicrosoftGraphConversationMemberAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphConversationMemberAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/members",
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
    * Get members from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/members/{conversationMember-id}` endpoint.
    *
    * A collection of all the members in the chat. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_member(
        &self,
        user_id: &str,
        chat_id: &str,
        conversation_member_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphConversationMemberAllOf> {
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
                "/users/{}/chats/{}/members/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&conversation_member_id.to_string()),
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
    * Delete navigation property members for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/chats/{chat-id}/members/{conversationMember-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_members(
        &self,
        user_id: &str,
        chat_id: &str,
        conversation_member_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/members/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&conversation_member_id.to_string()),
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
    * Update the navigation property members in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/chats/{chat-id}/members/{conversationMember-id}` endpoint.
    */
    pub async fn s_update_members(
        &self,
        user_id: &str,
        chat_id: &str,
        conversation_member_id: &str,
        body: &crate::types::MicrosoftGraphConversationMemberAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphConversationMemberAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/members/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&conversation_member_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/members/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_members_get_count_b_2_5d(
        &self,
        user_id: &str,
        chat_id: &str,
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
                "/users/{}/chats/{}/members/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * List messages in a chat.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/messages` endpoint.
    *
    * Retrieve the list of messages in a chat. This method supports federation. To list chat messages in application context, the request must be made from the tenant that the channel owner belongs to (represented by the **tenantId** property on the channel).
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-list-messages?view=graph-rest-1.0>
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
    pub async fn s_list_message(
        &self,
        user_id: &str,
        chat_id: &str,
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
                "/users/{}/chats/{}/messages?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * Send message in a chat.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/messages` endpoint.
    *
    * Send a new chatMessage in the specified chat. This API can't create a new chat; you must use the list chats method to retrieve the ID of an existing chat before you can create a chat message.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-post-messages?view=graph-rest-1.0>
    */
    pub async fn s_create_messages(
        &self,
        user_id: &str,
        chat_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages",
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
    * Get messages from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}` endpoint.
    *
    * A collection of all the messages in the chat. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_message(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageAllOf> {
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
                "/users/{}/chats/{}/messages/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
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
    * Delete navigation property messages for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_messages(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
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
    * Update the navigation property messages in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}` endpoint.
    */
    pub async fn s_update_messages(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
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
    * List hostedContents.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/hostedContents` endpoint.
    *
    * Retrieve the list of chatMessageHostedContent objects from a message. This API only lists the hosted content objects. To get the content bytes, see get chatmessage hosted content
    *
    * FROM: <https://docs.microsoft.com/graph/api/chatmessage-list-hostedcontents?view=graph-rest-1.0>
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
    pub async fn s_messages_list_hosted_content(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
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
                "/users/{}/chats/{}/messages/{}/hostedContents?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
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
    * Create new navigation property to hostedContents for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/hostedContents` endpoint.
    */
    pub async fn s_messages_create_hosted_contents(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageHostedContentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageHostedContentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}/hostedContents",
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
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get hostedContents from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    *
    * Content in a message hosted by Microsoft Teams - for example, images or code snippets.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_messages_get_hosted_content(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_hosted_content_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageHostedContentAllOf> {
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
                "/users/{}/chats/{}/messages/{}/hostedContents/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_hosted_content_id.to_string()),
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
    * Delete navigation property hostedContents for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_messages_delete_hosted_contents(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_hosted_content_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}/hostedContents/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_hosted_content_id.to_string()),
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
    * Update the navigation property hostedContents in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    */
    pub async fn s_messages_update_hosted_contents(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_hosted_content_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageHostedContentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageHostedContentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}/hostedContents/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_hosted_content_id.to_string()),
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
    * Get media content for the navigation property hostedContents from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/hostedContents/{chatMessageHostedContent-id}/$value` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chatmessage-list-hostedcontents?view=graph-rest-1.0>
    */
    pub async fn s_messages_get_hosted_contents_content(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_hosted_content_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}/hostedContents/{}/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_hosted_content_id.to_string()),
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
    * Update media content for the navigation property hostedContents in users.
    *
    * This function performs a `PUT` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/hostedContents/{chatMessageHostedContent-id}/$value` endpoint.
    */
    pub async fn s_messages_update_hosted_contents_content<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_hosted_content_id: &str,
        body: B,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}/hostedContents/{}/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_hosted_content_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/hostedContents/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_messages_hosted_contents_get_count_e_922(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
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
                "/users/{}/chats/{}/messages/{}/hostedContents/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
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
    * List replies.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/replies` endpoint.
    *
    * List all the replies to a message in a channel of a team. This method lists only the replies of the specified message, if any. To get the message itself, simply call get channel message.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chatmessage-list-replies?view=graph-rest-1.0>
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
    pub async fn s_messages_list_replie(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
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
                "/users/{}/chats/{}/messages/{}/replies?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
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
    * Reply to a message in a channel.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/replies` endpoint.
    *
    * Create a new reply to a chatMessage in a specified channel.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-post-messagereply?view=graph-rest-1.0>
    */
    pub async fn s_messages_create_replies(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}/replies",
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
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get replies from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}` endpoint.
    *
    * Replies for a specified message. Supports $expand for channel messages.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_messages_get_replie(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageAllOf> {
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
                "/users/{}/chats/{}/messages/{}/replies/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),
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
    * Delete navigation property replies for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_messages_delete_replies(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}/replies/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),
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
    * Update the navigation property replies in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}` endpoint.
    */
    pub async fn s_messages_update_replies(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        body: &crate::types::MicrosoftGraphChatMessageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}/replies/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),
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
    * List hostedContents.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents` endpoint.
    *
    * Retrieve the list of chatMessageHostedContent objects from a message. This API only lists the hosted content objects. To get the content bytes, see get chatmessage hosted content
    *
    * FROM: <https://docs.microsoft.com/graph/api/chatmessage-list-hostedcontents?view=graph-rest-1.0>
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
    pub async fn s_messages_replies_list_hosted_content(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
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
                "/users/{}/chats/{}/messages/{}/replies/{}/hostedContents?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),
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
    * Create new navigation property to hostedContents for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents` endpoint.
    */
    pub async fn s_messages_replies_create_hosted_contents(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        body: &crate::types::MicrosoftGraphChatMessageHostedContentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageHostedContentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}/replies/{}/hostedContents",
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
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Get hostedContents from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    *
    * Content in a message hosted by Microsoft Teams - for example, images or code snippets.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_messages_replies_get_hosted_content(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        chat_message_hosted_content_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageHostedContentAllOf> {
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
                "/users/{}/chats/{}/messages/{}/replies/{}/hostedContents/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),
                crate::progenitor_support::encode_path(&chat_message_hosted_content_id.to_string()),
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
    * Delete navigation property hostedContents for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_messages_replies_delete_hosted_contents(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        chat_message_hosted_content_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}/replies/{}/hostedContents/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),
                crate::progenitor_support::encode_path(&chat_message_hosted_content_id.to_string()),
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
    * Update the navigation property hostedContents in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    */
    pub async fn s_messages_replies_update_hosted_contents(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        chat_message_hosted_content_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageHostedContentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageHostedContentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}/replies/{}/hostedContents/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),
                crate::progenitor_support::encode_path(&chat_message_hosted_content_id.to_string()),
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
    * Get media content for the navigation property hostedContents from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/{chatMessageHostedContent-id}/$value` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chatmessage-list-hostedcontents?view=graph-rest-1.0>
    */
    pub async fn s_messages_replies_get_hosted_contents_content(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        chat_message_hosted_content_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}/replies/{}/hostedContents/{}/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),
                crate::progenitor_support::encode_path(&chat_message_hosted_content_id.to_string()),
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
    * Update media content for the navigation property hostedContents in users.
    *
    * This function performs a `PUT` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/{chatMessageHostedContent-id}/$value` endpoint.
    */
    pub async fn s_messages_replies_update_hosted_contents_content<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        chat_message_hosted_content_id: &str,
        body: B,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/messages/{}/replies/{}/hostedContents/{}/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),
                crate::progenitor_support::encode_path(&chat_message_hosted_content_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_messages_replies_hosted_contents_get_count_3_4f_5(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
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
                "/users/{}/chats/{}/messages/{}/replies/{}/hostedContents/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/messages/{chatMessage-id}/replies/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_messages_replies_get_count_cecc(
        &self,
        user_id: &str,
        chat_id: &str,
        chat_message_id: &str,
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
                "/users/{}/chats/{}/messages/{}/replies/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&chat_message_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/messages/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_messages_get_count_c_9_5d(
        &self,
        user_id: &str,
        chat_id: &str,
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
                "/users/{}/chats/{}/messages/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * List pinnedChatMessages in a chat.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/pinnedMessages` endpoint.
    *
    * Get a list of pinnedChatMessages in a chat.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-list-pinnedmessages?view=graph-rest-1.0>
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
    pub async fn s_list_pinned_message(
        &self,
        user_id: &str,
        chat_id: &str,
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
                "/users/{}/chats/{}/pinnedMessages?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * Pin a message in a chat.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/pinnedMessages` endpoint.
    *
    * Pin a chat message in the specified chat. This API cannot create a new chat; you must use the list chats method to retrieve the ID of an existing chat before you can pin a chat message.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-post-pinnedmessages?view=graph-rest-1.0>
    */
    pub async fn s_create_pinned_messages(
        &self,
        user_id: &str,
        chat_id: &str,
        body: &crate::types::MicrosoftGraphPinnedChatMessageInfoAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPinnedChatMessageInfoAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/pinnedMessages",
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
    * Get pinnedMessages from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/pinnedMessages/{pinnedChatMessageInfo-id}` endpoint.
    *
    * A collection of all the pinned messages in the chat. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_pinned_message(
        &self,
        user_id: &str,
        chat_id: &str,
        pinned_chat_message_info_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPinnedChatMessageInfoAllOf> {
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
                "/users/{}/chats/{}/pinnedMessages/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&pinned_chat_message_info_id.to_string()),
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
    * Delete navigation property pinnedMessages for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/chats/{chat-id}/pinnedMessages/{pinnedChatMessageInfo-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_pinned_messages(
        &self,
        user_id: &str,
        chat_id: &str,
        pinned_chat_message_info_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/pinnedMessages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&pinned_chat_message_info_id.to_string()),
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
    * Update the navigation property pinnedMessages in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/chats/{chat-id}/pinnedMessages/{pinnedChatMessageInfo-id}` endpoint.
    */
    pub async fn s_update_pinned_messages(
        &self,
        user_id: &str,
        chat_id: &str,
        pinned_chat_message_info_id: &str,
        body: &crate::types::MicrosoftGraphPinnedChatMessageInfoAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPinnedChatMessageInfoAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/pinnedMessages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&pinned_chat_message_info_id.to_string()),
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
    * Get message from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/pinnedMessages/{pinnedChatMessageInfo-id}/message` endpoint.
    *
    * Represents details about the chat message that is pinned.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_pinned_messages_get_message(
        &self,
        user_id: &str,
        chat_id: &str,
        pinned_chat_message_info_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageAllOf> {
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
                "/users/{}/chats/{}/pinnedMessages/{}/message?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&pinned_chat_message_info_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/pinnedMessages/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_pinned_messages_get_count_bc_2d(
        &self,
        user_id: &str,
        chat_id: &str,
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
                "/users/{}/chats/{}/pinnedMessages/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * List tabs in chat.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/tabs` endpoint.
    *
    * Retrieve the list of tabs in the specified chat.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-list-tabs?view=graph-rest-1.0>
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
    pub async fn s_list_tab(
        &self,
        user_id: &str,
        chat_id: &str,
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
                "/users/{}/chats/{}/tabs?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * Add tab to chat.
    *
    * This function performs a `POST` to the `/users/{user-id}/chats/{chat-id}/tabs` endpoint.
    *
    * Add (pin) a tab to the specified chat.
    * The corresponding app must already be installed in the chat.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chat-post-tabs?view=graph-rest-1.0>
    */
    pub async fn s_create_tabs(
        &self,
        user_id: &str,
        chat_id: &str,
        body: &crate::types::MicrosoftGraphTeamsTabAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsTabAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/tabs",
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
    * Get tabs from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/tabs/{teamsTab-id}` endpoint.
    *
    * A collection of all the tabs in the chat. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_tab(
        &self,
        user_id: &str,
        chat_id: &str,
        teams_tab_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsTabAllOf> {
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
                "/users/{}/chats/{}/tabs/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&teams_tab_id.to_string()),
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
    * Delete navigation property tabs for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/chats/{chat-id}/tabs/{teamsTab-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_tabs(
        &self,
        user_id: &str,
        chat_id: &str,
        teams_tab_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/tabs/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&teams_tab_id.to_string()),
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
    * Update the navigation property tabs in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/chats/{chat-id}/tabs/{teamsTab-id}` endpoint.
    */
    pub async fn s_update_tabs(
        &self,
        user_id: &str,
        chat_id: &str,
        teams_tab_id: &str,
        body: &crate::types::MicrosoftGraphTeamsTabAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsTabAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/chats/{}/tabs/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&teams_tab_id.to_string()),
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
    * Get teamsApp from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/tabs/{teamsTab-id}/teamsApp` endpoint.
    *
    * The application that is linked to the tab. This cannot be changed after tab creation.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_tabs_get_teams_app(
        &self,
        user_id: &str,
        chat_id: &str,
        teams_tab_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsAppAllOf> {
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
                "/users/{}/chats/{}/tabs/{}/teamsApp?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
                crate::progenitor_support::encode_path(&teams_tab_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/chats/{chat-id}/tabs/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_tabs_get_count_bf_26(
        &self,
        user_id: &str,
        chat_id: &str,
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
                "/users/{}/chats/{}/tabs/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&chat_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/chats/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_get_count_3_8c_2(
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
                "/users/{}/chats/$count?{}",
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
