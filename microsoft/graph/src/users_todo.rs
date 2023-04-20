use crate::Client;
use crate::ClientResult;

pub struct UsersTodo {
    pub client: Client,
}

impl UsersTodo {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersTodo { client }
    }

    /**
    * Get todo from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo` endpoint.
    *
    * Represents the To Do services available to a user.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_get_todo(
        &self,
        user_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphTodoAllOf> {
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
                "/users/{}/todo?{}",
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
    * Delete navigation property todo for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/todo` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_delete_todo(&self, user_id: &str, if_match: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo",
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
    * Update the navigation property todo in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/todo` endpoint.
    */
    pub async fn users_update_todo(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphTodoAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTodoAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo",
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
    * List lists.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists` endpoint.
    *
    * Get a list of the todoTaskList objects and their properties.
    *
    * FROM: <https://docs.microsoft.com/graph/api/todo-list-lists?view=graph-rest-1.0>
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
    pub async fn list_list(
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
                "/users/{}/todo/lists?{}",
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
    * Create todoTaskList.
    *
    * This function performs a `POST` to the `/users/{user-id}/todo/lists` endpoint.
    *
    * Create a new lists object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/todo-post-lists?view=graph-rest-1.0>
    */
    pub async fn create_lists(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphTodoTaskListAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTodoTaskListAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists",
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
    * Get lists from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}` endpoint.
    *
    * The task lists in the users mailbox.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn get_list(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphTodoTaskListAllOf> {
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
                "/users/{}/todo/lists/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
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
    * Delete navigation property lists for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/todo/lists/{todoTaskList-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn delete_lists(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
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
    * Update the navigation property lists in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/todo/lists/{todoTaskList-id}` endpoint.
    */
    pub async fn update_lists(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        body: &crate::types::MicrosoftGraphTodoTaskListAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTodoTaskListAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
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
    * Get extensions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/extensions` endpoint.
    *
    * The collection of open extensions defined for the task list. Nullable.
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
    pub async fn lists_list_extension(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
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
                "/users/{}/todo/lists/{}/extensions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
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
    * Create new navigation property to extensions for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/extensions` endpoint.
    */
    pub async fn lists_create_extensions(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/extensions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
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
    * Get extensions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the task list. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn lists_get_extension(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        extension_id: &str,
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
                "/users/{}/todo/lists/{}/extensions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
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
    * Delete navigation property extensions for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn lists_delete_extensions(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
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
    * Update the navigation property extensions in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/extensions/{extension-id}` endpoint.
    */
    pub async fn lists_update_extensions(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn lists_extensions_get_count_2_7f_6(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
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
                "/users/{}/todo/lists/{}/extensions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
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
    * List tasks.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks` endpoint.
    *
    * Get the **todoTask** resources from the **tasks** navigation property of a specified todoTaskList.
    *
    * FROM: <https://docs.microsoft.com/graph/api/todotasklist-list-tasks?view=graph-rest-1.0>
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
    pub async fn lists_list_task(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
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
                "/users/{}/todo/lists/{}/tasks?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
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
    * Create todoTask.
    *
    * This function performs a `POST` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks` endpoint.
    *
    * Create a new task object in a specified todoTaskList.
    *
    * FROM: <https://docs.microsoft.com/graph/api/todotasklist-post-tasks?view=graph-rest-1.0>
    */
    pub async fn lists_create_tasks(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        body: &crate::types::MicrosoftGraphTodoTaskAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTodoTaskAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
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
    * Get tasks from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}` endpoint.
    *
    * The tasks in this task list. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn lists_get_task(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphTodoTaskAllOf> {
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
                "/users/{}/todo/lists/{}/tasks/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * Delete navigation property tasks for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn lists_delete_tasks(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * Update the navigation property tasks in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}` endpoint.
    */
    pub async fn lists_update_tasks(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        body: &crate::types::MicrosoftGraphTodoTaskAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphTodoTaskAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * List taskFileAttachments.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/attachments` endpoint.
    *
    * Get a list of the taskFileAttachment objects and their properties. The **contentBytes** property will not be returned in the response. Use the Get attachment API to view the **contentBytes**.
    *
    * FROM: <https://docs.microsoft.com/graph/api/todotask-list-attachments?view=graph-rest-1.0>
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
    pub async fn lists_tasks_list_attachment(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
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
                "/users/{}/todo/lists/{}/tasks/{}/attachments?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * Create taskFileAttachment.
    *
    * This function performs a `POST` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/attachments` endpoint.
    *
    * Add a new taskFileAttachment object to a todoTask. This operation limits the size of the attachment you can add to under 3 MB. If the size of the file attachments is more than 3 MB, create an upload session to upload the attachments.
    *
    * FROM: <https://docs.microsoft.com/graph/api/todotask-post-attachments?view=graph-rest-1.0>
    */
    pub async fn lists_tasks_create_attachments(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        body: &crate::types::MicrosoftGraphAttachmentBaseAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentBaseAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/attachments",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * Get attachments from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/attachments/{attachmentBase-id}` endpoint.
    *
    * A collection of file attachments for the task.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn lists_tasks_get_attachment(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        attachment_base_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentBaseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/attachments/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&attachment_base_id.to_string()),
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
    * Delete navigation property attachments for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/attachments/{attachmentBase-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn lists_tasks_delete_attachments(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        attachment_base_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/attachments/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&attachment_base_id.to_string()),
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
    * Get media content for the navigation property attachments from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/attachments/{attachmentBase-id}/$value` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/todotask-list-attachments?view=graph-rest-1.0>
    */
    pub async fn lists_tasks_get_attachments_content(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        attachment_base_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/attachments/{}/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&attachment_base_id.to_string()),
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
    * Update media content for the navigation property attachments in users.
    *
    * This function performs a `PUT` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/attachments/{attachmentBase-id}/$value` endpoint.
    */
    pub async fn lists_tasks_update_attachments_content<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        attachment_base_id: &str,
        body: B,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/attachments/{}/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&attachment_base_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/attachments/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn lists_tasks_attachments_get_count_e_9c_9(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/attachments/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * Get attachmentSessions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/attachmentSessions` endpoint.
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
    pub async fn lists_tasks_list_attachment_session(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
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
                "/users/{}/todo/lists/{}/tasks/{}/attachmentSessions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * Get attachmentSessions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/attachmentSessions/{attachmentSession-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn lists_tasks_get_attachment_session(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        attachment_session_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentSessionAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/attachmentSessions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&attachment_session_id.to_string()),
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
    * Delete navigation property attachmentSessions for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/attachmentSessions/{attachmentSession-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn lists_tasks_delete_attachment_sessions(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        attachment_session_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/attachmentSessions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&attachment_session_id.to_string()),
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
    * Update the navigation property attachmentSessions in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/attachmentSessions/{attachmentSession-id}` endpoint.
    */
    pub async fn lists_tasks_update_attachment_sessions(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        attachment_session_id: &str,
        body: &crate::types::MicrosoftGraphAttachmentSessionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentSessionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/attachmentSessions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&attachment_session_id.to_string()),
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
    * Get content for the navigation property attachmentSessions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/attachmentSessions/{attachmentSession-id}/content` endpoint.
    *
    * The content streams that are uploaded.
    */
    pub async fn lists_tasks_get_attachment_sessions_content(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        attachment_session_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/attachmentSessions/{}/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&attachment_session_id.to_string()),
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
    * Update content for the navigation property attachmentSessions in users.
    *
    * This function performs a `PUT` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/attachmentSessions/{attachmentSession-id}/content` endpoint.
    *
    * The content streams that are uploaded.
    */
    pub async fn lists_tasks_update_attachment_sessions_content<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        attachment_session_id: &str,
        body: B,
    ) -> ClientResult<crate::types::MicrosoftGraphAttachmentSessionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/attachmentSessions/{}/content",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&attachment_session_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/attachmentSessions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn lists_tasks_attachment_sessions_get_count_5_3ee(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/attachmentSessions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * List checklistItems.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/checklistItems` endpoint.
    *
    * Get the checklistItem resources associated to a todoTask from the checklistItems navigation property.
    *
    * FROM: <https://docs.microsoft.com/graph/api/todotask-list-checklistitems?view=graph-rest-1.0>
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
    pub async fn lists_tasks_list_checklist_item(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
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
                "/users/{}/todo/lists/{}/tasks/{}/checklistItems?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * Create checklistItem.
    *
    * This function performs a `POST` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/checklistItems` endpoint.
    *
    * Create a new checklistItem object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/todotask-post-checklistitems?view=graph-rest-1.0>
    */
    pub async fn lists_tasks_create_checklist_items(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        body: &crate::types::MicrosoftGraphChecklistItemAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChecklistItemAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/checklistItems",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * Get checklistItems from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/checklistItems/{checklistItem-id}` endpoint.
    *
    * A collection of checklistItems linked to a task.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn lists_tasks_get_checklist_item(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        checklist_item_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphChecklistItemAllOf> {
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
                "/users/{}/todo/lists/{}/tasks/{}/checklistItems/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&checklist_item_id.to_string()),
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
    * Delete navigation property checklistItems for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/checklistItems/{checklistItem-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn lists_tasks_delete_checklist_items(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        checklist_item_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/checklistItems/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&checklist_item_id.to_string()),
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
    * Update the navigation property checklistItems in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/checklistItems/{checklistItem-id}` endpoint.
    */
    pub async fn lists_tasks_update_checklist_items(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        checklist_item_id: &str,
        body: &crate::types::MicrosoftGraphChecklistItemAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphChecklistItemAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/checklistItems/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&checklist_item_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/checklistItems/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn lists_tasks_checklist_items_get_count_f_104(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
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
                "/users/{}/todo/lists/{}/tasks/{}/checklistItems/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * Get extensions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/extensions` endpoint.
    *
    * The collection of open extensions defined for the task. Nullable.
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
    pub async fn lists_tasks_list_extension(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
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
                "/users/{}/todo/lists/{}/tasks/{}/extensions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * Create new navigation property to extensions for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/extensions` endpoint.
    */
    pub async fn lists_tasks_create_extensions(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/extensions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * Get extensions from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the task. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn lists_tasks_get_extension(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        extension_id: &str,
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
                "/users/{}/todo/lists/{}/tasks/{}/extensions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
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
    * Delete navigation property extensions for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn lists_tasks_delete_extensions(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
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
    * Update the navigation property extensions in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/extensions/{extension-id}` endpoint.
    */
    pub async fn lists_tasks_update_extensions(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&extension_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn lists_tasks_extensions_get_count_c_962(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
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
                "/users/{}/todo/lists/{}/tasks/{}/extensions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * List linkedResources.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/linkedResources` endpoint.
    *
    * Get information of one or more items in a partner application, based on which a specified task was created. The information is represented in a linkedResource object for each item. It includes an external ID for the item in the partner application, and if applicable, a deep link to that item in the application.
    *
    * FROM: <https://docs.microsoft.com/graph/api/todotask-list-linkedresources?view=graph-rest-1.0>
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
    pub async fn lists_tasks_list_linked_resource(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
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
                "/users/{}/todo/lists/{}/tasks/{}/linkedResources?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * Create linkedResource.
    *
    * This function performs a `POST` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/linkedResources` endpoint.
    *
    * Create a linkedResource object to associate a specified task with an item in a partner application. For example, you can associate a task with an email item in Outlook that spurred the task, and you can create a **linkedResource** object to track its association. You can also create a **linkedResource** object while creating a task.
    *
    * FROM: <https://docs.microsoft.com/graph/api/todotask-post-linkedresources?view=graph-rest-1.0>
    */
    pub async fn lists_tasks_create_linked_resources(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        body: &crate::types::MicrosoftGraphLinkedResourceAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphLinkedResourceAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/linkedResources",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * Get linkedResources from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/linkedResources/{linkedResource-id}` endpoint.
    *
    * A collection of resources linked to the task.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn lists_tasks_get_linked_resource(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        linked_resource_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphLinkedResourceAllOf> {
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
                "/users/{}/todo/lists/{}/tasks/{}/linkedResources/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&linked_resource_id.to_string()),
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
    * Delete navigation property linkedResources for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/linkedResources/{linkedResource-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn lists_tasks_delete_linked_resources(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        linked_resource_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/linkedResources/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&linked_resource_id.to_string()),
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
    * Update the navigation property linkedResources in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/linkedResources/{linkedResource-id}` endpoint.
    */
    pub async fn lists_tasks_update_linked_resources(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
        linked_resource_id: &str,
        body: &crate::types::MicrosoftGraphLinkedResourceAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphLinkedResourceAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/todo/lists/{}/tasks/{}/linkedResources/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
                crate::progenitor_support::encode_path(&linked_resource_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/{todoTask-id}/linkedResources/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn lists_tasks_linked_resources_get_count_5_8c_2(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
        todo_task_id: &str,
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
                "/users/{}/todo/lists/{}/tasks/{}/linkedResources/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/{todoTaskList-id}/tasks/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn lists_tasks_get_count_9520(
        &self,
        user_id: &str,
        todo_task_list_id: &str,
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
                "/users/{}/todo/lists/{}/tasks/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&todo_task_list_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/todo/lists/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn lists_get_count_4_7c(
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
                "/users/{}/todo/lists/$count?{}",
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
