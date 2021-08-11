use anyhow::Result;

use crate::Client;

pub struct Locations {
    client: Client,
}

impl Locations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Locations { client }
    }

    /**
     * List locations.
     *
     * This function performs a `GET` to the `/locations` endpoint.
     *
     * Retrieves all locations for your business.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     * * `start: &str` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     */
    pub async fn get_locations(
        &self,
        start: &str,
        page_size: f64,
    ) -> Result<Vec<crate::types::Location>> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("page_size={}", page_size));
        if !start.is_empty() {
            query_args.push(format!("start={}", start));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/locations?{}", query);

        let resp: crate::types::GetLocationResponse = self.client.get(&url, None).await.unwrap();

        // Return our response data.
        Ok(resp.data)
    }

    /**
     * Create new location.
     *
     * This function performs a `POST` to the `/locations` endpoint.
     *
     * Creates a new location for the business.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn post_location(
        &self,
        body: &crate::types::PostLocationRequest,
    ) -> Result<crate::types::Location> {
        let url = "/locations".to_string();
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * GET a location.
     *
     * This function performs a `GET` to the `/locations/<id>` endpoint.
     *
     * Retrieve a specific location.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn get_location(&self) -> Result<crate::types::Location> {
        let url = "/locations/<id>".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Update location.
     *
     * This function performs a `PATCH` to the `/locations/<id>` endpoint.
     *
     * Modifies a specific location.
     */
    pub async fn patch_location(
        &self,
        body: &crate::types::PatchLocationRequest,
    ) -> Result<crate::types::Location> {
        let url = "/locations/<id>".to_string();
        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
