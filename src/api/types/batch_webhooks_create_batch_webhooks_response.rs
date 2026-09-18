pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBatchWebhooksResponse {
    #[serde(flatten)]
    pub batch_webhook_fields: BatchWebhook,
    /// The HMAC signing secret. Returned exactly once at creation. This should be stored securely; if lost, delete and recreate the webhook to obtain a new secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_secret: Option<String>,
}

impl CreateBatchWebhooksResponse {
    pub fn builder() -> CreateBatchWebhooksResponseBuilder {
        <CreateBatchWebhooksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBatchWebhooksResponseBuilder {
    batch_webhook_fields: Option<BatchWebhook>,
    signing_secret: Option<String>,
}

impl CreateBatchWebhooksResponseBuilder {
    pub fn batch_webhook_fields(mut self, value: BatchWebhook) -> Self {
        self.batch_webhook_fields = Some(value);
        self
    }

    pub fn signing_secret(mut self, value: impl Into<String>) -> Self {
        self.signing_secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBatchWebhooksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`batch_webhook_fields`](CreateBatchWebhooksResponseBuilder::batch_webhook_fields)
    pub fn build(self) -> Result<CreateBatchWebhooksResponse, BuildError> {
        Ok(CreateBatchWebhooksResponse {
            batch_webhook_fields: self
                .batch_webhook_fields
                .ok_or_else(|| BuildError::missing_field("batch_webhook_fields"))?,
            signing_secret: self.signing_secret,
        })
    }
}
