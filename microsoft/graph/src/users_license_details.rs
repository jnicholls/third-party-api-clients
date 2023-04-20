use crate::Client;
use crate::ClientResult;

pub struct UsersLicenseDetails {
    pub client: Client,
}

impl UsersLicenseDetails {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersLicenseDetails { client }
    }

    /**
    * List licenseDetails.
    *
    * This function performs a `GET` to the `/users/{user-id}/licenseDetails` endpoint.
    *
    * Retrieve a list of licenseDetails objects for enterprise users. This API returns details for licenses that are directly assigned and those transitively assigned through memberships in licensed groups.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-list-licensedetails?view=graph-rest-1.0>
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
    pub async fn users_list_license_detail(
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
                "/users/{}/licenseDetails?{}",
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
    * Create new navigation property to licenseDetails for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/licenseDetails` endpoint.
    */
    pub async fn users_create_license_details(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphLicenseDetailsAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphLicenseDetailsAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/licenseDetails",
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
    * Get licenseDetails from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/licenseDetails/{licenseDetails-id}` endpoint.
    *
    * A collection of this user's license details. Read-only.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_get_license_detail(
        &self,
        user_id: &str,
        license_details_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphLicenseDetailsAllOf> {
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
                "/users/{}/licenseDetails/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&license_details_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property licenseDetails for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/licenseDetails/{licenseDetails-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_delete_license_details(
        &self,
        user_id: &str,
        license_details_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/licenseDetails/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&license_details_id.to_string()),
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
    * Update the navigation property licenseDetails in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/licenseDetails/{licenseDetails-id}` endpoint.
    */
    pub async fn users_update_license_details(
        &self,
        user_id: &str,
        license_details_id: &str,
        body: &crate::types::MicrosoftGraphLicenseDetailsAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphLicenseDetailsAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/licenseDetails/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&license_details_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/licenseDetails/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn get_count_8_3d(
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
                "/users/{}/licenseDetails/$count?{}",
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
