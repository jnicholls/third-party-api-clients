use crate::Client;
use crate::ClientResult;

pub struct UsersContact {
    pub client: Client,
}

impl UsersContact {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersContact { client }
    }

    /**
    * List contacts.
    *
    * This function performs a `GET` to the `/users/{user-id}/contacts` endpoint.
    *
    * Get a contact collection from the default contacts folder of the signed-in user. There are two scenarios where an app can get contacts in another user's contact folder:
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-list-contacts?view=graph-rest-1.0>
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
    pub async fn users_list_contact(
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
                "/users/{}/contacts?{}",
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
    * Create contact.
    *
    * This function performs a `POST` to the `/users/{user-id}/contacts` endpoint.
    *
    * Add a contact to the root Contacts folder or to the contacts endpoint of another contact folder.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-post-contacts?view=graph-rest-1.0>
    */
    pub async fn users_create_contacts(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphContactAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphContactAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contacts",
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
    * Get contacts from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/contacts/{contact-id}` endpoint.
    *
    * The user's contacts. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_get_contact(
        &self,
        user_id: &str,
        contact_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphContactAllOf> {
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
                "/users/{}/contacts/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property contacts for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/contacts/{contact-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_delete_contacts(
        &self,
        user_id: &str,
        contact_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contacts/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
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
    * Update the navigation property contacts in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/contacts/{contact-id}` endpoint.
    */
    pub async fn users_update_contacts(
        &self,
        user_id: &str,
        contact_id: &str,
        body: &crate::types::MicrosoftGraphContactAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphContactAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contacts/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contacts/{contact-id}/extensions` endpoint.
    *
    * The collection of open extensions defined for the contact. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `top: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `skip: u64` -- Indicates how long this provisioning action took to finish. Measured in milliseconds.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    * * `count: bool` -- Indicates if a sign-in is interactive or not.
    * * `orderby: &[String]` -- Order items by property values.
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_list_extension(
        &self,
        user_id: &str,
        contact_id: &str,
        top: u64,
        skip: u64,
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
                "/users/{}/contacts/{}/extensions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `POST` to the `/users/{user-id}/contacts/{contact-id}/extensions` endpoint.
    */
    pub async fn s_create_extensions(
        &self,
        user_id: &str,
        contact_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contacts/{}/extensions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contacts/{contact-id}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the contact. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_extension(
        &self,
        user_id: &str,
        contact_id: &str,
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
                "/users/{}/contacts/{}/extensions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/contacts/{contact-id}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_extensions(
        &self,
        user_id: &str,
        contact_id: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contacts/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/contacts/{contact-id}/extensions/{extension-id}` endpoint.
    */
    pub async fn s_update_extensions(
        &self,
        user_id: &str,
        contact_id: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contacts/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contacts/{contact-id}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_extensions_get_count_e_5d_6(
        &self,
        user_id: &str,
        contact_id: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/contacts/{}/extensions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get multiValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/contacts/{contact-id}/multiValueExtendedProperties` endpoint.
    *
    * The collection of multi-value extended properties defined for the contact. Read-only. Nullable.
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
    pub async fn s_list_multi_value_extended_propertie(
        &self,
        user_id: &str,
        contact_id: &str,
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
                "/users/{}/contacts/{}/multiValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create new navigation property to multiValueExtendedProperties for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/contacts/{contact-id}/multiValueExtendedProperties` endpoint.
    */
    pub async fn s_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        contact_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contacts/{}/multiValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
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
    * Get multiValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/contacts/{contact-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the contact. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        contact_id: &str,
        multi_value_legacy_extended_property_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
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
                "/users/{}/contacts/{}/multiValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
                crate::progenitor_support::encode_path(
                    &multi_value_legacy_extended_property_id.to_string()
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
    * Delete navigation property multiValueExtendedProperties for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/contacts/{contact-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        contact_id: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contacts/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
                crate::progenitor_support::encode_path(
                    &multi_value_legacy_extended_property_id.to_string()
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
    * Update the navigation property multiValueExtendedProperties in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/contacts/{contact-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        contact_id: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contacts/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
                crate::progenitor_support::encode_path(
                    &multi_value_legacy_extended_property_id.to_string()
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
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/contacts/{contact-id}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_multi_value_extended_properties_get_count_b_8ff(
        &self,
        user_id: &str,
        contact_id: &str,
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
                "/users/{}/contacts/{}/multiValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `GET` to the `/users/{user-id}/contacts/{contact-id}/photo` endpoint.
    *
    * Optional contact picture. You can get or set a photo for a contact.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_get_photo(
        &self,
        user_id: &str,
        contact_id: &str,
        select: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphProfilePhotoAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !select.is_empty() {
            query_args.push(("$select".to_string(), select.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/contacts/{}/photo?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `PATCH` to the `/users/{user-id}/contacts/{contact-id}/photo` endpoint.
    */
    pub async fn s_update_photo(
        &self,
        user_id: &str,
        contact_id: &str,
        body: &crate::types::MicrosoftGraphProfilePhotoAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphProfilePhotoAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contacts/{}/photo",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contacts/{contact-id}/photo/$value` endpoint.
    */
    pub async fn s_get_photo_content(&self, user_id: &str, contact_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contacts/{}/photo/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `PUT` to the `/users/{user-id}/contacts/{contact-id}/photo/$value` endpoint.
    */
    pub async fn s_update_photo_content<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        contact_id: &str,
        body: B,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contacts/{}/photo/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
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
    * Get singleValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/contacts/{contact-id}/singleValueExtendedProperties` endpoint.
    *
    * The collection of single-value extended properties defined for the contact. Read-only. Nullable.
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
    pub async fn s_list_single_value_extended_propertie(
        &self,
        user_id: &str,
        contact_id: &str,
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
                "/users/{}/contacts/{}/singleValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create new navigation property to singleValueExtendedProperties for users.
    *
    * This function performs a `POST` to the `/users/{user-id}/contacts/{contact-id}/singleValueExtendedProperties` endpoint.
    */
    pub async fn s_create_single_value_extended_properties(
        &self,
        user_id: &str,
        contact_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contacts/{}/singleValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
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
    * Get singleValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/contacts/{contact-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the contact. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        contact_id: &str,
        single_value_legacy_extended_property_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
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
                "/users/{}/contacts/{}/singleValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
                crate::progenitor_support::encode_path(
                    &single_value_legacy_extended_property_id.to_string()
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
    * Delete navigation property singleValueExtendedProperties for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/contacts/{contact-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        contact_id: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contacts/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
                crate::progenitor_support::encode_path(
                    &single_value_legacy_extended_property_id.to_string()
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
    * Update the navigation property singleValueExtendedProperties in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/contacts/{contact-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_update_single_value_extended_properties(
        &self,
        user_id: &str,
        contact_id: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contacts/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
                crate::progenitor_support::encode_path(
                    &single_value_legacy_extended_property_id.to_string()
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
    * Get the number of the resource.
    *
    * This function performs a `GET` to the `/users/{user-id}/contacts/{contact-id}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_single_value_extended_properties_get_count_5fe_7(
        &self,
        user_id: &str,
        contact_id: &str,
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
                "/users/{}/contacts/{}/singleValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `GET` to the `/users/{user-id}/contacts/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_get_count_4943(
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
                "/users/{}/contacts/$count?{}",
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
