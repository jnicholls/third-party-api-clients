use crate::Client;
use crate::ClientResult;

pub struct UsersOnenote {
    pub client: Client,
}

impl UsersOnenote {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersOnenote { client }
    }

    /**
    * Get onenote from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote` endpoint.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_get_onenote(
        &self,
        user_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteAllOf> {
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
                "/users/{}/onenote?{}",
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
    * Delete navigation property onenote for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/onenote` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_delete_onenote(&self, user_id: &str, if_match: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote",
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
    * Update the navigation property onenote in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/onenote` endpoint.
    */
    pub async fn users_update_onenote(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphOnenoteAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote",
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
    * List notebooks.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks` endpoint.
    *
    * Retrieve a list of notebook objects.
    *
    * FROM: <https://docs.microsoft.com/graph/api/onenote-list-notebooks?view=graph-rest-1.0>
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
    pub async fn list_notebook(
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
                "/users/{}/onenote/notebooks?{}",
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
    * Create notebook.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/notebooks` endpoint.
    *
    * Create a new OneNote notebook.
    *
    * FROM: <https://docs.microsoft.com/graph/api/onenote-post-notebooks?view=graph-rest-1.0>
    */
    pub async fn create_notebooks(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphNotebookAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphNotebookAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks",
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
    * Get notebooks from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}` endpoint.
    *
    * The collection of OneNote notebooks that are owned by the user or group. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn get_notebook(
        &self,
        user_id: &str,
        notebook_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphNotebookAllOf> {
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
                "/users/{}/onenote/notebooks/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
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
    * Delete navigation property notebooks for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/onenote/notebooks/{notebook-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn delete_notebooks(
        &self,
        user_id: &str,
        notebook_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
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
    * Update the navigation property notebooks in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/onenote/notebooks/{notebook-id}` endpoint.
    */
    pub async fn update_notebooks(
        &self,
        user_id: &str,
        notebook_id: &str,
        body: &crate::types::MicrosoftGraphNotebookAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphNotebookAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
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
    * List sectionGroups.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups` endpoint.
    *
    * Retrieve a list of section groups from the specified notebook.
    *
    * FROM: <https://docs.microsoft.com/graph/api/notebook-list-sectiongroups?view=graph-rest-1.0>
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
    pub async fn notebooks_list_section_group(
        &self,
        user_id: &str,
        notebook_id: &str,
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
                "/users/{}/onenote/notebooks/{}/sectionGroups?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
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
    * Create sectionGroup.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups` endpoint.
    *
    * Create a new section group in the specified notebook.
    *
    * FROM: <https://docs.microsoft.com/graph/api/notebook-post-sectiongroups?view=graph-rest-1.0>
    */
    pub async fn notebooks_create_section_groups(
        &self,
        user_id: &str,
        notebook_id: &str,
        body: &crate::types::MicrosoftGraphSectionGroupAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSectionGroupAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sectionGroups",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
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
    * Get sectionGroups from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}` endpoint.
    *
    * The section groups in the notebook. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn notebooks_get_section_group(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSectionGroupAllOf> {
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
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * Delete navigation property sectionGroups for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn notebooks_delete_section_groups(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * Update the navigation property sectionGroups in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}` endpoint.
    */
    pub async fn notebooks_update_section_groups(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        body: &crate::types::MicrosoftGraphSectionGroupAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSectionGroupAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * Get parentNotebook from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/parentNotebook` endpoint.
    *
    * The notebook that contains the section group. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn notebooks_section_groups_get_parent_notebook(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphNotebookAllOf> {
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
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/parentNotebook?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * Get parentSectionGroup from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/parentSectionGroup` endpoint.
    *
    * The section group that contains the section group. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn notebooks_section_groups_get_parent_group(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSectionGroupAllOf> {
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
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/parentSectionGroup?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * List sectionGroups.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sectionGroups` endpoint.
    *
    * Retrieve a list of section groups from the specified section group.
    *
    * FROM: <https://docs.microsoft.com/graph/api/sectiongroup-list-sectiongroups?view=graph-rest-1.0>
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
    pub async fn notebooks_section_groups_list(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
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
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sectionGroups?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * Get sectionGroups from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sectionGroups/{sectionGroup-id1}` endpoint.
    *
    * The section groups in the section. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn notebooks_section_groups_get(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        section_group_id_1: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSectionGroupAllOf> {
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
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sectionGroups/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id_1.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sectionGroups/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn notebooks_section_groups_get_count_2e_9f(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
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
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sectionGroups/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * List sections.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections` endpoint.
    *
    * Retrieve a list of onenoteSection objects from the specified section group.
    *
    * FROM: <https://docs.microsoft.com/graph/api/sectiongroup-list-sections?view=graph-rest-1.0>
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
    pub async fn notebooks_section_groups_list_section(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
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
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * Create section.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections` endpoint.
    *
    * Create a new onenoteSection in the specified section group.
    *
    * FROM: <https://docs.microsoft.com/graph/api/sectiongroup-post-sections?view=graph-rest-1.0>
    */
    pub async fn notebooks_section_groups_create_sections(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        body: &crate::types::MicrosoftGraphOnenoteSectionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteSectionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * Get sections from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}` endpoint.
    *
    * The sections in the section group. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn notebooks_section_groups_get_section(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteSectionAllOf> {
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
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Delete navigation property sections for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn notebooks_section_groups_delete_sections(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Update the navigation property sections in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}` endpoint.
    */
    pub async fn notebooks_section_groups_update_sections(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        body: &crate::types::MicrosoftGraphOnenoteSectionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteSectionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Get pages from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages` endpoint.
    *
    * The collection of pages in the section.  Read-only. Nullable.
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
    pub async fn notebooks_section_groups_sections_list_page(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
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
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}/pages?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Create new navigation property to pages for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages` endpoint.
    */
    pub async fn notebooks_section_groups_sections_create_pages(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        body: &crate::types::MicrosoftGraphOnenotePageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}/pages",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Get pages from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}` endpoint.
    *
    * The collection of pages in the section.  Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn notebooks_section_groups_sections_get_page(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
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
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}/pages/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Delete navigation property pages for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn notebooks_section_groups_sections_delete_pages(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}/pages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Update the navigation property pages in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}` endpoint.
    */
    pub async fn notebooks_section_groups_sections_update_pages(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        body: &crate::types::MicrosoftGraphOnenotePageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}/pages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Get content for the navigation property pages from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/content` endpoint.
    *
    * The page's HTML content.
    */
    pub async fn notebooks_section_groups_sections_get_pages_content(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}/pages/{}/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Update content for the navigation property pages in users.
    *
    * This function performs a `PUT` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/content` endpoint.
    *
    * The page's HTML content.
    */
    pub async fn notebooks_section_groups_sections_update_pages_content<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        body: B,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}/pages/{}/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Get parentNotebook from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/parentNotebook` endpoint.
    *
    * The notebook that contains the page.  Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn notebooks_section_groups_sections_pages_get_parent_notebook(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphNotebookAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
&format!("/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}/pages/{}/parentNotebook?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&notebook_id.to_string()),crate::progenitor_support::encode_path(&section_group_id.to_string()),crate::progenitor_support::encode_path(&onenote_section_id.to_string()),crate::progenitor_support::encode_path(&onenote_page_id.to_string()),query_), None);
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
    * Get parentSection from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/parentSection` endpoint.
    *
    * The section that contains the page. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn notebooks_section_groups_sections_pages_get_parent(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteSectionAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("$expand".to_string(), expand.join(" ")));
        }
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
&format!("/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}/pages/{}/parentSection?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&notebook_id.to_string()),crate::progenitor_support::encode_path(&section_group_id.to_string()),crate::progenitor_support::encode_path(&onenote_section_id.to_string()),crate::progenitor_support::encode_path(&onenote_page_id.to_string()),query_), None);
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
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn notebooks_section_groups_sections_pages_get_count_323(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
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
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}/pages/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Get parentNotebook from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/parentNotebook` endpoint.
    *
    * The notebook that contains the section.  Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn notebooks_section_groups_sections_get_parent_notebook(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphNotebookAllOf> {
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
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}/parentNotebook?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Get parentSectionGroup from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/parentSectionGroup` endpoint.
    *
    * The section group that contains the section.  Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn notebooks_section_groups_sections_get_parent_group(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSectionGroupAllOf> {
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
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/{}/parentSectionGroup?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/{sectionGroup-id}/sections/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn notebooks_section_groups_sections_get_count_2e_73(
        &self,
        user_id: &str,
        notebook_id: &str,
        section_group_id: &str,
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
                "/users/{}/onenote/notebooks/{}/sectionGroups/{}/sections/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sectionGroups/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn notebooks_section_groups_get_count_2e_9f_users_onenote(
        &self,
        user_id: &str,
        notebook_id: &str,
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
                "/users/{}/onenote/notebooks/{}/sectionGroups/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
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
    * List sections.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections` endpoint.
    *
    * Retrieve a list of onenoteSection objects from the specified notebook.
    *
    * FROM: <https://docs.microsoft.com/graph/api/notebook-list-sections?view=graph-rest-1.0>
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
    pub async fn notebooks_list_section(
        &self,
        user_id: &str,
        notebook_id: &str,
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
                "/users/{}/onenote/notebooks/{}/sections?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
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
    * Create section.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections` endpoint.
    *
    * Create a new onenoteSection in the specified notebook.
    *
    * FROM: <https://docs.microsoft.com/graph/api/notebook-post-sections?view=graph-rest-1.0>
    */
    pub async fn notebooks_create_sections(
        &self,
        user_id: &str,
        notebook_id: &str,
        body: &crate::types::MicrosoftGraphOnenoteSectionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteSectionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sections",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
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
    * Get sections from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}` endpoint.
    *
    * The sections in the notebook. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn notebooks_get_section(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteSectionAllOf> {
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
                "/users/{}/onenote/notebooks/{}/sections/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Delete navigation property sections for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn notebooks_delete_sections(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sections/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Update the navigation property sections in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}` endpoint.
    */
    pub async fn notebooks_update_sections(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
        body: &crate::types::MicrosoftGraphOnenoteSectionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteSectionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sections/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Get pages from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}/pages` endpoint.
    *
    * The collection of pages in the section.  Read-only. Nullable.
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
    pub async fn notebooks_sections_list_page(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
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
                "/users/{}/onenote/notebooks/{}/sections/{}/pages?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Create new navigation property to pages for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}/pages` endpoint.
    */
    pub async fn notebooks_sections_create_pages(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
        body: &crate::types::MicrosoftGraphOnenotePageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sections/{}/pages",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Get pages from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}` endpoint.
    *
    * The collection of pages in the section.  Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn notebooks_sections_get_page(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
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
                "/users/{}/onenote/notebooks/{}/sections/{}/pages/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Delete navigation property pages for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn notebooks_sections_delete_pages(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sections/{}/pages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Update the navigation property pages in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}` endpoint.
    */
    pub async fn notebooks_sections_update_pages(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        body: &crate::types::MicrosoftGraphOnenotePageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sections/{}/pages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Get content for the navigation property pages from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/content` endpoint.
    *
    * The page's HTML content.
    */
    pub async fn notebooks_sections_get_pages_content(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sections/{}/pages/{}/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Update content for the navigation property pages in users.
    *
    * This function performs a `PUT` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/content` endpoint.
    *
    * The page's HTML content.
    */
    pub async fn notebooks_sections_update_pages_content<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        body: B,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/notebooks/{}/sections/{}/pages/{}/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Get parentNotebook from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/parentNotebook` endpoint.
    *
    * The notebook that contains the page.  Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn notebooks_sections_pages_get_parent_notebook(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphNotebookAllOf> {
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
                "/users/{}/onenote/notebooks/{}/sections/{}/pages/{}/parentNotebook?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Get parentSection from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/parentSection` endpoint.
    *
    * The section that contains the page. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn notebooks_sections_pages_get_parent_section(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteSectionAllOf> {
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
                "/users/{}/onenote/notebooks/{}/sections/{}/pages/{}/parentSection?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}/pages/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn notebooks_sections_pages_get_count_c_5e_5(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
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
                "/users/{}/onenote/notebooks/{}/sections/{}/pages/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Get parentNotebook from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}/parentNotebook` endpoint.
    *
    * The notebook that contains the section.  Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn notebooks_sections_get_parent_notebook(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphNotebookAllOf> {
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
                "/users/{}/onenote/notebooks/{}/sections/{}/parentNotebook?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Get parentSectionGroup from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/{onenoteSection-id}/parentSectionGroup` endpoint.
    *
    * The section group that contains the section.  Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn notebooks_sections_get_parent_section_group(
        &self,
        user_id: &str,
        notebook_id: &str,
        onenote_section_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSectionGroupAllOf> {
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
                "/users/{}/onenote/notebooks/{}/sections/{}/parentSectionGroup?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/{notebook-id}/sections/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn notebooks_sections_get_count_2f_79(
        &self,
        user_id: &str,
        notebook_id: &str,
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
                "/users/{}/onenote/notebooks/{}/sections/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&notebook_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/onenote/notebooks/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn notebooks_get_count_af_06(
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
                "/users/{}/onenote/notebooks/$count?{}",
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
    * Get operations from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/operations` endpoint.
    *
    * The status of OneNote operations. Getting an operations collection is not supported, but you can get the status of long-running operations if the Operation-Location header is returned in the response. Read-only. Nullable.
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
    pub async fn list_operation(
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
                "/users/{}/onenote/operations?{}",
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
    * Create new navigation property to operations for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/operations` endpoint.
    */
    pub async fn create_operations(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphOnenoteOperationAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteOperationAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/operations",
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
    * Get operations from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/operations/{onenoteOperation-id}` endpoint.
    *
    * The status of OneNote operations. Getting an operations collection is not supported, but you can get the status of long-running operations if the Operation-Location header is returned in the response. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn get_operation(
        &self,
        user_id: &str,
        onenote_operation_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteOperationAllOf> {
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
                "/users/{}/onenote/operations/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_operation_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/onenote/operations/{onenoteOperation-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn delete_operations(
        &self,
        user_id: &str,
        onenote_operation_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/operations/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_operation_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/onenote/operations/{onenoteOperation-id}` endpoint.
    */
    pub async fn update_operations(
        &self,
        user_id: &str,
        onenote_operation_id: &str,
        body: &crate::types::MicrosoftGraphOnenoteOperationAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteOperationAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/operations/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_operation_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/onenote/operations/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn operations_get_count_adfa(
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
                "/users/{}/onenote/operations/$count?{}",
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
    * Get pages from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/pages` endpoint.
    *
    * The pages in all OneNote notebooks that are owned by the user or group.  Read-only. Nullable.
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
    pub async fn list_page(
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
                "/users/{}/onenote/pages?{}",
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
    * Create new navigation property to pages for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/pages` endpoint.
    */
    pub async fn create_pages(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphOnenotePageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/pages",
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
    * Get pages from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/pages/{onenotePage-id}` endpoint.
    *
    * The pages in all OneNote notebooks that are owned by the user or group.  Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn get_page(
        &self,
        user_id: &str,
        onenote_page_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
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
                "/users/{}/onenote/pages/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Delete navigation property pages for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/onenote/pages/{onenotePage-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn delete_pages(
        &self,
        user_id: &str,
        onenote_page_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/pages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Update the navigation property pages in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/onenote/pages/{onenotePage-id}` endpoint.
    */
    pub async fn update_pages(
        &self,
        user_id: &str,
        onenote_page_id: &str,
        body: &crate::types::MicrosoftGraphOnenotePageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/pages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Get content for the navigation property pages from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/pages/{onenotePage-id}/content` endpoint.
    *
    * The page's HTML content.
    */
    pub async fn get_pages_content(
        &self,
        user_id: &str,
        onenote_page_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/pages/{}/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Update content for the navigation property pages in users.
    *
    * This function performs a `PUT` to the `/users/{user-id}/onenote/pages/{onenotePage-id}/content` endpoint.
    *
    * The page's HTML content.
    */
    pub async fn update_pages_content<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        onenote_page_id: &str,
        body: B,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/pages/{}/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Get parentNotebook from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/pages/{onenotePage-id}/parentNotebook` endpoint.
    *
    * The notebook that contains the page.  Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn pages_get_parent_notebook(
        &self,
        user_id: &str,
        onenote_page_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphNotebookAllOf> {
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
                "/users/{}/onenote/pages/{}/parentNotebook?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Get parentSection from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/pages/{onenotePage-id}/parentSection` endpoint.
    *
    * The section that contains the page. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn pages_get_parent_section(
        &self,
        user_id: &str,
        onenote_page_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteSectionAllOf> {
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
                "/users/{}/onenote/pages/{}/parentSection?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/onenote/pages/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn pages_get_count_b_080(
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
                "/users/{}/onenote/pages/$count?{}",
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
    * Get resources from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/resources` endpoint.
    *
    * The image and other file resources in OneNote pages. Getting a resources collection is not supported, but you can get the binary content of a specific resource. Read-only. Nullable.
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
    pub async fn list_resource(
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
                "/users/{}/onenote/resources?{}",
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
    * Create new navigation property to resources for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/resources` endpoint.
    */
    pub async fn create_resources(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphOnenoteResourceAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteResourceAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/resources",
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
    * Get resources from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/resources/{onenoteResource-id}` endpoint.
    *
    * The image and other file resources in OneNote pages. Getting a resources collection is not supported, but you can get the binary content of a specific resource. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn get_resource(
        &self,
        user_id: &str,
        onenote_resource_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteResourceAllOf> {
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
                "/users/{}/onenote/resources/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_resource_id.to_string()),
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
    * Delete navigation property resources for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/onenote/resources/{onenoteResource-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn delete_resources(
        &self,
        user_id: &str,
        onenote_resource_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/resources/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_resource_id.to_string()),
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
    * Update the navigation property resources in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/onenote/resources/{onenoteResource-id}` endpoint.
    */
    pub async fn update_resources(
        &self,
        user_id: &str,
        onenote_resource_id: &str,
        body: &crate::types::MicrosoftGraphOnenoteResourceAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteResourceAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/resources/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_resource_id.to_string()),
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
    * Get content for the navigation property resources from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/resources/{onenoteResource-id}/content` endpoint.
    *
    * The content stream
    */
    pub async fn get_resources_content(
        &self,
        user_id: &str,
        onenote_resource_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/resources/{}/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_resource_id.to_string()),
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
    * Update content for the navigation property resources in users.
    *
    * This function performs a `PUT` to the `/users/{user-id}/onenote/resources/{onenoteResource-id}/content` endpoint.
    *
    * The content stream
    */
    pub async fn update_resources_content<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        onenote_resource_id: &str,
        body: B,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteResourceAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/resources/{}/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_resource_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/onenote/resources/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn resources_get_count_dba_6(
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
                "/users/{}/onenote/resources/$count?{}",
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
    * List sectionGroups.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups` endpoint.
    *
    * Retrieve a list of sectionGroup objects.
    *
    * FROM: <https://docs.microsoft.com/graph/api/onenote-list-sectiongroups?view=graph-rest-1.0>
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
    pub async fn list_section_group(
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
                "/users/{}/onenote/sectionGroups?{}",
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
    * Create new navigation property to sectionGroups for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/sectionGroups` endpoint.
    */
    pub async fn create_section_groups(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphSectionGroupAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSectionGroupAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sectionGroups",
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
    * Get sectionGroups from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}` endpoint.
    *
    * The section groups in all OneNote notebooks that are owned by the user or group.  Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn get_section_group(
        &self,
        user_id: &str,
        section_group_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSectionGroupAllOf> {
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
                "/users/{}/onenote/sectionGroups/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * Delete navigation property sectionGroups for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn delete_section_groups(
        &self,
        user_id: &str,
        section_group_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sectionGroups/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * Update the navigation property sectionGroups in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}` endpoint.
    */
    pub async fn update_section_groups(
        &self,
        user_id: &str,
        section_group_id: &str,
        body: &crate::types::MicrosoftGraphSectionGroupAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSectionGroupAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sectionGroups/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * Get parentNotebook from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/parentNotebook` endpoint.
    *
    * The notebook that contains the section group. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn section_groups_get_parent_notebook(
        &self,
        user_id: &str,
        section_group_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphNotebookAllOf> {
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
                "/users/{}/onenote/sectionGroups/{}/parentNotebook?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * Get parentSectionGroup from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/parentSectionGroup` endpoint.
    *
    * The section group that contains the section group. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn section_groups_get_parent_group(
        &self,
        user_id: &str,
        section_group_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSectionGroupAllOf> {
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
                "/users/{}/onenote/sectionGroups/{}/parentSectionGroup?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * List sectionGroups.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sectionGroups` endpoint.
    *
    * Retrieve a list of section groups from the specified section group.
    *
    * FROM: <https://docs.microsoft.com/graph/api/sectiongroup-list-sectiongroups?view=graph-rest-1.0>
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
    pub async fn section_groups_list(
        &self,
        user_id: &str,
        section_group_id: &str,
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
                "/users/{}/onenote/sectionGroups/{}/sectionGroups?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * Get sectionGroups from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sectionGroups/{sectionGroup-id1}` endpoint.
    *
    * The section groups in the section. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn section_groups_get(
        &self,
        user_id: &str,
        section_group_id: &str,
        section_group_id_1: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSectionGroupAllOf> {
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
                "/users/{}/onenote/sectionGroups/{}/sectionGroups/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id_1.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sectionGroups/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn section_groups_get_count_49_7a(
        &self,
        user_id: &str,
        section_group_id: &str,
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
                "/users/{}/onenote/sectionGroups/{}/sectionGroups/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * List sections.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections` endpoint.
    *
    * Retrieve a list of onenoteSection objects from the specified section group.
    *
    * FROM: <https://docs.microsoft.com/graph/api/sectiongroup-list-sections?view=graph-rest-1.0>
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
    pub async fn section_groups_list_section(
        &self,
        user_id: &str,
        section_group_id: &str,
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
                "/users/{}/onenote/sectionGroups/{}/sections?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * Create section.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections` endpoint.
    *
    * Create a new onenoteSection in the specified section group.
    *
    * FROM: <https://docs.microsoft.com/graph/api/sectiongroup-post-sections?view=graph-rest-1.0>
    */
    pub async fn section_groups_create_sections(
        &self,
        user_id: &str,
        section_group_id: &str,
        body: &crate::types::MicrosoftGraphOnenoteSectionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteSectionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sectionGroups/{}/sections",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * Get sections from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}` endpoint.
    *
    * The sections in the section group. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn section_groups_get_section(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteSectionAllOf> {
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
                "/users/{}/onenote/sectionGroups/{}/sections/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Delete navigation property sections for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn section_groups_delete_sections(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sectionGroups/{}/sections/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Update the navigation property sections in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}` endpoint.
    */
    pub async fn section_groups_update_sections(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        body: &crate::types::MicrosoftGraphOnenoteSectionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteSectionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sectionGroups/{}/sections/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Get pages from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages` endpoint.
    *
    * The collection of pages in the section.  Read-only. Nullable.
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
    pub async fn section_groups_sections_list_page(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
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
                "/users/{}/onenote/sectionGroups/{}/sections/{}/pages?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Create new navigation property to pages for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages` endpoint.
    */
    pub async fn section_groups_sections_create_pages(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        body: &crate::types::MicrosoftGraphOnenotePageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sectionGroups/{}/sections/{}/pages",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Get pages from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}` endpoint.
    *
    * The collection of pages in the section.  Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn section_groups_sections_get_page(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
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
                "/users/{}/onenote/sectionGroups/{}/sections/{}/pages/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Delete navigation property pages for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn section_groups_sections_delete_pages(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sectionGroups/{}/sections/{}/pages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Update the navigation property pages in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}` endpoint.
    */
    pub async fn section_groups_sections_update_pages(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        body: &crate::types::MicrosoftGraphOnenotePageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sectionGroups/{}/sections/{}/pages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Get content for the navigation property pages from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/content` endpoint.
    *
    * The page's HTML content.
    */
    pub async fn section_groups_sections_get_pages_content(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sectionGroups/{}/sections/{}/pages/{}/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Update content for the navigation property pages in users.
    *
    * This function performs a `PUT` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/content` endpoint.
    *
    * The page's HTML content.
    */
    pub async fn section_groups_sections_update_pages_content<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        body: B,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sectionGroups/{}/sections/{}/pages/{}/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Get parentNotebook from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/parentNotebook` endpoint.
    *
    * The notebook that contains the page.  Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn section_groups_sections_pages_get_parent_notebook(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphNotebookAllOf> {
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
                "/users/{}/onenote/sectionGroups/{}/sections/{}/pages/{}/parentNotebook?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Get parentSection from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/{onenotePage-id}/parentSection` endpoint.
    *
    * The section that contains the page. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn section_groups_sections_pages_get_parent(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteSectionAllOf> {
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
                "/users/{}/onenote/sectionGroups/{}/sections/{}/pages/{}/parentSection?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/pages/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn section_groups_sections_pages_get_count_dfde(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
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
                "/users/{}/onenote/sectionGroups/{}/sections/{}/pages/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Get parentNotebook from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/parentNotebook` endpoint.
    *
    * The notebook that contains the section.  Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn section_groups_sections_get_parent_notebook(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphNotebookAllOf> {
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
                "/users/{}/onenote/sectionGroups/{}/sections/{}/parentNotebook?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Get parentSectionGroup from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/{onenoteSection-id}/parentSectionGroup` endpoint.
    *
    * The section group that contains the section.  Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn section_groups_sections_get_parent_group(
        &self,
        user_id: &str,
        section_group_id: &str,
        onenote_section_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSectionGroupAllOf> {
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
                "/users/{}/onenote/sectionGroups/{}/sections/{}/parentSectionGroup?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/{sectionGroup-id}/sections/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn section_groups_sections_get_count_6826(
        &self,
        user_id: &str,
        section_group_id: &str,
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
                "/users/{}/onenote/sectionGroups/{}/sections/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&section_group_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/onenote/sectionGroups/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn section_groups_get_count_49_7a_users_onenote(
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
                "/users/{}/onenote/sectionGroups/$count?{}",
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
    * List sections.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sections` endpoint.
    *
    * Retrieve a list of onenoteSection objects.
    *
    * FROM: <https://docs.microsoft.com/graph/api/onenote-list-sections?view=graph-rest-1.0>
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
    pub async fn list_section(
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
                "/users/{}/onenote/sections?{}",
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
    * Create new navigation property to sections for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/sections` endpoint.
    */
    pub async fn create_sections(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphOnenoteSectionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteSectionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sections",
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
    * Get sections from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}` endpoint.
    *
    * The sections in all OneNote notebooks that are owned by the user or group.  Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn get_section(
        &self,
        user_id: &str,
        onenote_section_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteSectionAllOf> {
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
                "/users/{}/onenote/sections/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Delete navigation property sections for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn delete_sections(
        &self,
        user_id: &str,
        onenote_section_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sections/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Update the navigation property sections in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}` endpoint.
    */
    pub async fn update_sections(
        &self,
        user_id: &str,
        onenote_section_id: &str,
        body: &crate::types::MicrosoftGraphOnenoteSectionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteSectionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sections/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Get pages from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}/pages` endpoint.
    *
    * The collection of pages in the section.  Read-only. Nullable.
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
    pub async fn sections_list_page(
        &self,
        user_id: &str,
        onenote_section_id: &str,
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
                "/users/{}/onenote/sections/{}/pages?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Create new navigation property to pages for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}/pages` endpoint.
    */
    pub async fn sections_create_pages(
        &self,
        user_id: &str,
        onenote_section_id: &str,
        body: &crate::types::MicrosoftGraphOnenotePageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sections/{}/pages",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Get pages from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}/pages/{onenotePage-id}` endpoint.
    *
    * The collection of pages in the section.  Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn sections_get_page(
        &self,
        user_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
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
                "/users/{}/onenote/sections/{}/pages/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Delete navigation property pages for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}/pages/{onenotePage-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn sections_delete_pages(
        &self,
        user_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sections/{}/pages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Update the navigation property pages in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}/pages/{onenotePage-id}` endpoint.
    */
    pub async fn sections_update_pages(
        &self,
        user_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        body: &crate::types::MicrosoftGraphOnenotePageAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sections/{}/pages/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Get content for the navigation property pages from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}/pages/{onenotePage-id}/content` endpoint.
    *
    * The page's HTML content.
    */
    pub async fn sections_get_pages_content(
        &self,
        user_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sections/{}/pages/{}/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Update content for the navigation property pages in users.
    *
    * This function performs a `PUT` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}/pages/{onenotePage-id}/content` endpoint.
    *
    * The page's HTML content.
    */
    pub async fn sections_update_pages_content<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        body: B,
    ) -> ClientResult<crate::types::MicrosoftGraphOnenotePageAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/onenote/sections/{}/pages/{}/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Get parentNotebook from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}/pages/{onenotePage-id}/parentNotebook` endpoint.
    *
    * The notebook that contains the page.  Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn sections_pages_get_parent_notebook(
        &self,
        user_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphNotebookAllOf> {
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
                "/users/{}/onenote/sections/{}/pages/{}/parentNotebook?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * Get parentSection from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}/pages/{onenotePage-id}/parentSection` endpoint.
    *
    * The section that contains the page. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn sections_pages_get_parent_section(
        &self,
        user_id: &str,
        onenote_section_id: &str,
        onenote_page_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphOnenoteSectionAllOf> {
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
                "/users/{}/onenote/sections/{}/pages/{}/parentSection?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_page_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}/pages/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn sections_pages_get_count_1_7eb(
        &self,
        user_id: &str,
        onenote_section_id: &str,
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
                "/users/{}/onenote/sections/{}/pages/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Get parentNotebook from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}/parentNotebook` endpoint.
    *
    * The notebook that contains the section.  Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn sections_get_parent_notebook(
        &self,
        user_id: &str,
        onenote_section_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphNotebookAllOf> {
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
                "/users/{}/onenote/sections/{}/parentNotebook?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * Get parentSectionGroup from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/onenote/sections/{onenoteSection-id}/parentSectionGroup` endpoint.
    *
    * The section group that contains the section.  Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn sections_get_parent_section_group(
        &self,
        user_id: &str,
        onenote_section_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSectionGroupAllOf> {
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
                "/users/{}/onenote/sections/{}/parentSectionGroup?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&onenote_section_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/onenote/sections/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn sections_get_count_8b_0b(
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
                "/users/{}/onenote/sections/$count?{}",
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
