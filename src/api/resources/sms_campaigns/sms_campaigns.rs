use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SmsCampaignsClient {
    pub http_client: HttpClient,
}

impl SmsCampaignsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get all SMS campaigns in an account.
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
    ///         .sms_campaigns
    ///         .list(
    ///             &SmsCampaignsListQueryRequest {
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
        request: &SmsCampaignsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSmsCampaignsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/sms-campaigns",
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

    /// Create a new SMS campaign.
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
    ///         .sms_campaigns
    ///         .create(
    ///             &CreateSmsCampaignsRequest {
    ///                 name: "name".to_string(),
    ///                 list_id: None,
    ///                 folder_id: None,
    ///                 segments: None,
    ///                 excluded_segments: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateSmsCampaignsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SmsCampaign, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "3.0/sms-campaigns",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get the details for a single SMS campaign.
    ///
    /// # Arguments
    ///
    /// * `sms_campaign_id` - The unique id for the SMS campaign.
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
    ///         .sms_campaigns
    ///         .get(
    ///             &"sms_campaign_id".to_string(),
    ///             &SmsCampaignsGetQueryRequest {
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
        sms_campaign_id: &str,
        request: &SmsCampaignsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SmsCampaign, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/sms-campaigns/{}", sms_campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Remove a campaign from your Mailchimp account.
    ///
    /// # Arguments
    ///
    /// * `sms_campaign_id` - The unique id for the SMS campaign.
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
    ///         .sms_campaigns
    ///         .delete(&"sms_campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        sms_campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/sms-campaigns/{}", sms_campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update an SMS campaign.
    ///
    /// # Arguments
    ///
    /// * `sms_campaign_id` - The unique id for the SMS campaign.
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
    ///         .sms_campaigns
    ///         .update(
    ///             &"sms_campaign_id".to_string(),
    ///             &UpdateSmsCampaignsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        sms_campaign_id: &str,
        request: &UpdateSmsCampaignsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SmsCampaign, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/sms-campaigns/{}", sms_campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cancel a scheduled or sending SMS campaign.
    ///
    /// # Arguments
    ///
    /// * `sms_campaign_id` - The unique id for the SMS campaign.
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
    ///         .sms_campaigns
    ///         .create_action_cancel_send(&"sms_campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_cancel_send(
        &self,
        sms_campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/sms-campaigns/{}/actions/cancel-send", sms_campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Schedule an SMS campaign for delivery.
    ///
    /// # Arguments
    ///
    /// * `sms_campaign_id` - The unique id for the SMS campaign.
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
    ///         .sms_campaigns
    ///         .create_action_schedule(
    ///             &"sms_campaign_id".to_string(),
    ///             &CreateActionScheduleSmsCampaignsRequest {
    ///                 schedule_time: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_schedule(
        &self,
        sms_campaign_id: &str,
        request: &CreateActionScheduleSmsCampaignsRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/sms-campaigns/{}/actions/schedule", sms_campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Send an SMS campaign.
    ///
    /// # Arguments
    ///
    /// * `sms_campaign_id` - The unique id for the SMS campaign.
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
    ///         .sms_campaigns
    ///         .create_action_send(&"sms_campaign_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn create_action_send(
        &self,
        sms_campaign_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("3.0/sms-campaigns/{}/actions/send", sms_campaign_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get the content for an SMS campaign.
    ///
    /// # Arguments
    ///
    /// * `sms_campaign_id` - The unique id for the SMS campaign.
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
    ///         .sms_campaigns
    ///         .get_content(
    ///             &"sms_campaign_id".to_string(),
    ///             &SmsCampaignsGetContentQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_content(
        &self,
        sms_campaign_id: &str,
        request: &SmsCampaignsGetContentQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SmsCampaignContent, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/sms-campaigns/{}/content", sms_campaign_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Set the content for an SMS campaign.
    ///
    /// # Arguments
    ///
    /// * `sms_campaign_id` - The unique id for the SMS campaign.
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
    ///         .sms_campaigns
    ///         .upsert_content(
    ///             &"sms_campaign_id".to_string(),
    ///             &UpsertContentSmsCampaignsRequest {
    ///                 message_body: "message_body".to_string(),
    ///                 media: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upsert_content(
        &self,
        sms_campaign_id: &str,
        request: &UpsertContentSmsCampaignsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SmsCampaignContent, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("3.0/sms-campaigns/{}/content", sms_campaign_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
