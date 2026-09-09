use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TemplatesClient {
    pub http_client: HttpClient,
}

impl TemplatesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Get a list of an account's available templates.
    ///
    /// # Arguments
    ///
    /// * `fields` - A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    /// * `exclude_fields` - A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    /// * `count` - The number of records to return. Default value is 10. Maximum value is 1000
    /// * `offset` - Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    /// * `created_by` - The Mailchimp account user who created the template.
    /// * `since_date_created` - Restrict the response to templates created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `before_date_created` - Restrict the response to templates created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    /// * `type_` - Limit results based on template type.
    /// * `category` - Limit results based on category.
    /// * `folder_id` - The unique folder id.
    /// * `sort_field` - Returns user templates sorted by the specified field.
    /// * `content_type` - Limit results based on how the template's content is put together. Only templates of type `user` can be filtered by `content_type`. If you want to retrieve saved templates created with the legacy email editor, then filter `content_type` to `template`. If you'd rather pull your saved templates for the new editor, filter to `multichannel`. For code your own templates, filter to `html`.
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
    ///         .templates
    ///         .list(
    ///             &TemplatesListQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///                 count: None,
    ///                 offset: None,
    ///                 created_by: None,
    ///                 since_date_created: None,
    ///                 before_date_created: None,
    ///                 r#type: None,
    ///                 category: None,
    ///                 folder_id: None,
    ///                 sort_field: None,
    ///                 content_type: None,
    ///                 sort_dir: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &TemplatesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListTemplatesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "3.0/templates",
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .int("count", request.count.clone())
                    .int("offset", request.offset.clone())
                    .string("created_by", request.created_by.clone())
                    .string("since_date_created", request.since_date_created.clone())
                    .string("before_date_created", request.before_date_created.clone())
                    .string("type", request.r#type.clone())
                    .string("category", request.category.clone())
                    .string("folder_id", request.folder_id.clone())
                    .serialize("sort_field", request.sort_field.clone())
                    .serialize("content_type", request.content_type.clone())
                    .serialize("sort_dir", request.sort_dir.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new template for the account. Only Classic templates are supported.
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
    ///         .templates
    ///         .create(
    ///             &CreateTemplatesRequest {
    ///                 html: "html".to_string(),
    ///                 name: "Freddie's Jokes".to_string(),
    ///                 folder_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateTemplatesRequest,
        options: Option<RequestOptions>,
    ) -> Result<TemplateInstance, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "3.0/templates",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get information about a specific template.
    ///
    /// # Arguments
    ///
    /// * `template_id` - The unique id for the template.
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
    ///         .templates
    ///         .get(
    ///             &"template_id".to_string(),
    ///             &TemplatesGetQueryRequest {
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
        template_id: &str,
        request: &TemplatesGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TemplateInstance, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/templates/{}", template_id),
                None,
                QueryBuilder::new()
                    .string_array("fields", request.fields.clone())
                    .string_array("exclude_fields", request.exclude_fields.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a specific template.
    ///
    /// # Arguments
    ///
    /// * `template_id` - The unique id for the template.
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
    ///         .templates
    ///         .delete(&"template_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        template_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("3.0/templates/{}", template_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update the name, HTML, or `folder_id` of an existing template.
    ///
    /// # Arguments
    ///
    /// * `template_id` - The unique id for the template.
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
    ///         .templates
    ///         .update(
    ///             &"template_id".to_string(),
    ///             &UpdateTemplatesRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        template_id: &str,
        request: &UpdateTemplatesRequest,
        options: Option<RequestOptions>,
    ) -> Result<TemplateInstance, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("3.0/templates/{}", template_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get the sections that you can edit in a template, including each section's default content.
    ///
    /// # Arguments
    ///
    /// * `template_id` - The unique id for the template.
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
    ///         .templates
    ///         .list_default_content(
    ///             &"template_id".to_string(),
    ///             &ListDefaultContentQueryRequest {
    ///                 fields: vec![],
    ///                 exclude_fields: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_default_content(
        &self,
        template_id: &str,
        request: &ListDefaultContentQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDefaultContentTemplatesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("3.0/templates/{}/default-content", template_id),
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
