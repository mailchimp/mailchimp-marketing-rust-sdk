use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct FacebookAdsClient {
    pub http_client: HttpClient,
}

impl FacebookAdsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get list of Facebook ads.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `sort_field` - Returns files sorted by the specified field.
    /// * `sort_dir` - Determines the order direction for sorted results.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_marketing::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpClient::new(config).expect("Failed to build client");
    ///     client
    ///         .facebook_ads
    ///         .list(
    ///             &FacebookAdsListQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 sort_field: None,
    ///                 sort_dir: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &FacebookAdsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListFacebookAdsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/facebook-ads",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Get details of a Facebook ad.
    ///
    /// # Arguments
    ///
    /// * `outreach_id` - The outreach id.
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mailchimp_marketing::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = MailchimpClient::new(config).expect("Failed to build client");
    ///     client
    ///         .facebook_ads
    ///         .get(
    ///             &"outreach_id".to_string(),
    ///             &FacebookAdsGetQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        outreach_id: &str,
        request: &FacebookAdsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<FacebookAds, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/facebook-ads/{}", outreach_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }
}
