use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TemplateFoldersClient {
    pub http_client: HttpClient,
}

impl TemplateFoldersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get all folders used to organize templates.
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
    ///         .template_folders
    ///         .list(
    ///             &TemplateFoldersListQueryRequest {
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
        request: &TemplateFoldersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListTemplateFoldersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/template-folders",
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

    /// Create a new template folder.
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
    ///         .template_folders
    ///         .create(
    ///             &CreateTemplateFoldersRequest {
    ///                 name: "name".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateTemplateFoldersRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateTemplateFoldersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "3.0/template-folders",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific folder used to organize templates.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The unique id for the template folder.
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
    ///         .template_folders
    ///         .get(
    ///             &"folder_id".to_string(),
    ///             &TemplateFoldersGetQueryRequest {
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
        folder_id: &str,
        request: &TemplateFoldersGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetTemplateFoldersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/template-folders/{}", folder_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a specific template folder, and mark all the templates in the folder as 'unfiled'.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The unique id for the template folder.
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
    ///         .template_folders
    ///         .delete(&"folder_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        folder_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/template-folders/{}", folder_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a specific folder used to organize templates.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The unique id for the template folder.
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
    ///         .template_folders
    ///         .update(
    ///             &"folder_id".to_string(),
    ///             &UpdateTemplateFoldersRequest {
    ///                 name: "name".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        folder_id: &str,
        request: &UpdateTemplateFoldersRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateTemplateFoldersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/template-folders/{}", folder_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
