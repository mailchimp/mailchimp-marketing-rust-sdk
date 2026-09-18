//! API client and types for the Mailchimp API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints
//! - [`types`] - Request, response, and model types

pub mod resources;
pub mod types;

pub use resources::{
    AccountExportsClient, ActivityFeedClient, AudiencesClient, AuthorizedAppsClient,
    AutomationsClient, BatchWebhooksClient, BatchesClient, CampaignFoldersClient, CampaignsClient,
    ConnectedSitesClient, ConversationsClient, CustomerJourneysClient, EcommerceClient,
    FacebookAdsClient, FileManagerClient, LandingPagesClient, ListsClient, MailchimpClient,
    PingClient, ReportingClient, ReportsClient, RootClient, SearchCampaignsClient,
    SearchMembersClient, SmsCampaignsClient, SurveysClient, TemplateFoldersClient, TemplatesClient,
    VerifiedDomainsClient,
};
pub use types::*;
