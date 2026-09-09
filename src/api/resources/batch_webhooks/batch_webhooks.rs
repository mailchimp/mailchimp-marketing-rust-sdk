use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct BatchWebhooksClient {
    pub http_client: HttpClient,
}

impl BatchWebhooksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get all webhooks that have been configured for batches.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
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
    ///         .batch_webhooks
    ///         .list(
    ///             &BatchWebhooksListQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &BatchWebhooksListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListBatchWebhooksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/batch-webhooks",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Configure a webhook that will fire whenever any batch request completes processing.  You may only have a maximum of 20 batch webhooks.
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
    ///     client
    ///         .batch_webhooks
    ///         .create(
    ///             &CreateBatchWebhooksRequest {
    ///                 url: "http://yourdomain.com/webhook".to_string(),
    ///                 enabled: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateBatchWebhooksRequest,
        options: Option<RequestOptions>,
    ) -> Result<BatchWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "3.0/batch-webhooks",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific batch webhook.
    ///
    /// # Arguments
    ///
    /// * `batch_webhook_id` - The unique id for the batch webhook.
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
    ///         .batch_webhooks
    ///         .get(
    ///             &"batch_webhook_id".to_string(),
    ///             &BatchWebhooksGetQueryRequest {
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
        batch_webhook_id: &str,
        request: &BatchWebhooksGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BatchWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/batch-webhooks/{}", batch_webhook_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Remove a batch webhook. Webhooks will no longer be sent to the given URL.
    ///
    /// # Arguments
    ///
    /// * `batch_webhook_id` - The unique id for the batch webhook.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .batch_webhooks
    ///         .delete(&"batch_webhook_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        batch_webhook_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/batch-webhooks/{}", batch_webhook_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a webhook that will fire whenever any batch request completes processing.
    ///
    /// # Arguments
    ///
    /// * `batch_webhook_id` - The unique id for the batch webhook.
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
    ///         .batch_webhooks
    ///         .update(
    ///             &"batch_webhook_id".to_string(),
    ///             &UpdateBatchWebhooksRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        batch_webhook_id: &str,
        request: &UpdateBatchWebhooksRequest,
        options: Option<RequestOptions>,
    ) -> Result<BatchWebhook, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/batch-webhooks/{}", batch_webhook_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
