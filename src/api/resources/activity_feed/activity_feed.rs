use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ActivityFeedClient {
    pub http_client: HttpClient,
}

impl ActivityFeedClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get information about the activity feed endpoint's resources.
    ///
    /// # Arguments
    ///
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
    ///     client.activity_feed.list(None).await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ListActivityFeedResponseItem>, ApiError> {
        self.http_client
            .execute_request(Method::GET, "3.0/activity-feed", None, None, options)
            .await
    }

    /// Return the Chimp Chatter for this account ordered by most recent.
    ///
    /// # Arguments
    ///
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
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
    ///         .activity_feed
    ///         .list_chimp_chatter(
    ///             &ListChimpChatterQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_chimp_chatter(
        &self,
        request: &ListChimpChatterQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListChimpChatterActivityFeedResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/activity-feed/chimp-chatter",
                None,
                QueryBuilder::new()
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }
}
