use crate::Client;
use crate::ClientResult;

pub struct UsersTeam {
    pub client: Client,
}

impl UsersTeam {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersTeam { client }
    }

    /**
    * List joinedTeams.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams` endpoint.
    *
    * Get the teams in Microsoft Teams that the user is a direct member of.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-list-joinedteams?view=graph-rest-1.0>
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
    pub async fn users_list_joined_team(
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
                "/users/{}/joinedTeams?{}",
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
    * Create new navigation property to joinedTeams for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams` endpoint.
    */
    pub async fn users_create_joined_teams(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphTeamAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams",
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
    * Get joinedTeams from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_get_joined_team(
        &self,
        user_id: &str,
        team_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphTeamAllOf> {
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
                "/users/{}/joinedTeams/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Delete navigation property joinedTeams for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_delete_joined_teams(
        &self,
        user_id: &str,
        team_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Update the navigation property joinedTeams in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}` endpoint.
    */
    pub async fn users_update_joined_teams(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphTeamAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List allChannels.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/allChannels` endpoint.
    *
    * Get the list of channels either in this team or shared with this team (incoming channels).
    *
    * FROM: <https://docs.microsoft.com/graph/api/team-list-allchannels?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_list_all_channel(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/allChannels?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get allChannels from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/allChannels/{channel-id}` endpoint.
    *
    * List of channels either hosted in or shared with the team (incoming channels).
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_get_all_channel(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphChannelAllOf> {
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
                "/users/{}/joinedTeams/{}/allChannels/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/allChannels/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_all_channels_get_count_3_3d_5(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/allChannels/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List channels.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels` endpoint.
    *
    * Retrieve the list of channels in this team.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-list?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_list_channel(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/channels?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Create channel.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/channels` endpoint.
    *
    * Create a new channel in a team, as specified in the request body.  When you create a channel, the maximum length of the channel's `displayName` is 50 characters. This is the name that appears to the user in Microsoft Teams. If you're creating a private channel, you can add a maximum of 200 members.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-post?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_create_channels(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphChannelAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChannelAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get channels from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}` endpoint.
    *
    * The collection of channels and messages associated with the team.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_get_channel(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphChannelAllOf> {
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
                "/users/{}/joinedTeams/{}/channels/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * Delete navigation property channels for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_delete_channels(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * Update the navigation property channels in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}` endpoint.
    */
    pub async fn users_joined_teams_update_channels(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        body: &crate::types::MicrosoftGraphChannelAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChannelAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * Get filesFolder.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/filesFolder` endpoint.
    *
    * Get the metadata for the location where the files of a channel are stored.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-get-filesfolder?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_channels_get_files_folder(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphDriveItemAllOf> {
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
                "/users/{}/joinedTeams/{}/channels/{}/filesFolder?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * Get content for the navigation property filesFolder from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/filesFolder/content` endpoint.
    *
    * The content stream, if the item represents a file.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-get-filesfolder?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_channels_get_files_folder_content(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/filesFolder/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * Update content for the navigation property filesFolder in users.
    *
    * This function performs a `PUT` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/filesFolder/content` endpoint.
    *
    * The content stream, if the item represents a file.
    */
    pub async fn users_joined_teams_channels_update_files_folder_content<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        body: B,
    ) -> ClientResult<crate::types::MicrosoftGraphDriveItemAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/filesFolder/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * List members of a channel.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/members` endpoint.
    *
    * Retrieve a list of conversationMembers from a channel. This method supports federation. Only a user who is a member of the shared channel can retrieve the channel member list.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-list-members?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_channels_list_member(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/members?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * Add member to channel.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/members` endpoint.
    *
    * Add a conversationMember to a channel. This operation is allowed only for channels with a **membershipType** value of `private` or `shared`.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-post-members?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_channels_create_members(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        body: &crate::types::MicrosoftGraphConversationMemberAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphConversationMemberAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/members",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/members/{conversationMember-id}` endpoint.
    *
    * A collection of membership records associated with the channel.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_channels_get_member(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/members/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/members/{conversationMember-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_channels_delete_members(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        conversation_member_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/members/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/members/{conversationMember-id}` endpoint.
    */
    pub async fn users_joined_teams_channels_update_members(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        conversation_member_id: &str,
        body: &crate::types::MicrosoftGraphConversationMemberAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphConversationMemberAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/members/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/members/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_channels_members_get_count_ac_8d(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/members/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * List channel messages.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages` endpoint.
    *
    * Retrieve the list of messages (without the replies) in a channel of a team.  To get the replies for a message, call the list message replies or the get message reply API.  This method supports federation. To list channel messages in application context, the request must be made from the tenant that the channel owner belongs to (represented by the **tenantId** property on the channel).
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-list-messages?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_channels_list_message(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/messages?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * Send chatMessage in channel.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages` endpoint.
    *
    * Send a new chatMessage in the specified channel.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-post-messages?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_channels_create_messages(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}` endpoint.
    *
    * A collection of all the messages in the channel. A navigation property. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_channels_get_message(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_channels_delete_messages(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}` endpoint.
    */
    pub async fn users_joined_teams_channels_update_messages(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/hostedContents` endpoint.
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
    pub async fn users_joined_teams_channels_messages_list_hosted_content(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/hostedContents?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/hostedContents` endpoint.
    */
    pub async fn users_joined_teams_channels_messages_create_hosted_contents(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageHostedContentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageHostedContentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/hostedContents",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    *
    * Content in a message hosted by Microsoft Teams - for example, images or code snippets.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_channels_messages_get_hosted_content(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/hostedContents/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_channels_messages_delete_hosted_contents(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
        chat_message_hosted_content_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/hostedContents/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    */
    pub async fn users_joined_teams_channels_messages_update_hosted_contents(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
        chat_message_hosted_content_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageHostedContentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageHostedContentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/hostedContents/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/hostedContents/{chatMessageHostedContent-id}/$value` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chatmessage-list-hostedcontents?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_channels_messages_get_hosted_contents_content(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
        chat_message_hosted_content_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/hostedContents/{}/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `PUT` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/hostedContents/{chatMessageHostedContent-id}/$value` endpoint.
    */
    pub async fn users_joined_teams_channels_messages_update_hosted_contents_content<
        B: Into<reqwest::Body>,
    >(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
        chat_message_hosted_content_id: &str,
        body: B,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/hostedContents/{}/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/hostedContents/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_channels_messages_hosted_contents_get_count_414(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/hostedContents/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/replies` endpoint.
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
    pub async fn users_joined_teams_channels_messages_list_replie(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/replies?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/replies` endpoint.
    *
    * Create a new reply to a chatMessage in a specified channel.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-post-messagereply?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_channels_messages_create_replies(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/replies",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}` endpoint.
    *
    * Replies for a specified message. Supports $expand for channel messages.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_channels_messages_get_replie(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/replies/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_channels_messages_delete_replies(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/replies/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}` endpoint.
    */
    pub async fn users_joined_teams_channels_messages_update_replies(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        body: &crate::types::MicrosoftGraphChatMessageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/replies/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents` endpoint.
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
    pub async fn users_joined_teams_channels_messages_replies_list_hosted_content(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/replies/{}/hostedContents?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents` endpoint.
    */
    pub async fn users_joined_teams_channels_messages_replies_create_hosted_contents(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        body: &crate::types::MicrosoftGraphChatMessageHostedContentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageHostedContentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/replies/{}/hostedContents",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    *
    * Content in a message hosted by Microsoft Teams - for example, images or code snippets.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_channels_messages_replies_get_hosted_content(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/replies/{}/hostedContents/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_channels_messages_replies_delete_hosted_contents(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        chat_message_hosted_content_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/replies/{}/hostedContents/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    */
    pub async fn users_joined_teams_channels_messages_replies_update_hosted_contents(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        chat_message_hosted_content_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageHostedContentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageHostedContentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/replies/{}/hostedContents/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/{chatMessageHostedContent-id}/$value` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chatmessage-list-hostedcontents?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_channels_messages_replies_get_hosted_contents_content(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        chat_message_hosted_content_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/joinedTeams/{}/channels/{}/messages/{}/replies/{}/hostedContents/{}/$value",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&team_id.to_string()),crate::progenitor_support::encode_path(&channel_id.to_string()),crate::progenitor_support::encode_path(&chat_message_id.to_string()),crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),crate::progenitor_support::encode_path(&chat_message_hosted_content_id.to_string()),), None);
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
    * This function performs a `PUT` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/{chatMessageHostedContent-id}/$value` endpoint.
    */
    pub async fn users_joined_teams_channels_messages_replies_update_hosted_contents_content<
        B: Into<reqwest::Body>,
    >(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        chat_message_hosted_content_id: &str,
        body: B,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/joinedTeams/{}/channels/{}/messages/{}/replies/{}/hostedContents/{}/$value",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&team_id.to_string()),crate::progenitor_support::encode_path(&channel_id.to_string()),crate::progenitor_support::encode_path(&chat_message_id.to_string()),crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),crate::progenitor_support::encode_path(&chat_message_hosted_content_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_channels_messages_replies_hosted_contents_get_count_9464(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
&format!("/users/{}/joinedTeams/{}/channels/{}/messages/{}/replies/{}/hostedContents/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&team_id.to_string()),crate::progenitor_support::encode_path(&channel_id.to_string()),crate::progenitor_support::encode_path(&chat_message_id.to_string()),crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/{chatMessage-id}/replies/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_channels_messages_replies_get_count_3de_6(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/messages/{}/replies/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/messages/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_channels_messages_get_count_9955(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/messages/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * List sharedWithChannelTeamInfo.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/sharedWithTeams` endpoint.
    *
    * Get the list of teams that has been shared a specified channel. This operation is allowed only for channels with a **membershipType** value of `shared`.
    *
    * FROM: <https://docs.microsoft.com/graph/api/sharedwithchannelteaminfo-list?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_channels_list_shared(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/sharedWithTeams?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * Create new navigation property to sharedWithTeams for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/sharedWithTeams` endpoint.
    */
    pub async fn users_joined_teams_channels_create_shared(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        body: &crate::types::MicrosoftGraphSharedWithChannelTeamInfoAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSharedWithChannelTeamInfoAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/sharedWithTeams",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * Get sharedWithTeams from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/sharedWithTeams/{sharedWithChannelTeamInfo-id}` endpoint.
    *
    * A collection of teams with which a channel is shared.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_channels_get_shared(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        shared_with_channel_team_info_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSharedWithChannelTeamInfoAllOf> {
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
                "/users/{}/joinedTeams/{}/channels/{}/sharedWithTeams/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
                crate::progenitor_support::encode_path(
                    &shared_with_channel_team_info_id.to_string()
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
    * Delete navigation property sharedWithTeams for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/sharedWithTeams/{sharedWithChannelTeamInfo-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_channels_delete_shared(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        shared_with_channel_team_info_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/sharedWithTeams/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
                crate::progenitor_support::encode_path(
                    &shared_with_channel_team_info_id.to_string()
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
    * Update the navigation property sharedWithTeams in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/sharedWithTeams/{sharedWithChannelTeamInfo-id}` endpoint.
    */
    pub async fn users_joined_teams_channels_update_shared(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        shared_with_channel_team_info_id: &str,
        body: &crate::types::MicrosoftGraphSharedWithChannelTeamInfoAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSharedWithChannelTeamInfoAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/sharedWithTeams/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
                crate::progenitor_support::encode_path(
                    &shared_with_channel_team_info_id.to_string()
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
    * List allowedMembers.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/sharedWithTeams/{sharedWithChannelTeamInfo-id}/allowedMembers` endpoint.
    *
    * Get the list of conversationMembers who can access a shared channel. This method does not return the following conversationMembers from the team:
    * - Users with `Guest` role
    * - Users who are externally authenticated in the tenant
    *
    * FROM: <https://docs.microsoft.com/graph/api/sharedwithchannelteaminfo-list-allowedmembers?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_channels_shared_list_allowed_member(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        shared_with_channel_team_info_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/sharedWithTeams/{}/allowedMembers?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
                crate::progenitor_support::encode_path(
                    &shared_with_channel_team_info_id.to_string()
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
    * Get allowedMembers from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/sharedWithTeams/{sharedWithChannelTeamInfo-id}/allowedMembers/{conversationMember-id}` endpoint.
    *
    * A collection of team members who have access to the shared channel.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_channels_shared_get_allowed_member(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        shared_with_channel_team_info_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/sharedWithTeams/{}/allowedMembers/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
                crate::progenitor_support::encode_path(
                    &shared_with_channel_team_info_id.to_string()
                ),
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
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/sharedWithTeams/{sharedWithChannelTeamInfo-id}/allowedMembers/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_channels_shared_allowed_members_get_count_8dec(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        shared_with_channel_team_info_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/sharedWithTeams/{}/allowedMembers/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
                crate::progenitor_support::encode_path(
                    &shared_with_channel_team_info_id.to_string()
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/sharedWithTeams/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_channels_shared_get_count_3df_3(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/sharedWithTeams/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * List tabs in channel.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/tabs` endpoint.
    *
    * Retrieve the list of tabs in the specified channel within a team.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-list-tabs?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_channels_list_tab(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/tabs?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * Create new navigation property to tabs for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/tabs` endpoint.
    */
    pub async fn users_joined_teams_channels_create_tabs(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        body: &crate::types::MicrosoftGraphTeamsTabAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsTabAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/tabs",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/tabs/{teamsTab-id}` endpoint.
    *
    * A collection of all the tabs in the channel. A navigation property.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_channels_get_tab(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/tabs/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/tabs/{teamsTab-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_channels_delete_tabs(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        teams_tab_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/tabs/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/tabs/{teamsTab-id}` endpoint.
    */
    pub async fn users_joined_teams_channels_update_tabs(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        teams_tab_id: &str,
        body: &crate::types::MicrosoftGraphTeamsTabAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsTabAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/channels/{}/tabs/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/tabs/{teamsTab-id}/teamsApp` endpoint.
    *
    * The application that is linked to the tab. This cannot be changed after tab creation.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_channels_tabs_get_app(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/tabs/{}/teamsApp?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/{channel-id}/tabs/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_channels_tabs_get_count_6808(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/{}/tabs/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/channels/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_channels_get_count_e_058(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/channels/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get group from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/group` endpoint.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_get_group(
        &self,
        user_id: &str,
        team_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphGroupAllOf> {
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
                "/users/{}/joinedTeams/{}/group?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List incomingChannels.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/incomingChannels` endpoint.
    *
    * Get the list of incoming channels (channels shared with a team).
    *
    * FROM: <https://docs.microsoft.com/graph/api/team-list-incomingchannels?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_list_incoming_channel(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/incomingChannels?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get incomingChannels from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/incomingChannels/{channel-id}` endpoint.
    *
    * List of channels shared with the team.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_get_incoming_channel(
        &self,
        user_id: &str,
        team_id: &str,
        channel_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphChannelAllOf> {
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
                "/users/{}/joinedTeams/{}/incomingChannels/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&channel_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/incomingChannels/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_incoming_channels_get_count_bf_05(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/incomingChannels/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List apps in team.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/installedApps` endpoint.
    *
    * Retrieve a list of apps installed in the specified team.
    *
    * FROM: <https://docs.microsoft.com/graph/api/team-list-installedapps?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_list_installed_app(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/installedApps?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Add app to team.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/installedApps` endpoint.
    *
    * Install an app to the specified team.
    *
    * FROM: <https://docs.microsoft.com/graph/api/team-post-installedapps?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_create_installed_apps(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphTeamsAppInstallationAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsAppInstallationAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/installedApps",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/installedApps/{teamsAppInstallation-id}` endpoint.
    *
    * The apps installed in this team.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_get_installed_app(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/installedApps/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/installedApps/{teamsAppInstallation-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_delete_installed_apps(
        &self,
        user_id: &str,
        team_id: &str,
        teams_app_installation_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/installedApps/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/installedApps/{teamsAppInstallation-id}` endpoint.
    */
    pub async fn users_joined_teams_update_installed_apps(
        &self,
        user_id: &str,
        team_id: &str,
        teams_app_installation_id: &str,
        body: &crate::types::MicrosoftGraphTeamsAppInstallationAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsAppInstallationAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/installedApps/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/installedApps/{teamsAppInstallation-id}/teamsApp` endpoint.
    *
    * The app that is installed.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_installed_apps_get_app(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/installedApps/{}/teamsApp?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/installedApps/{teamsAppInstallation-id}/teamsAppDefinition` endpoint.
    *
    * The details of this version of the app.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_installed_apps_get_app_definition(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/installedApps/{}/teamsAppDefinition?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/installedApps/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_installed_apps_get_count_1148(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/installedApps/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List members of team.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/members` endpoint.
    *
    * Get the conversationMember collection of a team.
    *
    * FROM: <https://docs.microsoft.com/graph/api/team-list-members?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_list_member(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/members?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Add member to team.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/members` endpoint.
    *
    * Add a new conversationMember to a team.
    *
    * FROM: <https://docs.microsoft.com/graph/api/team-post-members?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_create_members(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphConversationMemberAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphConversationMemberAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/members",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/members/{conversationMember-id}` endpoint.
    *
    * Members and owners of the team.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_get_member(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/members/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/members/{conversationMember-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_delete_members(
        &self,
        user_id: &str,
        team_id: &str,
        conversation_member_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/members/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/members/{conversationMember-id}` endpoint.
    */
    pub async fn users_joined_teams_update_members(
        &self,
        user_id: &str,
        team_id: &str,
        conversation_member_id: &str,
        body: &crate::types::MicrosoftGraphConversationMemberAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphConversationMemberAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/members/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/members/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_members_get_count_5648(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/members/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get operations from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/operations` endpoint.
    *
    * The async operations that ran or are running on this team.
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
    pub async fn users_joined_teams_list_operation(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/operations?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Create new navigation property to operations for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/operations` endpoint.
    */
    pub async fn users_joined_teams_create_operations(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphTeamsAsyncOperationAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsAsyncOperationAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/operations",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get operations from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/operations/{teamsAsyncOperation-id}` endpoint.
    *
    * The async operations that ran or are running on this team.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_get_operation(
        &self,
        user_id: &str,
        team_id: &str,
        teams_async_operation_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsAsyncOperationAllOf> {
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
                "/users/{}/joinedTeams/{}/operations/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&teams_async_operation_id.to_string()),
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
    * Delete navigation property operations for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/operations/{teamsAsyncOperation-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_delete_operations(
        &self,
        user_id: &str,
        team_id: &str,
        teams_async_operation_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/operations/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&teams_async_operation_id.to_string()),
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
    * Update the navigation property operations in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/operations/{teamsAsyncOperation-id}` endpoint.
    */
    pub async fn users_joined_teams_update_operations(
        &self,
        user_id: &str,
        team_id: &str,
        teams_async_operation_id: &str,
        body: &crate::types::MicrosoftGraphTeamsAsyncOperationAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsAsyncOperationAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/operations/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&teams_async_operation_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/operations/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_operations_get_count_5268(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/operations/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get photo from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/photo` endpoint.
    *
    * The profile photo for the team.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_get_photo(
        &self,
        user_id: &str,
        team_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphProfilePhotoAllOf> {
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
                "/users/{}/joinedTeams/{}/photo?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/photo` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_delete_photo(
        &self,
        user_id: &str,
        team_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/photo",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/photo` endpoint.
    */
    pub async fn users_joined_teams_update_photo(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphProfilePhotoAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphProfilePhotoAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/photo",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/photo/$value` endpoint.
    */
    pub async fn users_joined_teams_get_photo_content(
        &self,
        user_id: &str,
        team_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/photo/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `PUT` to the `/users/{user-id}/joinedTeams/{team-id}/photo/$value` endpoint.
    */
    pub async fn users_joined_teams_update_photo_content<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        team_id: &str,
        body: B,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/photo/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get primaryChannel.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel` endpoint.
    *
    * Get the default channel, **General**, of a team.
    *
    * FROM: <https://docs.microsoft.com/graph/api/team-get-primarychannel?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_get_primary_channel(
        &self,
        user_id: &str,
        team_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphChannelAllOf> {
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
                "/users/{}/joinedTeams/{}/primaryChannel?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Delete navigation property primaryChannel for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_delete_primary_channel(
        &self,
        user_id: &str,
        team_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Update the navigation property primaryChannel in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel` endpoint.
    */
    pub async fn users_joined_teams_update_primary_channel(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphChannelAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChannelAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get filesFolder.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/filesFolder` endpoint.
    *
    * Get the metadata for the location where the files of a channel are stored.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-get-filesfolder?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_primary_channel_get_files_folder(
        &self,
        user_id: &str,
        team_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphDriveItemAllOf> {
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
                "/users/{}/joinedTeams/{}/primaryChannel/filesFolder?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get content for the navigation property filesFolder from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/filesFolder/content` endpoint.
    *
    * The content stream, if the item represents a file.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-get-filesfolder?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_primary_channel_get_files_folder_content(
        &self,
        user_id: &str,
        team_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/filesFolder/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Update content for the navigation property filesFolder in users.
    *
    * This function performs a `PUT` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/filesFolder/content` endpoint.
    *
    * The content stream, if the item represents a file.
    */
    pub async fn users_joined_teams_primary_channel_update_files_folder_content<
        B: Into<reqwest::Body>,
    >(
        &self,
        user_id: &str,
        team_id: &str,
        body: B,
    ) -> ClientResult<crate::types::MicrosoftGraphDriveItemAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/filesFolder/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List members of a channel.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/members` endpoint.
    *
    * Retrieve a list of conversationMembers from a channel. This method supports federation. Only a user who is a member of the shared channel can retrieve the channel member list.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-list-members?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_primary_channel_list_member(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/members?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Add member to channel.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/members` endpoint.
    *
    * Add a conversationMember to a channel. This operation is allowed only for channels with a **membershipType** value of `private` or `shared`.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-post-members?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_primary_channel_create_members(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphConversationMemberAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphConversationMemberAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/members",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/members/{conversationMember-id}` endpoint.
    *
    * A collection of membership records associated with the channel.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_primary_channel_get_member(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/members/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/members/{conversationMember-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_primary_channel_delete_members(
        &self,
        user_id: &str,
        team_id: &str,
        conversation_member_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/members/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/members/{conversationMember-id}` endpoint.
    */
    pub async fn users_joined_teams_primary_channel_update_members(
        &self,
        user_id: &str,
        team_id: &str,
        conversation_member_id: &str,
        body: &crate::types::MicrosoftGraphConversationMemberAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphConversationMemberAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/members/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/members/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_primary_channel_members_get_count_71_9e(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/members/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List channel messages.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages` endpoint.
    *
    * Retrieve the list of messages (without the replies) in a channel of a team.  To get the replies for a message, call the list message replies or the get message reply API.  This method supports federation. To list channel messages in application context, the request must be made from the tenant that the channel owner belongs to (represented by the **tenantId** property on the channel).
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-list-messages?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_primary_channel_list_message(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/messages?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Send chatMessage in channel.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages` endpoint.
    *
    * Send a new chatMessage in the specified channel.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-post-messages?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_primary_channel_create_messages(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}` endpoint.
    *
    * A collection of all the messages in the channel. A navigation property. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_primary_channel_get_message(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_primary_channel_delete_messages(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}` endpoint.
    */
    pub async fn users_joined_teams_primary_channel_update_messages(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/hostedContents` endpoint.
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
    pub async fn users_joined_teams_primary_channel_messages_list_hosted_content(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/hostedContents?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/hostedContents` endpoint.
    */
    pub async fn users_joined_teams_primary_channel_messages_create_hosted_contents(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageHostedContentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageHostedContentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/hostedContents",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    *
    * Content in a message hosted by Microsoft Teams - for example, images or code snippets.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_primary_channel_messages_get_hosted_content(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/hostedContents/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_primary_channel_messages_delete_hosted_contents(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
        chat_message_hosted_content_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/hostedContents/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    */
    pub async fn users_joined_teams_primary_channel_messages_update_hosted_contents(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
        chat_message_hosted_content_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageHostedContentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageHostedContentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/hostedContents/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/hostedContents/{chatMessageHostedContent-id}/$value` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chatmessage-list-hostedcontents?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_primary_channel_messages_get_hosted_contents_content(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
        chat_message_hosted_content_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/hostedContents/{}/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `PUT` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/hostedContents/{chatMessageHostedContent-id}/$value` endpoint.
    */
    pub async fn users_joined_teams_primary_channel_messages_update_hosted_contents_content<
        B: Into<reqwest::Body>,
    >(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
        chat_message_hosted_content_id: &str,
        body: B,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/hostedContents/{}/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/hostedContents/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_primary_channel_messages_hosted_contents_get_count_9_8f_7(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/hostedContents/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/replies` endpoint.
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
    pub async fn users_joined_teams_primary_channel_messages_list_replie(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/replies?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/replies` endpoint.
    *
    * Create a new reply to a chatMessage in a specified channel.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-post-messagereply?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_primary_channel_messages_create_replies(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/replies",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/replies/{chatMessage-id1}` endpoint.
    *
    * Replies for a specified message. Supports $expand for channel messages.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_primary_channel_messages_get_replie(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/replies/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/replies/{chatMessage-id1}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_primary_channel_messages_delete_replies(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/replies/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/replies/{chatMessage-id1}` endpoint.
    */
    pub async fn users_joined_teams_primary_channel_messages_update_replies(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        body: &crate::types::MicrosoftGraphChatMessageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/replies/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents` endpoint.
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
    pub async fn users_joined_teams_primary_channel_messages_replies_list_hosted_content(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/replies/{}/hostedContents?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents` endpoint.
    */
    pub async fn users_joined_teams_primary_channel_messages_replies_create_hosted_contents(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        body: &crate::types::MicrosoftGraphChatMessageHostedContentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageHostedContentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/replies/{}/hostedContents",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    *
    * Content in a message hosted by Microsoft Teams - for example, images or code snippets.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_primary_channel_messages_replies_get_hosted_content(
        &self,
        user_id: &str,
        team_id: &str,
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
&format!("/users/{}/joinedTeams/{}/primaryChannel/messages/{}/replies/{}/hostedContents/{}?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&team_id.to_string()),crate::progenitor_support::encode_path(&chat_message_id.to_string()),crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),crate::progenitor_support::encode_path(&chat_message_hosted_content_id.to_string()),query_), None);
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
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_primary_channel_messages_replies_delete_hosted_contents(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        chat_message_hosted_content_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/replies/{}/hostedContents/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/{chatMessageHostedContent-id}` endpoint.
    */
    pub async fn users_joined_teams_primary_channel_messages_replies_update_hosted_contents(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        chat_message_hosted_content_id: &str,
        body: &crate::types::MicrosoftGraphChatMessageHostedContentAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChatMessageHostedContentAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/replies/{}/hostedContents/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/{chatMessageHostedContent-id}/$value` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/chatmessage-list-hostedcontents?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_primary_channel_messages_replies_get_hosted_contents_content(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        chat_message_hosted_content_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/joinedTeams/{}/primaryChannel/messages/{}/replies/{}/hostedContents/{}/$value",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&team_id.to_string()),crate::progenitor_support::encode_path(&chat_message_id.to_string()),crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),crate::progenitor_support::encode_path(&chat_message_hosted_content_id.to_string()),), None);
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
    * This function performs a `PUT` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/{chatMessageHostedContent-id}/$value` endpoint.
    */
    pub async fn users_joined_teams_primary_channel_messages_replies_update_hosted_contents_content<
        B: Into<reqwest::Body>,
    >(
        &self,
        user_id: &str,
        team_id: &str,
        chat_message_id: &str,
        chat_message_id_1: &str,
        chat_message_hosted_content_id: &str,
        body: B,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/joinedTeams/{}/primaryChannel/messages/{}/replies/{}/hostedContents/{}/$value",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&team_id.to_string()),crate::progenitor_support::encode_path(&chat_message_id.to_string()),crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),crate::progenitor_support::encode_path(&chat_message_hosted_content_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/replies/{chatMessage-id1}/hostedContents/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_primary_channel_messages_replies_hosted_contents_get_count_f_576(
        &self,
        user_id: &str,
        team_id: &str,
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
&format!("/users/{}/joinedTeams/{}/primaryChannel/messages/{}/replies/{}/hostedContents/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&team_id.to_string()),crate::progenitor_support::encode_path(&chat_message_id.to_string()),crate::progenitor_support::encode_path(&chat_message_id_1.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/{chatMessage-id}/replies/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_primary_channel_messages_replies_get_count_5af_6(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/messages/{}/replies/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/messages/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_primary_channel_messages_get_count_0_8f_5(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/messages/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List sharedWithChannelTeamInfo.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/sharedWithTeams` endpoint.
    *
    * Get the list of teams that has been shared a specified channel. This operation is allowed only for channels with a **membershipType** value of `shared`.
    *
    * FROM: <https://docs.microsoft.com/graph/api/sharedwithchannelteaminfo-list?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_primary_channel_list_shared(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/sharedWithTeams?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Create new navigation property to sharedWithTeams for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/sharedWithTeams` endpoint.
    */
    pub async fn users_joined_teams_primary_channel_create_shared(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphSharedWithChannelTeamInfoAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSharedWithChannelTeamInfoAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/sharedWithTeams",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get sharedWithTeams from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/sharedWithTeams/{sharedWithChannelTeamInfo-id}` endpoint.
    *
    * A collection of teams with which a channel is shared.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_primary_channel_get_shared(
        &self,
        user_id: &str,
        team_id: &str,
        shared_with_channel_team_info_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSharedWithChannelTeamInfoAllOf> {
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
                "/users/{}/joinedTeams/{}/primaryChannel/sharedWithTeams/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(
                    &shared_with_channel_team_info_id.to_string()
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
    * Delete navigation property sharedWithTeams for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/sharedWithTeams/{sharedWithChannelTeamInfo-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_primary_channel_delete_shared(
        &self,
        user_id: &str,
        team_id: &str,
        shared_with_channel_team_info_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/sharedWithTeams/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(
                    &shared_with_channel_team_info_id.to_string()
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
    * Update the navigation property sharedWithTeams in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/sharedWithTeams/{sharedWithChannelTeamInfo-id}` endpoint.
    */
    pub async fn users_joined_teams_primary_channel_update_shared(
        &self,
        user_id: &str,
        team_id: &str,
        shared_with_channel_team_info_id: &str,
        body: &crate::types::MicrosoftGraphSharedWithChannelTeamInfoAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSharedWithChannelTeamInfoAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/sharedWithTeams/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(
                    &shared_with_channel_team_info_id.to_string()
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
    * List allowedMembers.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/sharedWithTeams/{sharedWithChannelTeamInfo-id}/allowedMembers` endpoint.
    *
    * Get the list of conversationMembers who can access a shared channel. This method does not return the following conversationMembers from the team:
    * - Users with `Guest` role
    * - Users who are externally authenticated in the tenant
    *
    * FROM: <https://docs.microsoft.com/graph/api/sharedwithchannelteaminfo-list-allowedmembers?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_primary_channel_shared_list_allowed_member(
        &self,
        user_id: &str,
        team_id: &str,
        shared_with_channel_team_info_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/sharedWithTeams/{}/allowedMembers?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(
                    &shared_with_channel_team_info_id.to_string()
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
    * Get allowedMembers from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/sharedWithTeams/{sharedWithChannelTeamInfo-id}/allowedMembers/{conversationMember-id}` endpoint.
    *
    * A collection of team members who have access to the shared channel.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_primary_channel_shared_get_allowed_member(
        &self,
        user_id: &str,
        team_id: &str,
        shared_with_channel_team_info_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/sharedWithTeams/{}/allowedMembers/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(
                    &shared_with_channel_team_info_id.to_string()
                ),
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
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/sharedWithTeams/{sharedWithChannelTeamInfo-id}/allowedMembers/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_primary_channel_shared_allowed_members_get_count_3d_16(
        &self,
        user_id: &str,
        team_id: &str,
        shared_with_channel_team_info_id: &str,
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
&format!("/users/{}/joinedTeams/{}/primaryChannel/sharedWithTeams/{}/allowedMembers/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&team_id.to_string()),crate::progenitor_support::encode_path(&shared_with_channel_team_info_id.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/sharedWithTeams/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_primary_channel_shared_get_count_d_3ab(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/sharedWithTeams/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List tabs in channel.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/tabs` endpoint.
    *
    * Retrieve the list of tabs in the specified channel within a team.
    *
    * FROM: <https://docs.microsoft.com/graph/api/channel-list-tabs?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_primary_channel_list_tab(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/tabs?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Create new navigation property to tabs for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/tabs` endpoint.
    */
    pub async fn users_joined_teams_primary_channel_create_tabs(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphTeamsTabAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsTabAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/tabs",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/tabs/{teamsTab-id}` endpoint.
    *
    * A collection of all the tabs in the channel. A navigation property.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_primary_channel_get_tab(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/tabs/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/tabs/{teamsTab-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_primary_channel_delete_tabs(
        &self,
        user_id: &str,
        team_id: &str,
        teams_tab_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/tabs/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/tabs/{teamsTab-id}` endpoint.
    */
    pub async fn users_joined_teams_primary_channel_update_tabs(
        &self,
        user_id: &str,
        team_id: &str,
        teams_tab_id: &str,
        body: &crate::types::MicrosoftGraphTeamsTabAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamsTabAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/primaryChannel/tabs/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/tabs/{teamsTab-id}/teamsApp` endpoint.
    *
    * The application that is linked to the tab. This cannot be changed after tab creation.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_primary_channel_tabs_get_app(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/tabs/{}/teamsApp?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/primaryChannel/tabs/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_primary_channel_tabs_get_count_8_6f_9(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/primaryChannel/tabs/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get schedule.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule` endpoint.
    *
    * Retrieve the properties and relationships of a schedule object. The schedule creation process conforms to the One API guideline for resource based long running operations (RELO).
    * When clients use the PUT method, if the schedule is provisioned, the operation updates the schedule; otherwise, the operation starts the schedule provisioning process in the background. During schedule provisioning, clients can use the GET method to get the schedule and look at the `provisionStatus` property for the current state of the provisioning. If the provisioning failed, clients can get additional information from the `provisionStatusCode` property. Clients can also inspect the configuration of the schedule.
    *
    * FROM: <https://docs.microsoft.com/graph/api/schedule-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_get_schedule(
        &self,
        user_id: &str,
        team_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphScheduleAllOf> {
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
                "/users/{}/joinedTeams/{}/schedule?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Update the navigation property schedule in users.
    *
    * This function performs a `PUT` to the `/users/{user-id}/joinedTeams/{team-id}/schedule` endpoint.
    */
    pub async fn users_joined_teams_update_schedule(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphScheduleAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphScheduleAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Delete navigation property schedule for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/schedule` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_delete_schedule(
        &self,
        user_id: &str,
        team_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List offerShiftRequest.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/offerShiftRequests` endpoint.
    *
    * Retrieve the properties and relationships of all offerShiftRequest objects in a team.
    *
    * FROM: <https://docs.microsoft.com/graph/api/offershiftrequest-list?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_schedule_list_offer_shift_request(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/schedule/offerShiftRequests?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Create new navigation property to offerShiftRequests for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/offerShiftRequests` endpoint.
    */
    pub async fn users_joined_teams_schedule_create_offer_shift_requests(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphOfferShiftRequestAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOfferShiftRequestAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/offerShiftRequests",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get offerShiftRequests from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/offerShiftRequests/{offerShiftRequest-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_schedule_get_offer_shift_request(
        &self,
        user_id: &str,
        team_id: &str,
        offer_shift_request_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOfferShiftRequestAllOf> {
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
                "/users/{}/joinedTeams/{}/schedule/offerShiftRequests/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&offer_shift_request_id.to_string()),
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
    * Delete navigation property offerShiftRequests for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/offerShiftRequests/{offerShiftRequest-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_delete_offer_shift_requests(
        &self,
        user_id: &str,
        team_id: &str,
        offer_shift_request_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/offerShiftRequests/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&offer_shift_request_id.to_string()),
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
    * Update the navigation property offerShiftRequests in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/offerShiftRequests/{offerShiftRequest-id}` endpoint.
    */
    pub async fn users_joined_teams_schedule_update_offer_shift_requests(
        &self,
        user_id: &str,
        team_id: &str,
        offer_shift_request_id: &str,
        body: &crate::types::MicrosoftGraphOfferShiftRequestAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOfferShiftRequestAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/offerShiftRequests/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&offer_shift_request_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/offerShiftRequests/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_offer_shift_requests_get_count_b_933(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/schedule/offerShiftRequests/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List openShiftChangeRequests.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/openShiftChangeRequests` endpoint.
    *
    * Retrieve a list of openShiftChangeRequest objects in a team.
    *
    * FROM: <https://docs.microsoft.com/graph/api/openshiftchangerequest-list?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_schedule_list_open_shift_change_request(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/schedule/openShiftChangeRequests?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Create openShiftChangeRequest.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/openShiftChangeRequests` endpoint.
    *
    * Create instance of an openShiftChangeRequest object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/openshiftchangerequest-post?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_schedule_create_open_shift_change_requests(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphOpenShiftChangeRequestAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOpenShiftChangeRequestAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/openShiftChangeRequests",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get openShiftChangeRequests from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/openShiftChangeRequests/{openShiftChangeRequest-id}` endpoint.
    *
    * The open shift requests in the schedule.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_schedule_get_open_shift_change_request(
        &self,
        user_id: &str,
        team_id: &str,
        open_shift_change_request_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOpenShiftChangeRequestAllOf> {
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
                "/users/{}/joinedTeams/{}/schedule/openShiftChangeRequests/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&open_shift_change_request_id.to_string()),
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
    * Delete navigation property openShiftChangeRequests for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/openShiftChangeRequests/{openShiftChangeRequest-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_delete_open_shift_change_requests(
        &self,
        user_id: &str,
        team_id: &str,
        open_shift_change_request_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/openShiftChangeRequests/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&open_shift_change_request_id.to_string()),
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
    * Update the navigation property openShiftChangeRequests in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/openShiftChangeRequests/{openShiftChangeRequest-id}` endpoint.
    */
    pub async fn users_joined_teams_schedule_update_open_shift_change_requests(
        &self,
        user_id: &str,
        team_id: &str,
        open_shift_change_request_id: &str,
        body: &crate::types::MicrosoftGraphOpenShiftChangeRequestAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOpenShiftChangeRequestAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/openShiftChangeRequests/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&open_shift_change_request_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/openShiftChangeRequests/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_open_shift_change_requests_get_count_27_0a(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/schedule/openShiftChangeRequests/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List openShifts.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/openShifts` endpoint.
    *
    * List openShift objects in a team.
    *
    * FROM: <https://docs.microsoft.com/graph/api/openshift-list?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_schedule_list_open_shift(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/schedule/openShifts?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Create openShift.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/openShifts` endpoint.
    *
    * Create an instance of an openShift object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/openshift-post?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_schedule_create_open_shifts(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphOpenShiftAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOpenShiftAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/openShifts",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get openShifts from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/openShifts/{openShift-id}` endpoint.
    *
    * The set of open shifts in a scheduling group in the schedule.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_schedule_get_open_shift(
        &self,
        user_id: &str,
        team_id: &str,
        open_shift_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOpenShiftAllOf> {
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
                "/users/{}/joinedTeams/{}/schedule/openShifts/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&open_shift_id.to_string()),
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
    * Delete navigation property openShifts for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/openShifts/{openShift-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_delete_open_shifts(
        &self,
        user_id: &str,
        team_id: &str,
        open_shift_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/openShifts/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&open_shift_id.to_string()),
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
    * Update the navigation property openShifts in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/openShifts/{openShift-id}` endpoint.
    */
    pub async fn users_joined_teams_schedule_update_open_shifts(
        &self,
        user_id: &str,
        team_id: &str,
        open_shift_id: &str,
        body: &crate::types::MicrosoftGraphOpenShiftAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOpenShiftAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/openShifts/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&open_shift_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/openShifts/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_open_shifts_get_count_78_5e(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/schedule/openShifts/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List scheduleGroups.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/schedulingGroups` endpoint.
    *
    * Get the list of schedulingGroups in this schedule.
    *
    * FROM: <https://docs.microsoft.com/graph/api/schedule-list-schedulinggroups?view=graph-rest-1.0>
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
    */
    pub async fn users_joined_teams_schedule_list_scheduling_group(
        &self,
        user_id: &str,
        team_id: &str,
        top: u64,
        skip: u64,
        search: &str,
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
                "/users/{}/joinedTeams/{}/schedule/schedulingGroups?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Create schedulingGroup.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/schedulingGroups` endpoint.
    *
    * Create a new schedulingGroup.
    *
    * FROM: <https://docs.microsoft.com/graph/api/schedule-post-schedulinggroups?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_schedule_create_scheduling_groups(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphSchedulingGroupAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSchedulingGroupAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/schedulingGroups",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get schedulingGroups from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/schedulingGroups/{schedulingGroup-id}` endpoint.
    *
    * The logical grouping of users in the schedule (usually by role).
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn users_joined_teams_schedule_get_scheduling_group(
        &self,
        user_id: &str,
        team_id: &str,
        scheduling_group_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSchedulingGroupAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/schedulingGroups/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&scheduling_group_id.to_string()),
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
    * Delete navigation property schedulingGroups for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/schedulingGroups/{schedulingGroup-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_delete_scheduling_groups(
        &self,
        user_id: &str,
        team_id: &str,
        scheduling_group_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/schedulingGroups/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&scheduling_group_id.to_string()),
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
    * Update the navigation property schedulingGroups in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/schedulingGroups/{schedulingGroup-id}` endpoint.
    */
    pub async fn users_joined_teams_schedule_update_scheduling_groups(
        &self,
        user_id: &str,
        team_id: &str,
        scheduling_group_id: &str,
        body: &crate::types::MicrosoftGraphSchedulingGroupAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSchedulingGroupAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/schedulingGroups/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&scheduling_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/schedulingGroups/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_scheduling_groups_get_count_b_253(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/schedule/schedulingGroups/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List shifts.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/shifts` endpoint.
    *
    * Get the list of shift instances in a schedule.
    *
    * FROM: <https://docs.microsoft.com/graph/api/schedule-list-shifts?view=graph-rest-1.0>
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
    */
    pub async fn users_joined_teams_schedule_list_shift(
        &self,
        user_id: &str,
        team_id: &str,
        top: u64,
        skip: u64,
        search: &str,
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
                "/users/{}/joinedTeams/{}/schedule/shifts?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Create shift.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/shifts` endpoint.
    *
    * Create a new shift instance in a schedule. The duration of a shift cannot be less than 1 minute or longer than 24 hours.
    *
    * FROM: <https://docs.microsoft.com/graph/api/schedule-post-shifts?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_schedule_create_shifts(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphShiftAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphShiftAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/shifts",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get shifts from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/shifts/{shift-id}` endpoint.
    *
    * The shifts in the schedule.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn users_joined_teams_schedule_get_shift(
        &self,
        user_id: &str,
        team_id: &str,
        shift_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphShiftAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/shifts/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&shift_id.to_string()),
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
    * Delete navigation property shifts for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/shifts/{shift-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_delete_shifts(
        &self,
        user_id: &str,
        team_id: &str,
        shift_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/shifts/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&shift_id.to_string()),
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
    * Update the navigation property shifts in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/shifts/{shift-id}` endpoint.
    */
    pub async fn users_joined_teams_schedule_update_shifts(
        &self,
        user_id: &str,
        team_id: &str,
        shift_id: &str,
        body: &crate::types::MicrosoftGraphShiftAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphShiftAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/shifts/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&shift_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/shifts/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_shifts_get_count_6a_1a(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/schedule/shifts/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List swapShiftsChangeRequest.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/swapShiftsChangeRequests` endpoint.
    *
    * Retrieve a list of swapShiftsChangeRequest objects in the team.
    *
    * FROM: <https://docs.microsoft.com/graph/api/swapshiftschangerequest-list?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_schedule_list_swap_shifts_change_request(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/schedule/swapShiftsChangeRequests?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Create swapShiftsChangeRequest.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/swapShiftsChangeRequests` endpoint.
    *
    * Create an instance of a swapShiftsChangeRequest object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/swapshiftschangerequest-post?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_schedule_create_swap_shifts_change_requests(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphSwapShiftsChangeRequestAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSwapShiftsChangeRequestAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/swapShiftsChangeRequests",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get swapShiftsChangeRequests from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/swapShiftsChangeRequests/{swapShiftsChangeRequest-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_schedule_get_swap_shifts_change_request(
        &self,
        user_id: &str,
        team_id: &str,
        swap_shifts_change_request_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSwapShiftsChangeRequestAllOf> {
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
                "/users/{}/joinedTeams/{}/schedule/swapShiftsChangeRequests/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&swap_shifts_change_request_id.to_string()),
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
    * Delete navigation property swapShiftsChangeRequests for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/swapShiftsChangeRequests/{swapShiftsChangeRequest-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_delete_swap_shifts_change_requests(
        &self,
        user_id: &str,
        team_id: &str,
        swap_shifts_change_request_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/swapShiftsChangeRequests/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&swap_shifts_change_request_id.to_string()),
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
    * Update the navigation property swapShiftsChangeRequests in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/swapShiftsChangeRequests/{swapShiftsChangeRequest-id}` endpoint.
    */
    pub async fn users_joined_teams_schedule_update_swap_shifts_change_requests(
        &self,
        user_id: &str,
        team_id: &str,
        swap_shifts_change_request_id: &str,
        body: &crate::types::MicrosoftGraphSwapShiftsChangeRequestAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSwapShiftsChangeRequestAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/swapShiftsChangeRequests/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&swap_shifts_change_request_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/swapShiftsChangeRequests/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_swap_shifts_change_requests_get_count_b_5f_7(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/schedule/swapShiftsChangeRequests/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List timeOffReasons.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timeOffReasons` endpoint.
    *
    * Get the list of timeOffReasons in a schedule.
    *
    * FROM: <https://docs.microsoft.com/graph/api/schedule-list-timeoffreasons?view=graph-rest-1.0>
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
    */
    pub async fn users_joined_teams_schedule_list_time_off_reason(
        &self,
        user_id: &str,
        team_id: &str,
        top: u64,
        skip: u64,
        search: &str,
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
                "/users/{}/joinedTeams/{}/schedule/timeOffReasons?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Create timeOffReason.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timeOffReasons` endpoint.
    *
    * Create a new timeOffReason.
    *
    * FROM: <https://docs.microsoft.com/graph/api/schedule-post-timeoffreasons?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_schedule_create_time_off_reasons(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphTimeOffReasonAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTimeOffReasonAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/timeOffReasons",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get timeOffReasons from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timeOffReasons/{timeOffReason-id}` endpoint.
    *
    * The set of reasons for a time off in the schedule.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn users_joined_teams_schedule_get_time_off_reason(
        &self,
        user_id: &str,
        team_id: &str,
        time_off_reason_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphTimeOffReasonAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/timeOffReasons/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&time_off_reason_id.to_string()),
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
    * Delete navigation property timeOffReasons for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timeOffReasons/{timeOffReason-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_delete_time_off_reasons(
        &self,
        user_id: &str,
        team_id: &str,
        time_off_reason_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/timeOffReasons/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&time_off_reason_id.to_string()),
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
    * Update the navigation property timeOffReasons in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timeOffReasons/{timeOffReason-id}` endpoint.
    */
    pub async fn users_joined_teams_schedule_update_time_off_reasons(
        &self,
        user_id: &str,
        team_id: &str,
        time_off_reason_id: &str,
        body: &crate::types::MicrosoftGraphTimeOffReasonAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTimeOffReasonAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/timeOffReasons/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&time_off_reason_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timeOffReasons/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_time_off_reasons_get_count_9789(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/schedule/timeOffReasons/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List timeOffRequest.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timeOffRequests` endpoint.
    *
    * Retrieve a list of timeOffRequest objects in the team.
    *
    * FROM: <https://docs.microsoft.com/graph/api/timeoffrequest-list?view=graph-rest-1.0>
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
    */
    pub async fn users_joined_teams_schedule_list_time_off_request(
        &self,
        user_id: &str,
        team_id: &str,
        top: u64,
        skip: u64,
        search: &str,
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
                "/users/{}/joinedTeams/{}/schedule/timeOffRequests?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Create new navigation property to timeOffRequests for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timeOffRequests` endpoint.
    */
    pub async fn users_joined_teams_schedule_create_time_off_requests(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphTimeOffRequestAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTimeOffRequestAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/timeOffRequests",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get timeOffRequests from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timeOffRequests/{timeOffRequest-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn users_joined_teams_schedule_get_time_off_request(
        &self,
        user_id: &str,
        team_id: &str,
        time_off_request_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphTimeOffRequestAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/timeOffRequests/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&time_off_request_id.to_string()),
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
    * Delete navigation property timeOffRequests for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timeOffRequests/{timeOffRequest-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_delete_time_off_requests(
        &self,
        user_id: &str,
        team_id: &str,
        time_off_request_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/timeOffRequests/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&time_off_request_id.to_string()),
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
    * Update the navigation property timeOffRequests in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timeOffRequests/{timeOffRequest-id}` endpoint.
    */
    pub async fn users_joined_teams_schedule_update_time_off_requests(
        &self,
        user_id: &str,
        team_id: &str,
        time_off_request_id: &str,
        body: &crate::types::MicrosoftGraphTimeOffRequestAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTimeOffRequestAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/timeOffRequests/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&time_off_request_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timeOffRequests/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_time_off_requests_get_count_c_179(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/schedule/timeOffRequests/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List timesOff.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timesOff` endpoint.
    *
    * Get the list of timeOff instances in a schedule.
    *
    * FROM: <https://docs.microsoft.com/graph/api/schedule-list-timesoff?view=graph-rest-1.0>
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
    */
    pub async fn users_joined_teams_schedule_list_times_off(
        &self,
        user_id: &str,
        team_id: &str,
        top: u64,
        skip: u64,
        search: &str,
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
                "/users/{}/joinedTeams/{}/schedule/timesOff?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Create timeOff.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timesOff` endpoint.
    *
    * Create a new timeOff instance in a schedule.
    *
    * FROM: <https://docs.microsoft.com/graph/api/schedule-post-timesoff?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_schedule_create_times_off(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphTimeOffAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTimeOffAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/timesOff",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get timesOff from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timesOff/{timeOff-id}` endpoint.
    *
    * The instances of times off in the schedule.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn users_joined_teams_schedule_get_times_off(
        &self,
        user_id: &str,
        team_id: &str,
        time_off_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphTimeOffAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/timesOff/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&time_off_id.to_string()),
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
    * Delete navigation property timesOff for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timesOff/{timeOff-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_delete_times_off(
        &self,
        user_id: &str,
        team_id: &str,
        time_off_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/timesOff/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&time_off_id.to_string()),
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
    * Update the navigation property timesOff in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timesOff/{timeOff-id}` endpoint.
    */
    pub async fn users_joined_teams_schedule_update_times_off(
        &self,
        user_id: &str,
        team_id: &str,
        time_off_id: &str,
        body: &crate::types::MicrosoftGraphTimeOffAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTimeOffAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/schedule/timesOff/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&time_off_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/schedule/timesOff/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_schedule_times_off_get_count_1ead(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/schedule/timesOff/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * List teamworkTags.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/tags` endpoint.
    *
    * Get a list of the tag objects and their properties.
    *
    * FROM: <https://docs.microsoft.com/graph/api/teamworktag-list?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_list_tag(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/tags?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Create teamworkTag.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/tags` endpoint.
    *
    * Create a standard tag for members in a team.
    *
    * FROM: <https://docs.microsoft.com/graph/api/teamworktag-post?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_create_tags(
        &self,
        user_id: &str,
        team_id: &str,
        body: &crate::types::MicrosoftGraphTeamworkTagAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamworkTagAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/tags",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get tags from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/tags/{teamworkTag-id}` endpoint.
    *
    * The tags associated with the team.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_get_tag(
        &self,
        user_id: &str,
        team_id: &str,
        teamwork_tag_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphTeamworkTagAllOf> {
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
                "/users/{}/joinedTeams/{}/tags/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&teamwork_tag_id.to_string()),
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
    * Delete navigation property tags for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/tags/{teamworkTag-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_delete_tags(
        &self,
        user_id: &str,
        team_id: &str,
        teamwork_tag_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/tags/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&teamwork_tag_id.to_string()),
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
    * Update the navigation property tags in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/tags/{teamworkTag-id}` endpoint.
    */
    pub async fn users_joined_teams_update_tags(
        &self,
        user_id: &str,
        team_id: &str,
        teamwork_tag_id: &str,
        body: &crate::types::MicrosoftGraphTeamworkTagAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamworkTagAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/tags/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&teamwork_tag_id.to_string()),
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
    * List members in a teamworkTag.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/tags/{teamworkTag-id}/members` endpoint.
    *
    * Get a list of the members of a standard tag in a team and their properties.
    *
    * FROM: <https://docs.microsoft.com/graph/api/teamworktagmember-list?view=graph-rest-1.0>
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
    pub async fn users_joined_teams_tags_list_member(
        &self,
        user_id: &str,
        team_id: &str,
        teamwork_tag_id: &str,
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
                "/users/{}/joinedTeams/{}/tags/{}/members?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&teamwork_tag_id.to_string()),
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
    * Create teamworkTagMember.
    *
    * This function performs a `POST` to the `/users/{user-id}/joinedTeams/{team-id}/tags/{teamworkTag-id}/members` endpoint.
    *
    * Create a new teamworkTagMember object in a team.
    *
    * FROM: <https://docs.microsoft.com/graph/api/teamworktagmember-post?view=graph-rest-1.0>
    */
    pub async fn users_joined_teams_tags_create_members(
        &self,
        user_id: &str,
        team_id: &str,
        teamwork_tag_id: &str,
        body: &crate::types::MicrosoftGraphTeamworkTagMemberAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamworkTagMemberAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/tags/{}/members",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&teamwork_tag_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/tags/{teamworkTag-id}/members/{teamworkTagMember-id}` endpoint.
    *
    * Users assigned to the tag.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_tags_get_member(
        &self,
        user_id: &str,
        team_id: &str,
        teamwork_tag_id: &str,
        teamwork_tag_member_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphTeamworkTagMemberAllOf> {
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
                "/users/{}/joinedTeams/{}/tags/{}/members/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&teamwork_tag_id.to_string()),
                crate::progenitor_support::encode_path(&teamwork_tag_member_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/joinedTeams/{team-id}/tags/{teamworkTag-id}/members/{teamworkTagMember-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_tags_delete_members(
        &self,
        user_id: &str,
        team_id: &str,
        teamwork_tag_id: &str,
        teamwork_tag_member_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/tags/{}/members/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&teamwork_tag_id.to_string()),
                crate::progenitor_support::encode_path(&teamwork_tag_member_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/joinedTeams/{team-id}/tags/{teamworkTag-id}/members/{teamworkTagMember-id}` endpoint.
    */
    pub async fn users_joined_teams_tags_update_members(
        &self,
        user_id: &str,
        team_id: &str,
        teamwork_tag_id: &str,
        teamwork_tag_member_id: &str,
        body: &crate::types::MicrosoftGraphTeamworkTagMemberAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTeamworkTagMemberAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/joinedTeams/{}/tags/{}/members/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&teamwork_tag_id.to_string()),
                crate::progenitor_support::encode_path(&teamwork_tag_member_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/tags/{teamworkTag-id}/members/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_tags_members_get_count_f_8b_7(
        &self,
        user_id: &str,
        team_id: &str,
        teamwork_tag_id: &str,
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
                "/users/{}/joinedTeams/{}/tags/{}/members/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
                crate::progenitor_support::encode_path(&teamwork_tag_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/tags/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_tags_get_count_7511(
        &self,
        user_id: &str,
        team_id: &str,
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
                "/users/{}/joinedTeams/{}/tags/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * Get template from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/{team-id}/template` endpoint.
    *
    * The template this team was created from. See available templates.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_joined_teams_get_template(
        &self,
        user_id: &str,
        team_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
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
                "/users/{}/joinedTeams/{}/template?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&team_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/joinedTeams/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_joined_teams_get_count_0d_57(
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
                "/users/{}/joinedTeams/$count?{}",
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
