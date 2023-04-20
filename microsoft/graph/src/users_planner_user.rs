use crate::Client;
use crate::ClientResult;

pub struct UsersPlannerUser {
    pub client: Client,
}

impl UsersPlannerUser {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersPlannerUser { client }
    }

    /**
    * Get planner from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner` endpoint.
    *
    * Entry-point to the Planner resource that might exist for a user. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_get_planner(
        &self,
        user_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerUserAllOf> {
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
                "/users/{}/planner?{}",
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
    * Delete navigation property planner for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/planner` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_delete_planner(&self, user_id: &str, if_match: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner",
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
    * Update the navigation property planner in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/planner` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_update_planner(
        &self,
        user_id: &str,
        if_match: &str,
        body: &crate::types::MicrosoftGraphPlannerUserAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerUserAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner",
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
    * List plans.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/plans` endpoint.
    *
    * Retrieve a list of **plannerplan** objects shared with a user object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/planneruser-list-plans?view=graph-rest-1.0>
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
    pub async fn users_planner_list_plan(
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
                "/users/{}/planner/plans?{}",
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
    * Create new navigation property to plans for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/planner/plans` endpoint.
    */
    pub async fn users_planner_create_plans(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphPlannerPlanAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerPlanAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans",
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
    * Get plans from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}` endpoint.
    *
    * Read-only. Nullable. Returns the plannerTasks assigned to the user.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_get_plan(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerPlanAllOf> {
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
                "/users/{}/planner/plans/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property plans for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/planner/plans/{plannerPlan-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_delete_plans(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
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
    * Update the navigation property plans in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/planner/plans/{plannerPlan-id}` endpoint.
    */
    pub async fn users_planner_update_plans(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        body: &crate::types::MicrosoftGraphPlannerPlanAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerPlanAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
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
    * List buckets.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets` endpoint.
    *
    * Retrieve a list of plannerBucket objects contained by a plannerPlan object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerplan-list-buckets?view=graph-rest-1.0>
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
    pub async fn users_planner_plans_list_bucket(
        &self,
        user_id: &str,
        planner_plan_id: &str,
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
                "/users/{}/planner/plans/{}/buckets?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create new navigation property to buckets for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets` endpoint.
    */
    pub async fn users_planner_plans_create_buckets(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        body: &crate::types::MicrosoftGraphPlannerBucketAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerBucketAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/buckets",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
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
    * Get buckets from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}` endpoint.
    *
    * Read-only. Nullable. Collection of buckets in the plan.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_plans_get_bucket(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerBucketAllOf> {
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
                "/users/{}/planner/plans/{}/buckets/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property buckets for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_delete_buckets(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/buckets/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
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
    * Update the navigation property buckets in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}` endpoint.
    */
    pub async fn users_planner_plans_update_buckets(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        body: &crate::types::MicrosoftGraphPlannerBucketAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerBucketAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/buckets/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
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
    * List tasks.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks` endpoint.
    *
    * Retrieve a list of plannerTask objects associated to a plannerBucket object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerbucket-list-tasks?view=graph-rest-1.0>
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
    pub async fn users_planner_plans_buckets_list_task(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
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
                "/users/{}/planner/plans/{}/buckets/{}/tasks?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create new navigation property to tasks for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks` endpoint.
    */
    pub async fn users_planner_plans_buckets_create_tasks(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        body: &crate::types::MicrosoftGraphPlannerTaskAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerTaskAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/buckets/{}/tasks",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks/{plannerTask-id}` endpoint.
    *
    * Read-only. Nullable. The collection of tasks in the bucket.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_plans_buckets_get_task(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        planner_task_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerTaskAllOf> {
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
                "/users/{}/planner/plans/{}/buckets/{}/tasks/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `DELETE` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks/{plannerTask-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_buckets_delete_tasks(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        planner_task_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/buckets/{}/tasks/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks/{plannerTask-id}` endpoint.
    */
    pub async fn users_planner_plans_buckets_update_tasks(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        planner_task_id: &str,
        body: &crate::types::MicrosoftGraphPlannerTaskAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerTaskAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/buckets/{}/tasks/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Get plannerAssignedToTaskBoardTaskFormat.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks/{plannerTask-id}/assignedToTaskBoardFormat` endpoint.
    *
    * Retrieve the properties and relationships of a **plannerAssignedToTaskBoardTaskFormat** object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerassignedtotaskboardtaskformat-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_plans_buckets_tasks_get_assigned_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        planner_task_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerAssignedTaskBoardFormatAllOf> {
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
                "/users/{}/planner/plans/{}/buckets/{}/tasks/{}/assignedToTaskBoardFormat?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property assignedToTaskBoardFormat for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks/{plannerTask-id}/assignedToTaskBoardFormat` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_buckets_tasks_delete_assigned_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        planner_task_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/buckets/{}/tasks/{}/assignedToTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Update the navigation property assignedToTaskBoardFormat in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks/{plannerTask-id}/assignedToTaskBoardFormat` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerassignedtotaskboardtaskformat-update?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_buckets_tasks_update_assigned_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        planner_task_id: &str,
        if_match: &str,
        body: &crate::types::MicrosoftGraphPlannerAssignedTaskBoardFormatAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerAssignedTaskBoardFormatAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/buckets/{}/tasks/{}/assignedToTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Get plannerBucketTaskBoardTaskFormat.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks/{plannerTask-id}/bucketTaskBoardFormat` endpoint.
    *
    * Retrieve the properties and relationships of **plannerBucketTaskBoardTaskFormat** object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerbuckettaskboardtaskformat-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_plans_buckets_tasks_get_bucket_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        planner_task_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerBucketTaskBoardFormatAllOf> {
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
                "/users/{}/planner/plans/{}/buckets/{}/tasks/{}/bucketTaskBoardFormat?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property bucketTaskBoardFormat for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks/{plannerTask-id}/bucketTaskBoardFormat` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_buckets_tasks_delete_bucket_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        planner_task_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/buckets/{}/tasks/{}/bucketTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Update the navigation property bucketTaskBoardFormat in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks/{plannerTask-id}/bucketTaskBoardFormat` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerbuckettaskboardtaskformat-update?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_buckets_tasks_update_bucket_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        planner_task_id: &str,
        if_match: &str,
        body: &crate::types::MicrosoftGraphPlannerBucketTaskBoardFormatAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerBucketTaskBoardFormatAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/buckets/{}/tasks/{}/bucketTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Get plannerTaskDetails.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks/{plannerTask-id}/details` endpoint.
    *
    * Retrieve the properties and relationships of a **plannerTaskDetails** object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannertaskdetails-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_plans_buckets_tasks_get_detail(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        planner_task_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerTaskDetailsAllOf> {
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
                "/users/{}/planner/plans/{}/buckets/{}/tasks/{}/details?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property details for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks/{plannerTask-id}/details` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_buckets_tasks_delete_details(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        planner_task_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/buckets/{}/tasks/{}/details",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Update the navigation property details in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks/{plannerTask-id}/details` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannertaskdetails-update?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_buckets_tasks_update_details(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        planner_task_id: &str,
        if_match: &str,
        body: &crate::types::MicrosoftGraphPlannerTaskDetailsAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerTaskDetailsAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/buckets/{}/tasks/{}/details",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Get plannerProgressTaskBoardTaskFormat.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks/{plannerTask-id}/progressTaskBoardFormat` endpoint.
    *
    * Retrieve the properties and relationships of **plannerProgressTaskBoardTaskFormat** object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerprogresstaskboardtaskformat-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_plans_buckets_tasks_get_progress_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        planner_task_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerProgressTaskBoardFormatAllOf> {
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
                "/users/{}/planner/plans/{}/buckets/{}/tasks/{}/progressTaskBoardFormat?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property progressTaskBoardFormat for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks/{plannerTask-id}/progressTaskBoardFormat` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_buckets_tasks_delete_progress_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        planner_task_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/buckets/{}/tasks/{}/progressTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Update the navigation property progressTaskBoardFormat in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks/{plannerTask-id}/progressTaskBoardFormat` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerprogresstaskboardtaskformat-update?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_buckets_tasks_update_progress_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
        planner_task_id: &str,
        if_match: &str,
        body: &crate::types::MicrosoftGraphPlannerProgressTaskBoardFormatAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerProgressTaskBoardFormatAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/buckets/{}/tasks/{}/progressTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/{plannerBucket-id}/tasks/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_buckets_tasks_get_count_de_1b(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_bucket_id: &str,
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
                "/users/{}/planner/plans/{}/buckets/{}/tasks/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_bucket_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/buckets/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_buckets_get_count_f_7f_0(
        &self,
        user_id: &str,
        planner_plan_id: &str,
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
                "/users/{}/planner/plans/{}/buckets/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get plannerPlanDetails.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/details` endpoint.
    *
    * Retrieve the properties and relationships of a **plannerPlanDetails** object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerplandetails-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_plans_get_detail(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerPlanDetailsAllOf> {
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
                "/users/{}/planner/plans/{}/details?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property details for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/details` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_delete_details(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/details",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
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
    * Update the navigation property details in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/details` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerplandetails-update?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_update_details(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        if_match: &str,
        body: &crate::types::MicrosoftGraphPlannerPlanDetailsAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerPlanDetailsAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/details",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
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
    * List tasks.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks` endpoint.
    *
    * Retrieve a list of plannerTask objects associated with a plannerPlan object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerplan-list-tasks?view=graph-rest-1.0>
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
    pub async fn users_planner_plans_list_task(
        &self,
        user_id: &str,
        planner_plan_id: &str,
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
                "/users/{}/planner/plans/{}/tasks?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create new navigation property to tasks for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks` endpoint.
    */
    pub async fn users_planner_plans_create_tasks(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        body: &crate::types::MicrosoftGraphPlannerTaskAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerTaskAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/tasks",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks/{plannerTask-id}` endpoint.
    *
    * Read-only. Nullable. Collection of tasks in the plan.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_plans_get_task(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_task_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerTaskAllOf> {
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
                "/users/{}/planner/plans/{}/tasks/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `DELETE` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks/{plannerTask-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_delete_tasks(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_task_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/tasks/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks/{plannerTask-id}` endpoint.
    */
    pub async fn users_planner_plans_update_tasks(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_task_id: &str,
        body: &crate::types::MicrosoftGraphPlannerTaskAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerTaskAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/tasks/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Get plannerAssignedToTaskBoardTaskFormat.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks/{plannerTask-id}/assignedToTaskBoardFormat` endpoint.
    *
    * Retrieve the properties and relationships of a **plannerAssignedToTaskBoardTaskFormat** object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerassignedtotaskboardtaskformat-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_plans_tasks_get_assigned_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_task_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerAssignedTaskBoardFormatAllOf> {
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
                "/users/{}/planner/plans/{}/tasks/{}/assignedToTaskBoardFormat?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property assignedToTaskBoardFormat for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks/{plannerTask-id}/assignedToTaskBoardFormat` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_tasks_delete_assigned_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_task_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/tasks/{}/assignedToTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Update the navigation property assignedToTaskBoardFormat in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks/{plannerTask-id}/assignedToTaskBoardFormat` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerassignedtotaskboardtaskformat-update?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_tasks_update_assigned_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_task_id: &str,
        if_match: &str,
        body: &crate::types::MicrosoftGraphPlannerAssignedTaskBoardFormatAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerAssignedTaskBoardFormatAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/tasks/{}/assignedToTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Get plannerBucketTaskBoardTaskFormat.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks/{plannerTask-id}/bucketTaskBoardFormat` endpoint.
    *
    * Retrieve the properties and relationships of **plannerBucketTaskBoardTaskFormat** object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerbuckettaskboardtaskformat-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_plans_tasks_get_bucket_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_task_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerBucketTaskBoardFormatAllOf> {
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
                "/users/{}/planner/plans/{}/tasks/{}/bucketTaskBoardFormat?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property bucketTaskBoardFormat for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks/{plannerTask-id}/bucketTaskBoardFormat` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_tasks_delete_bucket_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_task_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/tasks/{}/bucketTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Update the navigation property bucketTaskBoardFormat in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks/{plannerTask-id}/bucketTaskBoardFormat` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerbuckettaskboardtaskformat-update?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_tasks_update_bucket_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_task_id: &str,
        if_match: &str,
        body: &crate::types::MicrosoftGraphPlannerBucketTaskBoardFormatAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerBucketTaskBoardFormatAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/tasks/{}/bucketTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Get plannerTaskDetails.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks/{plannerTask-id}/details` endpoint.
    *
    * Retrieve the properties and relationships of a **plannerTaskDetails** object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannertaskdetails-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_plans_tasks_get_detail(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_task_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerTaskDetailsAllOf> {
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
                "/users/{}/planner/plans/{}/tasks/{}/details?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property details for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks/{plannerTask-id}/details` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_tasks_delete_details(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_task_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/tasks/{}/details",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Update the navigation property details in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks/{plannerTask-id}/details` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannertaskdetails-update?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_tasks_update_details(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_task_id: &str,
        if_match: &str,
        body: &crate::types::MicrosoftGraphPlannerTaskDetailsAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerTaskDetailsAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/tasks/{}/details",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Get plannerProgressTaskBoardTaskFormat.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks/{plannerTask-id}/progressTaskBoardFormat` endpoint.
    *
    * Retrieve the properties and relationships of **plannerProgressTaskBoardTaskFormat** object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerprogresstaskboardtaskformat-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_plans_tasks_get_progress_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_task_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerProgressTaskBoardFormatAllOf> {
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
                "/users/{}/planner/plans/{}/tasks/{}/progressTaskBoardFormat?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property progressTaskBoardFormat for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks/{plannerTask-id}/progressTaskBoardFormat` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_tasks_delete_progress_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_task_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/tasks/{}/progressTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Update the navigation property progressTaskBoardFormat in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks/{plannerTask-id}/progressTaskBoardFormat` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerprogresstaskboardtaskformat-update?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_tasks_update_progress_task_board_format(
        &self,
        user_id: &str,
        planner_plan_id: &str,
        planner_task_id: &str,
        if_match: &str,
        body: &crate::types::MicrosoftGraphPlannerProgressTaskBoardFormatAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerProgressTaskBoardFormatAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/plans/{}/tasks/{}/progressTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/{plannerPlan-id}/tasks/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_tasks_get_count_08_2b(
        &self,
        user_id: &str,
        planner_plan_id: &str,
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
                "/users/{}/planner/plans/{}/tasks/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_plan_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `GET` to the `/users/{user-id}/planner/plans/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_plans_get_count_b_724(
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
                "/users/{}/planner/plans/$count?{}",
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
    * List tasks.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/tasks` endpoint.
    *
    * Retrieve a list of **plannertask** objects assigned to a User.
    *
    * FROM: <https://docs.microsoft.com/graph/api/planneruser-list-tasks?view=graph-rest-1.0>
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
    pub async fn users_planner_list_task(
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
                "/users/{}/planner/tasks?{}",
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
    * Create new navigation property to tasks for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/planner/tasks` endpoint.
    */
    pub async fn users_planner_create_tasks(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphPlannerTaskAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerTaskAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/tasks",
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
    * Get tasks from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/tasks/{plannerTask-id}` endpoint.
    *
    * Read-only. Nullable. Returns the plannerPlans shared with the user.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_get_task(
        &self,
        user_id: &str,
        planner_task_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerTaskAllOf> {
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
                "/users/{}/planner/tasks/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `DELETE` to the `/users/{user-id}/planner/tasks/{plannerTask-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_delete_tasks(
        &self,
        user_id: &str,
        planner_task_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/tasks/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/planner/tasks/{plannerTask-id}` endpoint.
    */
    pub async fn users_planner_update_tasks(
        &self,
        user_id: &str,
        planner_task_id: &str,
        body: &crate::types::MicrosoftGraphPlannerTaskAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerTaskAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/tasks/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Get plannerAssignedToTaskBoardTaskFormat.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/tasks/{plannerTask-id}/assignedToTaskBoardFormat` endpoint.
    *
    * Retrieve the properties and relationships of a **plannerAssignedToTaskBoardTaskFormat** object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerassignedtotaskboardtaskformat-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_tasks_get_assigned_task_board_format(
        &self,
        user_id: &str,
        planner_task_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerAssignedTaskBoardFormatAllOf> {
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
                "/users/{}/planner/tasks/{}/assignedToTaskBoardFormat?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property assignedToTaskBoardFormat for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/planner/tasks/{plannerTask-id}/assignedToTaskBoardFormat` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_tasks_delete_assigned_task_board_format(
        &self,
        user_id: &str,
        planner_task_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/tasks/{}/assignedToTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Update the navigation property assignedToTaskBoardFormat in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/planner/tasks/{plannerTask-id}/assignedToTaskBoardFormat` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerassignedtotaskboardtaskformat-update?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_tasks_update_assigned_task_board_format(
        &self,
        user_id: &str,
        planner_task_id: &str,
        if_match: &str,
        body: &crate::types::MicrosoftGraphPlannerAssignedTaskBoardFormatAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerAssignedTaskBoardFormatAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/tasks/{}/assignedToTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Get plannerBucketTaskBoardTaskFormat.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/tasks/{plannerTask-id}/bucketTaskBoardFormat` endpoint.
    *
    * Retrieve the properties and relationships of **plannerBucketTaskBoardTaskFormat** object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerbuckettaskboardtaskformat-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_tasks_get_bucket_task_board_format(
        &self,
        user_id: &str,
        planner_task_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerBucketTaskBoardFormatAllOf> {
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
                "/users/{}/planner/tasks/{}/bucketTaskBoardFormat?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property bucketTaskBoardFormat for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/planner/tasks/{plannerTask-id}/bucketTaskBoardFormat` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_tasks_delete_bucket_task_board_format(
        &self,
        user_id: &str,
        planner_task_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/tasks/{}/bucketTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Update the navigation property bucketTaskBoardFormat in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/planner/tasks/{plannerTask-id}/bucketTaskBoardFormat` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerbuckettaskboardtaskformat-update?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_tasks_update_bucket_task_board_format(
        &self,
        user_id: &str,
        planner_task_id: &str,
        if_match: &str,
        body: &crate::types::MicrosoftGraphPlannerBucketTaskBoardFormatAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerBucketTaskBoardFormatAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/tasks/{}/bucketTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Get plannerTaskDetails.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/tasks/{plannerTask-id}/details` endpoint.
    *
    * Retrieve the properties and relationships of a **plannerTaskDetails** object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannertaskdetails-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_tasks_get_detail(
        &self,
        user_id: &str,
        planner_task_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerTaskDetailsAllOf> {
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
                "/users/{}/planner/tasks/{}/details?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property details for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/planner/tasks/{plannerTask-id}/details` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_tasks_delete_details(
        &self,
        user_id: &str,
        planner_task_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/tasks/{}/details",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Update the navigation property details in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/planner/tasks/{plannerTask-id}/details` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannertaskdetails-update?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_tasks_update_details(
        &self,
        user_id: &str,
        planner_task_id: &str,
        if_match: &str,
        body: &crate::types::MicrosoftGraphPlannerTaskDetailsAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerTaskDetailsAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/tasks/{}/details",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Get plannerProgressTaskBoardTaskFormat.
    *
    * This function performs a `GET` to the `/users/{user-id}/planner/tasks/{plannerTask-id}/progressTaskBoardFormat` endpoint.
    *
    * Retrieve the properties and relationships of **plannerProgressTaskBoardTaskFormat** object.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerprogresstaskboardtaskformat-get?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_planner_tasks_get_progress_task_board_format(
        &self,
        user_id: &str,
        planner_task_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerProgressTaskBoardFormatAllOf> {
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
                "/users/{}/planner/tasks/{}/progressTaskBoardFormat?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property progressTaskBoardFormat for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/planner/tasks/{plannerTask-id}/progressTaskBoardFormat` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_tasks_delete_progress_task_board_format(
        &self,
        user_id: &str,
        planner_task_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/tasks/{}/progressTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * Update the navigation property progressTaskBoardFormat in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/planner/tasks/{plannerTask-id}/progressTaskBoardFormat` endpoint.
    *
    * FROM: <https://docs.microsoft.com/graph/api/plannerprogresstaskboardtaskformat-update?view=graph-rest-1.0>
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_tasks_update_progress_task_board_format(
        &self,
        user_id: &str,
        planner_task_id: &str,
        if_match: &str,
        body: &crate::types::MicrosoftGraphPlannerProgressTaskBoardFormatAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphPlannerProgressTaskBoardFormatAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/planner/tasks/{}/progressTaskBoardFormat",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&planner_task_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/planner/tasks/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_planner_tasks_get_count_8740(
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
                "/users/{}/planner/tasks/$count?{}",
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
