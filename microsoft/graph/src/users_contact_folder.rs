use crate::Client;
use crate::ClientResult;

pub struct UsersContactFolder {
    pub client: Client,
}

impl UsersContactFolder {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersContactFolder { client }
    }

    /**
    * List contactFolders.
    *
    * This function performs a `GET` to the `/users/{user-id}/contactFolders` endpoint.
    *
    * Get the contact folder collection in the default Contacts folder of the signed-in user.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-list-contactfolders?view=graph-rest-1.0>
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
    pub async fn users_list_contact_folder(
        &self,
        user_id: &str,
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
                "/users/{}/contactFolders?{}",
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
    * Create ContactFolder.
    *
    * This function performs a `POST` to the `/users/{user-id}/contactFolders` endpoint.
    *
    * Create a new contactFolder under the user's default contacts folder. You can also create a new contactfolder as a child of any specified contact folder.
    *
    * FROM: <https://docs.microsoft.com/graph/api/user-post-contactfolders?view=graph-rest-1.0>
    */
    pub async fn users_create_contact_folders(
        &self,
        user_id: &str,
        body: &crate::types::MicrosoftGraphContactFolderAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphContactFolderAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders",
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
    * Get contactFolders from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}` endpoint.
    *
    * The user's contacts folders. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn users_get_contact_folder(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphContactFolderAllOf> {
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
                "/users/{}/contactFolders/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property contactFolders for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/contactFolders/{contactFolder-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn users_delete_contact_folders(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * Update the navigation property contactFolders in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/contactFolders/{contactFolder-id}` endpoint.
    */
    pub async fn users_update_contact_folders(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        body: &crate::types::MicrosoftGraphContactFolderAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphContactFolderAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * List childFolders.
    *
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders` endpoint.
    *
    * Get a collection of child folders under the specified contact folder.
    *
    * FROM: <https://docs.microsoft.com/graph/api/contactfolder-list-childfolders?view=graph-rest-1.0>
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
    pub async fn s_list_child(
        &self,
        user_id: &str,
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/childFolders?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create ContactFolder.
    *
    * This function performs a `POST` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders` endpoint.
    *
    * Create a new contactFolder as a child of a specified folder.  You can also create a new contactFolder under the user's default contact folder.
    *
    * FROM: <https://docs.microsoft.com/graph/api/contactfolder-post-childfolders?view=graph-rest-1.0>
    */
    pub async fn s_create_child(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        body: &crate::types::MicrosoftGraphContactFolderAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphContactFolderAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * Get childFolders from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}` endpoint.
    *
    * The collection of child folders in the folder. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_child(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        select: &[String],
        expand: &[String],
    ) -> ClientResult<crate::types::MicrosoftGraphContactFolderAllOf> {
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
                "/users/{}/contactFolders/{}/childFolders/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Delete navigation property childFolders for users.
    *
    * This function performs a `DELETE` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_child(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * Update the navigation property childFolders in users.
    *
    * This function performs a `PATCH` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}` endpoint.
    */
    pub async fn s_update_child(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        body: &crate::types::MicrosoftGraphContactFolderAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphContactFolderAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * List contacts.
    *
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts` endpoint.
    *
    * Get a contact collection from the default Contacts folder of the signed-in user (`.../me/contacts`), or from the specified contact folder.
    *
    * FROM: <https://docs.microsoft.com/graph/api/contactfolder-list-contacts?view=graph-rest-1.0>
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
    pub async fn s_child_list_contact(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
                "/users/{}/contactFolders/{}/childFolders/{}/contacts?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `POST` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts` endpoint.
    *
    * Add a contact to the root Contacts folder or to the `contacts` endpoint of another contact folder.
    *
    * FROM: <https://docs.microsoft.com/graph/api/contactfolder-post-contacts?view=graph-rest-1.0>
    */
    pub async fn s_child_create_contacts(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        body: &crate::types::MicrosoftGraphContactAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphContactAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}/contacts",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}` endpoint.
    *
    * The contacts in the folder. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_child_get_contact(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
                "/users/{}/contactFolders/{}/childFolders/{}/contacts/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_child_delete_contacts(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        contact_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}/contacts/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}` endpoint.
    */
    pub async fn s_child_update_contacts(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        contact_id: &str,
        body: &crate::types::MicrosoftGraphContactAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphContactAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}/contacts/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/extensions` endpoint.
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
    pub async fn s_child_contacts_list_extension(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
                "/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/extensions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/extensions` endpoint.
    */
    pub async fn s_child_contacts_create_extensions(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        contact_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/extensions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the contact. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_child_contacts_get_extension(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
                "/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/extensions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_child_contacts_delete_extensions(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        contact_id: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/extensions/{extension-id}` endpoint.
    */
    pub async fn s_child_contacts_update_extensions(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        contact_id: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_child_contacts_extensions_get_count_5b_30(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
                "/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/extensions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/multiValueExtendedProperties` endpoint.
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
    pub async fn s_child_contacts_list_multi_value_extended_propertie(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
&format!("/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/multiValueExtendedProperties?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),crate::progenitor_support::encode_path(&contact_id.to_string()),query_), None);
        self.client
            .get(
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
    * This function performs a `POST` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/multiValueExtendedProperties` endpoint.
    */
    pub async fn s_child_contacts_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        contact_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/multiValueExtendedProperties",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),crate::progenitor_support::encode_path(&contact_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the contact. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_child_contacts_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
&format!("/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/multiValueExtendedProperties/{}?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),crate::progenitor_support::encode_path(&contact_id.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),query_), None);
        self.client
            .get(
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
    * This function performs a `DELETE` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_child_contacts_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        contact_id: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/multiValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),crate::progenitor_support::encode_path(&contact_id.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `PATCH` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_child_contacts_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        contact_id: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/multiValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),crate::progenitor_support::encode_path(&contact_id.to_string()),crate::progenitor_support::encode_path(&multi_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_child_contacts_multi_value_extended_properties_get_count_8515(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
&format!("/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/multiValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),crate::progenitor_support::encode_path(&contact_id.to_string()),query_), None);
        self.client
            .get(
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/photo` endpoint.
    *
    * Optional contact picture. You can get or set a photo for a contact.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_child_contacts_get_photo(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
                "/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/photo?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/photo` endpoint.
    */
    pub async fn s_child_contacts_update_photo(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        contact_id: &str,
        body: &crate::types::MicrosoftGraphProfilePhotoAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphProfilePhotoAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/photo",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/photo/$value` endpoint.
    */
    pub async fn s_child_contacts_get_photo_content(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        contact_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/photo/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `PUT` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/photo/$value` endpoint.
    */
    pub async fn s_child_contacts_update_photo_content<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        contact_id: &str,
        body: B,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/photo/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/singleValueExtendedProperties` endpoint.
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
    pub async fn s_child_contacts_list_single_value_extended_propertie(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
&format!("/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/singleValueExtendedProperties?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),crate::progenitor_support::encode_path(&contact_id.to_string()),query_), None);
        self.client
            .get(
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
    * This function performs a `POST` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/singleValueExtendedProperties` endpoint.
    */
    pub async fn s_child_contacts_create_single_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        contact_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/singleValueExtendedProperties",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),crate::progenitor_support::encode_path(&contact_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the contact. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_child_contacts_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
&format!("/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/singleValueExtendedProperties/{}?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),crate::progenitor_support::encode_path(&contact_id.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),query_), None);
        self.client
            .get(
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
    * This function performs a `DELETE` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_child_contacts_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        contact_id: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
&format!("/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/singleValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),crate::progenitor_support::encode_path(&contact_id.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `PATCH` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_child_contacts_update_single_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        contact_id: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
&format!("/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/singleValueExtendedProperties/{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),crate::progenitor_support::encode_path(&contact_id.to_string()),crate::progenitor_support::encode_path(&single_value_legacy_extended_property_id.to_string()),), None);
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/{contact-id}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_child_contacts_single_value_extended_properties_get_count_2359(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
&format!("/users/{}/contactFolders/{}/childFolders/{}/contacts/{}/singleValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),crate::progenitor_support::encode_path(&contact_id.to_string()),query_), None);
        self.client
            .get(
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/contacts/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_child_contacts_get_count_6cbe(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}/contacts/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/multiValueExtendedProperties` endpoint.
    *
    * The collection of multi-value extended properties defined for the contactFolder. Read-only. Nullable.
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
    pub async fn s_child_list_multi_value_extended_propertie(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
                "/users/{}/contactFolders/{}/childFolders/{}/multiValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `POST` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/multiValueExtendedProperties` endpoint.
    */
    pub async fn s_child_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}/multiValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the contactFolder. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_child_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
                "/users/{}/contactFolders/{}/childFolders/{}/multiValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_child_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_child_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_child_multi_value_extended_properties_get_count_55_4c(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
&format!("/users/{}/contactFolders/{}/childFolders/{}/multiValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),query_), None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get singleValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/singleValueExtendedProperties` endpoint.
    *
    * The collection of single-value extended properties defined for the contactFolder. Read-only. Nullable.
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
    pub async fn s_child_list_single_value_extended_propertie(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
                "/users/{}/contactFolders/{}/childFolders/{}/singleValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `POST` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/singleValueExtendedProperties` endpoint.
    */
    pub async fn s_child_create_single_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}/singleValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the contactFolder. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_child_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
                "/users/{}/contactFolders/{}/childFolders/{}/singleValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_child_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_child_update_single_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/{contactFolder-id1}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_child_single_value_extended_properties_get_count_0e_56(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_folder_id_1: &str,
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
&format!("/users/{}/contactFolders/{}/childFolders/{}/singleValueExtendedProperties/$count?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id.to_string()),crate::progenitor_support::encode_path(&contact_folder_id_1.to_string()),query_), None);
        self.client
            .get(
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/childFolders/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_child_get_count_9149(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/childFolders/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * List contacts.
    *
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts` endpoint.
    *
    * Get a contact collection from the default Contacts folder of the signed-in user (`.../me/contacts`), or from the specified contact folder.
    *
    * FROM: <https://docs.microsoft.com/graph/api/contactfolder-list-contacts?view=graph-rest-1.0>
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
    pub async fn s_list_contact(
        &self,
        user_id: &str,
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/contacts?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `POST` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts` endpoint.
    *
    * Add a contact to the root Contacts folder or to the `contacts` endpoint of another contact folder.
    *
    * FROM: <https://docs.microsoft.com/graph/api/contactfolder-post-contacts?view=graph-rest-1.0>
    */
    pub async fn s_create_contacts(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        body: &crate::types::MicrosoftGraphContactAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphContactAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/contacts",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}` endpoint.
    *
    * The contacts in the folder. Navigation property. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_contact(
        &self,
        user_id: &str,
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/contacts/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_contacts(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/contacts/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}` endpoint.
    */
    pub async fn s_update_contacts(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_id: &str,
        body: &crate::types::MicrosoftGraphContactAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphContactAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/contacts/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/extensions` endpoint.
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
    pub async fn s_contacts_list_extension(
        &self,
        user_id: &str,
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/contacts/{}/extensions?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/extensions` endpoint.
    */
    pub async fn s_contacts_create_extensions(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/contacts/{}/extensions",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/extensions/{extension-id}` endpoint.
    *
    * The collection of open extensions defined for the contact. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_contacts_get_extension(
        &self,
        user_id: &str,
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/contacts/{}/extensions/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/extensions/{extension-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_contacts_delete_extensions(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_id: &str,
        extension_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/contacts/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/extensions/{extension-id}` endpoint.
    */
    pub async fn s_contacts_update_extensions(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_id: &str,
        extension_id: &str,
        body: &crate::types::MicrosoftGraphExtensionAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphExtensionAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/contacts/{}/extensions/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/extensions/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_contacts_extensions_get_count_8e_14(
        &self,
        user_id: &str,
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/contacts/{}/extensions/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/multiValueExtendedProperties` endpoint.
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
    pub async fn s_contacts_list_multi_value_extended_propertie(
        &self,
        user_id: &str,
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/contacts/{}/multiValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/multiValueExtendedProperties` endpoint.
    */
    pub async fn s_contacts_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/contacts/{}/multiValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the contact. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_contacts_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/contacts/{}/multiValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_contacts_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_id: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/contacts/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_contacts_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_id: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/contacts/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_contacts_multi_value_extended_properties_get_count_3ab_0(
        &self,
        user_id: &str,
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/contacts/{}/multiValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/photo` endpoint.
    *
    * Optional contact picture. You can get or set a photo for a contact.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    */
    pub async fn s_contacts_get_photo(
        &self,
        user_id: &str,
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/contacts/{}/photo?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/photo` endpoint.
    */
    pub async fn s_contacts_update_photo(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_id: &str,
        body: &crate::types::MicrosoftGraphProfilePhotoAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphProfilePhotoAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/contacts/{}/photo",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/photo/$value` endpoint.
    */
    pub async fn s_contacts_get_photo_content(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/contacts/{}/photo/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `PUT` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/photo/$value` endpoint.
    */
    pub async fn s_contacts_update_photo_content<B: Into<reqwest::Body>>(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_id: &str,
        body: B,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/contacts/{}/photo/$value",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/singleValueExtendedProperties` endpoint.
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
    pub async fn s_contacts_list_single_value_extended_propertie(
        &self,
        user_id: &str,
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/contacts/{}/singleValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `POST` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/singleValueExtendedProperties` endpoint.
    */
    pub async fn s_contacts_create_single_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/contacts/{}/singleValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the contact. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_contacts_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/contacts/{}/singleValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_contacts_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_id: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/contacts/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_contacts_update_single_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        contact_id: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/contacts/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/{contact-id}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_contacts_single_value_extended_properties_get_count_8723(
        &self,
        user_id: &str,
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/contacts/{}/singleValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/contacts/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_contacts_get_count_5cc_4(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        filter: &str,
    ) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/contacts/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/multiValueExtendedProperties` endpoint.
    *
    * The collection of multi-value extended properties defined for the contactFolder. Read-only. Nullable.
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
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/multiValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `POST` to the `/users/{user-id}/contactFolders/{contactFolder-id}/multiValueExtendedProperties` endpoint.
    */
    pub async fn s_create_multi_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/multiValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of multi-value extended properties defined for the contactFolder. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_multi_value_extended_propertie(
        &self,
        user_id: &str,
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/multiValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/contactFolders/{contactFolder-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_multi_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        multi_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/contactFolders/{contactFolder-id}/multiValueExtendedProperties/{multiValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_update_multi_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        multi_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphMultiValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/multiValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/multiValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_multi_value_extended_properties_get_count_cdaf(
        &self,
        user_id: &str,
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/multiValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Get singleValueExtendedProperties from users.
    *
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/singleValueExtendedProperties` endpoint.
    *
    * The collection of single-value extended properties defined for the contactFolder. Read-only. Nullable.
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
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/singleValueExtendedProperties?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `POST` to the `/users/{user-id}/contactFolders/{contactFolder-id}/singleValueExtendedProperties` endpoint.
    */
    pub async fn s_create_single_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/singleValueExtendedProperties",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * The collection of single-value extended properties defined for the contactFolder. Read-only. Nullable.
    *
    * **Parameters:**
    *
    * * `select: &[String]` -- Select properties to be returned.
    * * `expand: &[String]` -- Expand related entities.
    */
    pub async fn s_get_single_value_extended_propertie(
        &self,
        user_id: &str,
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/singleValueExtendedProperties/{}?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `DELETE` to the `/users/{user-id}/contactFolders/{contactFolder-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    *
    * **Parameters:**
    *
    * * `if_match: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_delete_single_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        single_value_legacy_extended_property_id: &str,
        if_match: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `PATCH` to the `/users/{user-id}/contactFolders/{contactFolder-id}/singleValueExtendedProperties/{singleValueLegacyExtendedProperty-id}` endpoint.
    */
    pub async fn s_update_single_value_extended_properties(
        &self,
        user_id: &str,
        contact_folder_id: &str,
        single_value_legacy_extended_property_id: &str,
        body: &crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf,
    ) -> ClientResult<crate::types::MicrosoftGraphSingleValueLegacyExtendedPropertyAllOf> {
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/{}/singleValueExtendedProperties/{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/{contactFolder-id}/singleValueExtendedProperties/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `search: &str` -- The unique idenfier for an entity. Read-only.
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_single_value_extended_properties_get_count_e_4df(
        &self,
        user_id: &str,
        contact_folder_id: &str,
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
                "/users/{}/contactFolders/{}/singleValueExtendedProperties/$count?{}",
                crate::progenitor_support::encode_path(&user_id.to_string()),
                crate::progenitor_support::encode_path(&contact_folder_id.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
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
    * This function performs a `GET` to the `/users/{user-id}/contactFolders/$count` endpoint.
    *
    * **Parameters:**
    *
    * * `filter: &str` -- The unique idenfier for an entity. Read-only.
    */
    pub async fn s_get_count_7_2bb(&self, user_id: &str, filter: &str) -> ClientResult<i64> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("$filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/contactFolders/$count?{}",
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
