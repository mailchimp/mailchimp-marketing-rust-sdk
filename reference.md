# Reference
## root
<details><summary><code>client.root.<a href="/src/api/resources/root/client.rs">list</a>() -> Result&lt;ListRootResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get links to all other resources available in the API.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .root
        .list(
            &RootListQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## AccountExports
<details><summary><code>client.account_exports.<a href="/src/api/resources/account_exports/client.rs">list</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListAccountExportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of account exports for a given account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .account_exports
        .list(
            &AccountExportsListQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account_exports.<a href="/src/api/resources/account_exports/client.rs">create</a>(request: CreateAccountExportsRequest) -> Result&lt;CreateAccountExportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new account export in your Mailchimp account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .account_exports
        .create(
            &CreateAccountExportsRequest {
                include_stages: vec![
                    CreateAccountExportsRequestIncludeStagesItem::Audiences,
                    CreateAccountExportsRequestIncludeStagesItem::GalleryFiles,
                ],
                since_timestamp: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**include_stages:** `Vec<CreateAccountExportsRequestIncludeStagesItem>` — The stages of an account export to include.
    
</dd>
</dl>

<dl>
<dd>

**since_timestamp:** `Option<String>` — An ISO 8601 date that will limit the export to only records created after a given time. For instance, the reports stage will contain any campaign sent after the given timestamp. Audiences, however, are excluded from this limit.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.account_exports.<a href="/src/api/resources/account_exports/client.rs">get</a>(export_id: String) -> Result&lt;GetAccountExportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific account export.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .account_exports
        .get(
            &"export_id".to_string(),
            &AccountExportsGetQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**export_id:** `String` — The unique id for the account export.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ActivityFeed
<details><summary><code>client.activity_feed.<a href="/src/api/resources/activity_feed/client.rs">list</a>() -> Result&lt;Vec&lt;ListActivityFeedResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about the activity feed endpoint's resources.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client.activity_feed.list(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.activity_feed.<a href="/src/api/resources/activity_feed/client.rs">list_chimp_chatter</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListChimpChatterActivityFeedResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Return the Chimp Chatter for this account ordered by most recent.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .activity_feed
        .list_chimp_chatter(
            &ListChimpChatterQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## AuthorizedApps
<details><summary><code>client.authorized_apps.<a href="/src/api/resources/authorized_apps/client.rs">list</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListAuthorizedAppsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of an account's registered, connected applications.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .authorized_apps
        .list(
            &AuthorizedAppsListQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.authorized_apps.<a href="/src/api/resources/authorized_apps/client.rs">get</a>(app_id: String) -> Result&lt;GetAuthorizedAppsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific authorized application.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .authorized_apps
        .get(
            &"app_id".to_string(),
            &AuthorizedAppsGetQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**app_id:** `String` — The unique id for the connected authorized application.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## automations
<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">list</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, before_create_time: Option&lt;Option&lt;String&gt;&gt;, since_create_time: Option&lt;Option&lt;String&gt;&gt;, before_start_time: Option&lt;Option&lt;String&gt;&gt;, since_start_time: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;ListAutomationsRequestStatus&gt;&gt;) -> Result&lt;ListAutomationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a summary of an account's classic automations.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .list(
            &AutomationsListQueryRequest {
                count: None,
                offset: None,
                fields: vec![],
                exclude_fields: vec![],
                before_create_time: None,
                since_create_time: None,
                before_start_time: None,
                since_start_time: None,
                status: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**before_create_time:** `Option<String>` — Restrict the response to automations created before this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**since_create_time:** `Option<String>` — Restrict the response to automations created after this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**before_start_time:** `Option<String>` — Restrict the response to automations started before this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**since_start_time:** `Option<String>` — Restrict the response to automations started after this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListAutomationsRequestStatus>` — Restrict the results to automations with the specified status.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">create</a>(request: CreateAutomationsRequest) -> Result&lt;AutomationWorkflow, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new classic automation in your Mailchimp account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .create(
            &CreateAutomationsRequest {
                recipients: CreateAutomationsRequestRecipients {
                    ..Default::default()
                },
                trigger_settings: CreateAutomationsRequestTriggerSettings {
                    workflow_type:
                        CreateAutomationsRequestTriggerSettingsWorkflowType::AbandonedBrowse,
                },
                settings: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**recipients:** `CreateAutomationsRequestRecipients` — List settings for the Automation.
    
</dd>
</dl>

<dl>
<dd>

**settings:** `Option<CreateAutomationsRequestSettings>` — The settings for the Automation workflow.
    
</dd>
</dl>

<dl>
<dd>

**trigger_settings:** `CreateAutomationsRequestTriggerSettings` — Trigger settings for the Automation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">get</a>(workflow_id: String) -> Result&lt;AutomationWorkflow, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a summary of an individual classic automation workflow's settings and content. The `trigger_settings` object returns information for the first email in the workflow.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .get(
            &"workflow_id".to_string(),
            &AutomationsGetQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**workflow_id:** `String` — The unique id for the Automation workflow.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">create_action_archive</a>(workflow_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Archiving will permanently end your automation and keep the report data. You’ll be able to replicate your archived automation, but you can’t restart it.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .create_action_archive(&"workflow_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**workflow_id:** `String` — The unique id for the Automation workflow.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">create_action_pause_all_email</a>(workflow_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Pause all emails in a specific classic automation workflow.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .create_action_pause_all_email(&"workflow_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**workflow_id:** `String` — The unique id for the Automation workflow.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">create_action_start_all_email</a>(workflow_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Start all emails in a classic automation workflow.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .create_action_start_all_email(&"workflow_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**workflow_id:** `String` — The unique id for the Automation workflow.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">list_emails</a>(workflow_id: String) -> Result&lt;ListEmailsAutomationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a summary of the emails in a classic automation workflow.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .list_emails(&"workflow_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**workflow_id:** `String` — The unique id for the Automation workflow.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">get_email</a>(workflow_id: String, workflow_email_id: String) -> Result&lt;AutomationWorkflowEmail, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about an individual classic automation workflow email.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .get_email(
            &"workflow_id".to_string(),
            &"workflow_email_id".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**workflow_id:** `String` — The unique id for the Automation workflow.
    
</dd>
</dl>

<dl>
<dd>

**workflow_email_id:** `String` — The unique id for the Automation workflow email.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">delete_email</a>(workflow_id: String, workflow_email_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Removes an individual classic automation workflow email. Emails from certain workflow types, including the Abandoned Cart Email (abandonedCart) and Product Retargeting Email (abandonedBrowse) Workflows, cannot be deleted.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .delete_email(
            &"workflow_id".to_string(),
            &"workflow_email_id".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**workflow_id:** `String` — The unique id for the Automation workflow.
    
</dd>
</dl>

<dl>
<dd>

**workflow_email_id:** `String` — The unique id for the Automation workflow email.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">update_email</a>(workflow_id: String, workflow_email_id: String, request: UpdateEmailAutomationsRequest) -> Result&lt;AutomationWorkflowEmail, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update settings for a classic automation workflow email.  Only works with workflows of type: abandonedBrowse, abandonedCart, emailFollowup, or singleWelcome.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .update_email(
            &"workflow_id".to_string(),
            &"workflow_email_id".to_string(),
            &UpdateEmailAutomationsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**workflow_id:** `String` — The unique id for the Automation workflow.
    
</dd>
</dl>

<dl>
<dd>

**workflow_email_id:** `String` — The unique id for the Automation workflow email.
    
</dd>
</dl>

<dl>
<dd>

**delay:** `Option<UpdateEmailAutomationsRequestDelay>` — The delay settings for an automation email.
    
</dd>
</dl>

<dl>
<dd>

**settings:** `Option<UpdateEmailAutomationsRequestSettings>` — Settings for the campaign including the email subject, from name, and from email address.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">create_email_action_pause</a>(workflow_id: String, workflow_email_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Pause an automated email.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .create_email_action_pause(
            &"workflow_id".to_string(),
            &"workflow_email_id".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**workflow_id:** `String` — The unique id for the Automation workflow.
    
</dd>
</dl>

<dl>
<dd>

**workflow_email_id:** `String` — The unique id for the Automation workflow email.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">create_email_action_start</a>(workflow_id: String, workflow_email_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Start an automated email.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .create_email_action_start(
            &"workflow_id".to_string(),
            &"workflow_email_id".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**workflow_id:** `String` — The unique id for the Automation workflow.
    
</dd>
</dl>

<dl>
<dd>

**workflow_email_id:** `String` — The unique id for the Automation workflow email.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">list_email_queue</a>(workflow_id: String, workflow_email_id: String) -> Result&lt;ListEmailQueueAutomationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a classic automation email queue.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .list_email_queue(
            &"workflow_id".to_string(),
            &"workflow_email_id".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**workflow_id:** `String` — The unique id for the Automation workflow.
    
</dd>
</dl>

<dl>
<dd>

**workflow_email_id:** `String` — The unique id for the Automation workflow email.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">create_email_queue</a>(workflow_id: String, workflow_email_id: String, request: CreateEmailQueueAutomationsRequest) -> Result&lt;SubscriberInAutomationQueue, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Manually add a subscriber to a workflow, bypassing the default trigger settings. You can also use this endpoint to trigger a series of automated emails in an API 3.0 workflow type.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .create_email_queue(
            &"workflow_id".to_string(),
            &"workflow_email_id".to_string(),
            &CreateEmailQueueAutomationsRequest {
                email_address: "email_address".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**workflow_id:** `String` — The unique id for the Automation workflow.
    
</dd>
</dl>

<dl>
<dd>

**workflow_email_id:** `String` — The unique id for the Automation workflow email.
    
</dd>
</dl>

<dl>
<dd>

**email_address:** `String` — The list member's email address.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">get_email_queue</a>(workflow_id: String, workflow_email_id: String, subscriber_hash: String) -> Result&lt;SubscriberInAutomationQueue, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific subscriber in a classic automation email queue.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .get_email_queue(
            &"workflow_id".to_string(),
            &"workflow_email_id".to_string(),
            &"subscriber_hash".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**workflow_id:** `String` — The unique id for the Automation workflow.
    
</dd>
</dl>

<dl>
<dd>

**workflow_email_id:** `String` — The unique id for the Automation workflow email.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">list_removed_subscribers</a>(workflow_id: String) -> Result&lt;ListRemovedSubscribersAutomationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about subscribers who were removed from a classic automation workflow.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .list_removed_subscribers(&"workflow_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**workflow_id:** `String` — The unique id for the Automation workflow.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">create_removed_subscriber</a>(workflow_id: String, request: CreateRemovedSubscriberAutomationsRequest) -> Result&lt;SubscriberRemovedFromAutomationWorkflow, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Remove a subscriber from a specific classic automation workflow. You can remove a subscriber at any point in an automation workflow, regardless of how many emails they've been sent from that workflow. Once they're removed, they can never be added back to the same workflow.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .create_removed_subscriber(
            &"workflow_id".to_string(),
            &CreateRemovedSubscriberAutomationsRequest {
                email_address: "email_address".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**workflow_id:** `String` — The unique id for the Automation workflow.
    
</dd>
</dl>

<dl>
<dd>

**email_address:** `String` — The list member's email address.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.automations.<a href="/src/api/resources/automations/client.rs">get_removed_subscriber</a>(workflow_id: String, subscriber_hash: String) -> Result&lt;SubscriberRemovedFromAutomationWorkflow, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific subscriber who was removed from a classic automation workflow.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .automations
        .get_removed_subscriber(
            &"workflow_id".to_string(),
            &"subscriber_hash".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**workflow_id:** `String` — The unique id for the Automation workflow.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## BatchWebhooks
<details><summary><code>client.batch_webhooks.<a href="/src/api/resources/batch_webhooks/client.rs">list</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListBatchWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get all webhooks that have been configured for batches.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .batch_webhooks
        .list(
            &BatchWebhooksListQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.batch_webhooks.<a href="/src/api/resources/batch_webhooks/client.rs">create</a>(request: CreateBatchWebhooksRequest) -> Result&lt;BatchWebhook, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Configure a webhook that will fire whenever any batch request completes processing.  You may only have a maximum of 20 batch webhooks.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .batch_webhooks
        .create(
            &CreateBatchWebhooksRequest {
                url: "http://yourdomain.com/webhook".to_string(),
                enabled: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**enabled:** `Option<bool>` — Whether the webhook receives requests or not.
    
</dd>
</dl>

<dl>
<dd>

**url:** `String` — A valid URL for the Webhook.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.batch_webhooks.<a href="/src/api/resources/batch_webhooks/client.rs">get</a>(batch_webhook_id: String) -> Result&lt;BatchWebhook, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific batch webhook.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .batch_webhooks
        .get(
            &"batch_webhook_id".to_string(),
            &BatchWebhooksGetQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**batch_webhook_id:** `String` — The unique id for the batch webhook.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.batch_webhooks.<a href="/src/api/resources/batch_webhooks/client.rs">delete</a>(batch_webhook_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Remove a batch webhook. Webhooks will no longer be sent to the given URL.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .batch_webhooks
        .delete(&"batch_webhook_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**batch_webhook_id:** `String` — The unique id for the batch webhook.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.batch_webhooks.<a href="/src/api/resources/batch_webhooks/client.rs">update</a>(batch_webhook_id: String, request: UpdateBatchWebhooksRequest) -> Result&lt;BatchWebhook, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a webhook that will fire whenever any batch request completes processing.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .batch_webhooks
        .update(
            &"batch_webhook_id".to_string(),
            &UpdateBatchWebhooksRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**batch_webhook_id:** `String` — The unique id for the batch webhook.
    
</dd>
</dl>

<dl>
<dd>

**enabled:** `Option<bool>` — Whether the webhook receives requests or not.
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — A valid URL for the Webhook.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## batches
<details><summary><code>client.batches.<a href="/src/api/resources/batches/client.rs">list</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListBatchesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a summary of batch requests that have been made.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .batches
        .list(
            &BatchesListQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.batches.<a href="/src/api/resources/batches/client.rs">create</a>(request: CreateBatchesRequest) -> Result&lt;Batch, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Begin processing a batch operations request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .batches
        .create(
            &CreateBatchesRequest {
                operations: vec![CreateBatchesRequestOperationsItem {
                    body: None,
                    headers: None,
                    method: CreateBatchesRequestOperationsItemMethod::Get,
                    operation_id: None,
                    params: None,
                    path: "/lists".to_string(),
                }],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**operations:** `Vec<CreateBatchesRequestOperationsItem>` — An array of objects that describes operations to perform.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.batches.<a href="/src/api/resources/batches/client.rs">get</a>(batch_id: String) -> Result&lt;Batch, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the status of a batch request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .batches
        .get(
            &"batch_id".to_string(),
            &BatchesGetQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**batch_id:** `String` — The unique id for the batch operation.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.batches.<a href="/src/api/resources/batches/client.rs">delete</a>(batch_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Stops a batch request from running. Since only one batch request is run at a time, this can be used to cancel a long running request. The results of any completed operations will not be available after this call.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client.batches.delete(&"batch_id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**batch_id:** `String` — The unique id for the batch operation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## CampaignFolders
<details><summary><code>client.campaign_folders.<a href="/src/api/resources/campaign_folders/client.rs">list</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;CampaignFolders, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get all folders used to organize campaigns.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaign_folders
        .list(
            &CampaignFoldersListQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaign_folders.<a href="/src/api/resources/campaign_folders/client.rs">create</a>(request: CreateCampaignFoldersRequest) -> Result&lt;CampaignFolders, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new campaign folder.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaign_folders
        .create(
            &CreateCampaignFoldersRequest {
                name: "name".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` — Name to associate with the folder.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaign_folders.<a href="/src/api/resources/campaign_folders/client.rs">get</a>(folder_id: String) -> Result&lt;GetCampaignFoldersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific folder used to organize campaigns.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaign_folders
        .get(
            &"folder_id".to_string(),
            &CampaignFoldersGetQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**folder_id:** `String` — The unique id for the campaign folder.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaign_folders.<a href="/src/api/resources/campaign_folders/client.rs">delete</a>(folder_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a specific campaign folder, and mark all the campaigns in the folder as 'unfiled'.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaign_folders
        .delete(&"folder_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**folder_id:** `String` — The unique id for the campaign folder.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaign_folders.<a href="/src/api/resources/campaign_folders/client.rs">update</a>(folder_id: String, request: UpdateCampaignFoldersRequest) -> Result&lt;UpdateCampaignFoldersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a specific folder used to organize campaigns.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaign_folders
        .update(
            &"folder_id".to_string(),
            &UpdateCampaignFoldersRequest {
                name: "name".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**folder_id:** `String` — The unique id for the campaign folder.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — Name to associate with the folder.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## campaigns
<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">list</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, type_: Option&lt;Option&lt;ListCampaignsRequestType&gt;&gt;, status: Option&lt;Option&lt;ListCampaignsRequestStatus&gt;&gt;, before_send_time: Option&lt;Option&lt;String&gt;&gt;, since_send_time: Option&lt;Option&lt;String&gt;&gt;, before_create_time: Option&lt;Option&lt;String&gt;&gt;, since_create_time: Option&lt;Option&lt;String&gt;&gt;, list_id: Option&lt;Option&lt;String&gt;&gt;, folder_id: Option&lt;Option&lt;String&gt;&gt;, member_id: Option&lt;Option&lt;String&gt;&gt;, sort_field: Option&lt;Option&lt;ListCampaignsRequestSortField&gt;&gt;, sort_dir: Option&lt;Option&lt;ListCampaignsRequestSortDir&gt;&gt;, include_resend_shortcut_eligibility: Option&lt;Option&lt;bool&gt;&gt;, include_resend_shortcut_usage: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;ListCampaignsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get all campaigns in an account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .list(
            &CampaignsListQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                r#type: None,
                status: None,
                before_send_time: None,
                since_send_time: None,
                before_create_time: None,
                since_create_time: None,
                list_id: None,
                folder_id: None,
                member_id: None,
                sort_field: None,
                sort_dir: None,
                include_resend_shortcut_eligibility: None,
                include_resend_shortcut_usage: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<ListCampaignsRequestType>` — The campaign type.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListCampaignsRequestStatus>` — The status of the campaign.
    
</dd>
</dl>

<dl>
<dd>

**before_send_time:** `Option<String>` — Restrict the response to campaigns sent before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**since_send_time:** `Option<String>` — Restrict the response to campaigns sent after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**before_create_time:** `Option<String>` — Restrict the response to campaigns created before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**since_create_time:** `Option<String>` — Restrict the response to campaigns created after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**list_id:** `Option<String>` — The unique id for the list.
    
</dd>
</dl>

<dl>
<dd>

**folder_id:** `Option<String>` — The unique folder id.
    
</dd>
</dl>

<dl>
<dd>

**member_id:** `Option<String>` — Retrieve campaigns sent to a particular list member. Member ID is The MD5 hash of the lowercase version of the list member’s email address.
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<ListCampaignsRequestSortField>` — Returns files sorted by the specified field.
    
</dd>
</dl>

<dl>
<dd>

**sort_dir:** `Option<ListCampaignsRequestSortDir>` — Determines the order direction for sorted results.
    
</dd>
</dl>

<dl>
<dd>

**include_resend_shortcut_eligibility:** `Option<bool>` — Return the `resend_shortcut_eligibility` field in the response, which tells you if the campaign is eligible for the various Campaign Resend Shortcuts offered.
    
</dd>
</dl>

<dl>
<dd>

**include_resend_shortcut_usage:** `Option<bool>` — Return the `resend_shortcut_usage` field in the response.  This includes information about campaigns related by a shortcut.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">create</a>(request: CreateCampaignsRequest) -> Result&lt;Campaign, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new Mailchimp campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .create(
            &CreateCampaignsRequest {
                r#type: CreateCampaignsRequestType::Regular,
                content_type: None,
                recipients: None,
                rss_opts: None,
                settings: None,
                social_card: None,
                tracking: None,
                variate_settings: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**content_type:** `Option<CreateCampaignsRequestContentType>` — How the campaign's content is put together. The old drag and drop editor uses 'template' while the new editor uses 'multichannel'. Defaults to template.
    
</dd>
</dl>

<dl>
<dd>

**recipients:** `Option<CreateCampaignsRequestRecipients>` — List settings for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**rss_opts:** `Option<CreateCampaignsRequestRssOpts>` — [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options, specific to an RSS campaign.
    
</dd>
</dl>

<dl>
<dd>

**settings:** `Option<CreateCampaignsRequestSettings>` — The settings for your campaign, including subject, from name, reply-to address, and more.
    
</dd>
</dl>

<dl>
<dd>

**social_card:** `Option<CreateCampaignsRequestSocialCard>` — The preview for the campaign, rendered by social networks like Facebook and Twitter. [Learn more](https://mailchimp.com/help/enable-and-customize-social-cards/).
    
</dd>
</dl>

<dl>
<dd>

**tracking:** `Option<CampaignTrackingOptions>` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `CreateCampaignsRequestType` — There are four types of [campaigns](https://mailchimp.com/help/getting-started-with-campaigns/) you can create in Mailchimp. A/B Split campaigns have been deprecated and variate campaigns should be used instead.
    
</dd>
</dl>

<dl>
<dd>

**variate_settings:** `Option<CreateCampaignsRequestVariateSettings>` — The settings specific to A/B test campaigns.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">get</a>(campaign_id: String, include_resend_shortcut_eligibility: Option&lt;Option&lt;bool&gt;&gt;, include_resend_shortcut_usage: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;Campaign, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .get(
            &"campaign_id".to_string(),
            &CampaignsGetQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                include_resend_shortcut_eligibility: None,
                include_resend_shortcut_usage: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**include_resend_shortcut_eligibility:** `Option<bool>` — Return the `resend_shortcut_eligibility` field in the response, which tells you if the campaign is eligible for the various Campaign Resend Shortcuts offered.
    
</dd>
</dl>

<dl>
<dd>

**include_resend_shortcut_usage:** `Option<bool>` — Return the `resend_shortcut_usage` field in the response.  This includes information about campaigns related by a shortcut.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">delete</a>(campaign_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Remove a campaign from your Mailchimp account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .delete(&"campaign_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">update</a>(campaign_id: String, request: UpdateCampaignsRequest) -> Result&lt;Campaign, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update some or all of the settings for a specific campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .update(
            &"campaign_id".to_string(),
            &UpdateCampaignsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**recipients:** `Option<UpdateCampaignsRequestRecipients>` — List settings for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**rss_opts:** `Option<UpdateCampaignsRequestRssOpts>` — [RSS](https://mailchimp.com/help/share-your-blog-posts-with-mailchimp/) options for a campaign.
    
</dd>
</dl>

<dl>
<dd>

**settings:** `Option<UpdateCampaignsRequestSettings>` — The settings for your campaign, including subject, from name, reply-to address, and more.
    
</dd>
</dl>

<dl>
<dd>

**social_card:** `Option<UpdateCampaignsRequestSocialCard>` — The preview for the campaign, rendered by social networks like Facebook and Twitter. [Learn more](https://mailchimp.com/help/enable-and-customize-social-cards/).
    
</dd>
</dl>

<dl>
<dd>

**tracking:** `Option<CampaignTrackingOptions>` 
    
</dd>
</dl>

<dl>
<dd>

**variate_settings:** `Option<UpdateCampaignsRequestVariateSettings>` — The settings specific to A/B test campaigns.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">create_action_cancel_send</a>(campaign_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cancel a Regular or Plain-Text Campaign after you send, before all of your recipients receive it. This feature is included with Mailchimp Pro.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .create_action_cancel_send(&"campaign_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">create_action_create_resend</a>(campaign_id: String, request: CreateActionCreateResendCampaignsRequest) -> Result&lt;Campaign, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Remove the guesswork for resending a campaign to certain segments. You can use this endpoint as a shortcut to replicate a campaign and resend it to common segments, such as those who didn't open the campaign, or any new subscribers since it was sent.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .create_action_create_resend(
            &"campaign_id".to_string(),
            &CreateActionCreateResendCampaignsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**shortcut_type:** `Option<CreateActionCreateResendCampaignsRequestShortcutType>` — Which campaign resend shortcut to use. Default is `to_non_openers`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">create_action_pause</a>(campaign_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Pause an RSS-Driven campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .create_action_pause(&"campaign_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">create_action_replicate</a>(campaign_id: String) -> Result&lt;Campaign, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Replicate a campaign in saved or send status.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .create_action_replicate(&"campaign_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">create_action_resume</a>(campaign_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Resume an RSS-Driven campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .create_action_resume(&"campaign_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">create_action_schedule</a>(campaign_id: String, request: CreateActionScheduleCampaignsRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Schedule a campaign for delivery. If you're using Multivariate Campaigns to test send times or sending RSS Campaigns, use the send action instead.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .create_action_schedule(
            &"campaign_id".to_string(),
            &CreateActionScheduleCampaignsRequest {
                schedule_time: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
                batch_delivery: None,
                timewarp: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**batch_delivery:** `Option<CreateActionScheduleCampaignsRequestBatchDelivery>` — Choose whether the campaign should use [Batch Delivery](https://mailchimp.com/help/schedule-batch-delivery/). Cannot be set to `true` for campaigns using [Timewarp](https://mailchimp.com/help/use-timewarp/).
    
</dd>
</dl>

<dl>
<dd>

**schedule_time:** `String` — The UTC date and time to schedule the campaign for delivery in ISO 8601 format. Campaigns may only be scheduled to send on the quarter-hour (:00, :15, :30, :45).
    
</dd>
</dl>

<dl>
<dd>

**timewarp:** `Option<bool>` — Choose whether the campaign should use [Timewarp](https://mailchimp.com/help/use-timewarp/) when sending. Campaigns scheduled with Timewarp are localized based on the recipients' time zones. For example, a Timewarp campaign with a `schedule_time` of 13:00 will be sent to each recipient at 1:00pm in their local time. Cannot be set to `true` for campaigns using [Batch Delivery](https://mailchimp.com/help/schedule-batch-delivery/).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">create_action_send</a>(campaign_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send a Mailchimp campaign. For RSS Campaigns, the campaign will send according to its schedule. All other campaigns will send immediately.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .create_action_send(&"campaign_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">create_action_test</a>(campaign_id: String, request: CreateActionTestCampaignsRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send a test email.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .create_action_test(
            &"campaign_id".to_string(),
            &CreateActionTestCampaignsRequest {
                send_type: CreateActionTestCampaignsRequestSendType::HTML,
                test_emails: vec!["test_emails".to_string()],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**send_type:** `CreateActionTestCampaignsRequestSendType` — Choose the type of test email to send.
    
</dd>
</dl>

<dl>
<dd>

**test_emails:** `Vec<String>` — An array of email addresses to send the test email to.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">create_action_unschedule</a>(campaign_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Unschedule a scheduled campaign that hasn't started sending.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .create_action_unschedule(&"campaign_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">get_content</a>(campaign_id: String) -> Result&lt;CampaignContent, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the the HTML and plain-text content for a campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .get_content(
            &"campaign_id".to_string(),
            &CampaignsGetContentQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">upsert_content</a>(campaign_id: String, request: CampaignContent) -> Result&lt;CampaignContent, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Set the content for a campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .upsert_content(
            &"campaign_id".to_string(),
            &CampaignContent {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">list_feedback</a>(campaign_id: String) -> Result&lt;ListFeedbackCampaignsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get team feedback while you're working together on a Mailchimp campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .list_feedback(
            &"campaign_id".to_string(),
            &ListFeedbackQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">create_feedback</a>(campaign_id: String, request: CreateFeedbackCampaignsRequest) -> Result&lt;CreateFeedbackCampaignsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add feedback on a specific campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .create_feedback(
            &"campaign_id".to_string(),
            &CreateFeedbackCampaignsRequest {
                message: "message".to_string(),
                block_id: None,
                is_complete: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**block_id:** `Option<i64>` — The block id for the editable block that the feedback addresses.
    
</dd>
</dl>

<dl>
<dd>

**is_complete:** `Option<bool>` — The status of feedback.
    
</dd>
</dl>

<dl>
<dd>

**message:** `String` — The content of the feedback.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">get_feedback</a>(campaign_id: String, feedback_id: String) -> Result&lt;CampaignFeedback, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a specific feedback message from a campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .get_feedback(
            &"campaign_id".to_string(),
            &"feedback_id".to_string(),
            &GetFeedbackQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**feedback_id:** `String` — The unique id for the feedback message.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">delete_feedback</a>(campaign_id: String, feedback_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Remove a specific feedback message for a campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .delete_feedback(&"campaign_id".to_string(), &"feedback_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**feedback_id:** `String` — The unique id for the feedback message.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">update_feedback</a>(campaign_id: String, feedback_id: String, request: UpdateFeedbackCampaignsRequest) -> Result&lt;CampaignFeedback, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a specific feedback message for a campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .update_feedback(
            &"campaign_id".to_string(),
            &"feedback_id".to_string(),
            &UpdateFeedbackCampaignsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**feedback_id:** `String` — The unique id for the feedback message.
    
</dd>
</dl>

<dl>
<dd>

**block_id:** `Option<i64>` — The block id for the editable block that the feedback addresses.
    
</dd>
</dl>

<dl>
<dd>

**is_complete:** `Option<bool>` — The status of feedback.
    
</dd>
</dl>

<dl>
<dd>

**message:** `Option<String>` — The content of the feedback.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.campaigns.<a href="/src/api/resources/campaigns/client.rs">list_send_checklist</a>(campaign_id: String) -> Result&lt;ListSendChecklistCampaignsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Review the send checklist for a campaign, and resolve any issues before sending.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .campaigns
        .list_send_checklist(
            &"campaign_id".to_string(),
            &ListSendChecklistQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ConnectedSites
<details><summary><code>client.connected_sites.<a href="/src/api/resources/connected_sites/client.rs">list</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListConnectedSitesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get all connected sites in an account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .connected_sites
        .list(
            &ConnectedSitesListQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connected_sites.<a href="/src/api/resources/connected_sites/client.rs">create</a>(request: CreateConnectedSitesRequest) -> Result&lt;ConnectedSite, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new Mailchimp connected site.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .connected_sites
        .create(
            &CreateConnectedSitesRequest {
                domain: "example.com".to_string(),
                foreign_id: "MC001".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain:** `String` — The connected site domain.
    
</dd>
</dl>

<dl>
<dd>

**foreign_id:** `String` — The unique identifier for the site.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connected_sites.<a href="/src/api/resources/connected_sites/client.rs">get</a>(connected_site_id: String) -> Result&lt;ConnectedSite, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific connected site.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .connected_sites
        .get(
            &"connected_site_id".to_string(),
            &ConnectedSitesGetQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**connected_site_id:** `String` — The unique identifier for the site.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connected_sites.<a href="/src/api/resources/connected_sites/client.rs">delete</a>(connected_site_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Remove a connected site from your Mailchimp account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .connected_sites
        .delete(&"connected_site_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**connected_site_id:** `String` — The unique identifier for the site.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.connected_sites.<a href="/src/api/resources/connected_sites/client.rs">create_action_verify_script_installation</a>(connected_site_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Verify that the connected sites script has been installed, either via the script URL or fragment.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .connected_sites
        .create_action_verify_script_installation(&"connected_site_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**connected_site_id:** `String` — The unique identifier for the site.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## conversations
<details><summary><code>client.conversations.<a href="/src/api/resources/conversations/client.rs">list</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, has_unread_messages: Option&lt;Option&lt;ListConversationsRequestHasUnreadMessages&gt;&gt;, list_id: Option&lt;Option&lt;String&gt;&gt;, campaign_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListConversationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of conversations for the account. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .conversations
        .list(
            &ConversationsListQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                has_unread_messages: None,
                list_id: None,
                campaign_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**has_unread_messages:** `Option<ListConversationsRequestHasUnreadMessages>` — Whether the conversation has any unread messages.
    
</dd>
</dl>

<dl>
<dd>

**list_id:** `Option<String>` — The unique id for the list.
    
</dd>
</dl>

<dl>
<dd>

**campaign_id:** `Option<String>` — The unique id for the campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.conversations.<a href="/src/api/resources/conversations/client.rs">get</a>(conversation_id: String) -> Result&lt;Conversation, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get details about an individual conversation. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .conversations
        .get(
            &"conversation_id".to_string(),
            &ConversationsGetQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**conversation_id:** `String` — The unique id for the conversation.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.conversations.<a href="/src/api/resources/conversations/client.rs">list_messages</a>(conversation_id: String, is_read: Option&lt;Option&lt;ListMessagesConversationsRequestIsRead&gt;&gt;, before_timestamp: Option&lt;Option&lt;String&gt;&gt;, since_timestamp: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListMessagesConversationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get messages from a specific conversation. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .conversations
        .list_messages(
            &"conversation_id".to_string(),
            &ListMessagesQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                is_read: None,
                before_timestamp: None,
                since_timestamp: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**conversation_id:** `String` — The unique id for the conversation.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**is_read:** `Option<ListMessagesConversationsRequestIsRead>` — Whether a conversation message has been marked as read.
    
</dd>
</dl>

<dl>
<dd>

**before_timestamp:** `Option<String>` — Restrict the response to messages created before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**since_timestamp:** `Option<String>` — Restrict the response to messages created after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.conversations.<a href="/src/api/resources/conversations/client.rs">get_message</a>(conversation_id: String, message_id: String) -> Result&lt;ConversationMessage, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get an individual message in a conversation. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .conversations
        .get_message(
            &"conversation_id".to_string(),
            &"message_id".to_string(),
            &GetMessageQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**conversation_id:** `String` — The unique id for the conversation.
    
</dd>
</dl>

<dl>
<dd>

**message_id:** `String` — The unique id for the conversation message.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## CustomerJourneys
<details><summary><code>client.customer_journeys.<a href="/src/api/resources/customer_journeys/client.rs">create_journey_step_action_trigger</a>(journey_id: i64, step_id: i64, request: CreateJourneyStepActionTriggerCustomerJourneysRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

A step trigger in an Automation flow. To use it, create a starting point or step from the Automation flow builder in the app using the Customer Journeys API condition. We’ll provide a url during the process that includes the {journey_id} and {step_id}. You’ll then be able to use this endpoint to trigger the condition for the posted contact.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .customer_journeys
        .create_journey_step_action_trigger(
            1,
            1,
            &CreateJourneyStepActionTriggerCustomerJourneysRequest {
                email_address: "email_address".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**journey_id:** `i64` — The id for the flow.
    
</dd>
</dl>

<dl>
<dd>

**step_id:** `i64` — The id for the Step.
    
</dd>
</dl>

<dl>
<dd>

**email_address:** `String` — The list member's email address.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ecommerce
<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">list</a>() -> Result&lt;ListEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about the e-commerce endpoint's resources.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client.ecommerce.list(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">list_orders</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, campaign_id: Option&lt;Option&lt;String&gt;&gt;, outreach_id: Option&lt;Option&lt;String&gt;&gt;, customer_id: Option&lt;Option&lt;String&gt;&gt;, has_outreach: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;ListOrdersEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about an account's orders.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .list_orders(
            &ListOrdersQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                campaign_id: None,
                outreach_id: None,
                customer_id: None,
                has_outreach: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**campaign_id:** `Option<String>` — Restrict results to orders with a specific `campaign_id` value.
    
</dd>
</dl>

<dl>
<dd>

**outreach_id:** `Option<String>` — Restrict results to orders with a specific `outreach_id` value.
    
</dd>
</dl>

<dl>
<dd>

**customer_id:** `Option<String>` — Restrict results to orders made by a specific customer.
    
</dd>
</dl>

<dl>
<dd>

**has_outreach:** `Option<bool>` — Restrict results to orders that have an outreach attached. For example, an email campaign or Facebook ad.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">list_stores</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListStoresEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about all stores in the account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .list_stores(
            &ListStoresQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_store</a>(request: CreateStoreEcommerceRequest) -> Result&lt;ECommerceStore, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a new store to your Mailchimp account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .create_store(
            &CreateStoreEcommerceRequest {
                currency_code: "USD".to_string(),
                id: "example_store".to_string(),
                list_id: "1a2df69511".to_string(),
                name: "Freddie's Cat Hat Emporium".to_string(),
                address: None,
                domain: None,
                email_address: None,
                is_syncing: None,
                money_format: None,
                phone: None,
                platform: None,
                primary_locale: None,
                timezone: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**address:** `Option<CreateStoreEcommerceRequestAddress>` — The store address.
    
</dd>
</dl>

<dl>
<dd>

**currency_code:** `String` — The three-letter ISO 4217 code for the currency that the store accepts.
    
</dd>
</dl>

<dl>
<dd>

**domain:** `Option<String>` — The store domain. This parameter is required for Connected Sites and Google Ads.
    
</dd>
</dl>

<dl>
<dd>

**email_address:** `Option<String>` — The email address for the store.
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` — The unique identifier for the store.
    
</dd>
</dl>

<dl>
<dd>

**is_syncing:** `Option<bool>` — Whether to disable automations because the store is currently [syncing](https://mailchimp.com/developer/marketing/docs/e-commerce/#pausing-store-automations).
    
</dd>
</dl>

<dl>
<dd>

**list_id:** `String` — The unique identifier for the list associated with the store. The `list_id` for a specific store cannot change.
    
</dd>
</dl>

<dl>
<dd>

**money_format:** `Option<String>` — The currency format for the store. For example: `$`, `£`, etc.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — The name of the store.
    
</dd>
</dl>

<dl>
<dd>

**phone:** `Option<String>` — The store phone number.
    
</dd>
</dl>

<dl>
<dd>

**platform:** `Option<String>` — The e-commerce platform of the store.
    
</dd>
</dl>

<dl>
<dd>

**primary_locale:** `Option<String>` — The primary locale for the store. For example: `en`, `de`, etc.
    
</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<String>` — The timezone for the store.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_store</a>(store_id: String) -> Result&lt;ECommerceStore, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific store.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_store(
            &"store_id".to_string(),
            &GetStoreQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">delete_store</a>(store_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a store. Deleting a store will also delete any associated subresources, including Customers, Orders, Products, and Carts.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .delete_store(&"store_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">update_store</a>(store_id: String, request: UpdateStoreEcommerceRequest) -> Result&lt;ECommerceStore, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a store.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .update_store(
            &"store_id".to_string(),
            &UpdateStoreEcommerceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**address:** `Option<UpdateStoreEcommerceRequestAddress>` — The store address.
    
</dd>
</dl>

<dl>
<dd>

**currency_code:** `Option<String>` — The three-letter ISO 4217 code for the currency that the store accepts.
    
</dd>
</dl>

<dl>
<dd>

**domain:** `Option<String>` — The store domain.
    
</dd>
</dl>

<dl>
<dd>

**email_address:** `Option<String>` — The email address for the store.
    
</dd>
</dl>

<dl>
<dd>

**is_syncing:** `Option<bool>` — Whether to disable automations because the store is currently [syncing](https://mailchimp.com/developer/marketing/docs/e-commerce/#pausing-store-automations).
    
</dd>
</dl>

<dl>
<dd>

**money_format:** `Option<String>` — The currency format for the store. For example: `$`, `£`, etc.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — The name of the store.
    
</dd>
</dl>

<dl>
<dd>

**phone:** `Option<String>` — The store phone number.
    
</dd>
</dl>

<dl>
<dd>

**platform:** `Option<String>` — The e-commerce platform of the store.
    
</dd>
</dl>

<dl>
<dd>

**primary_locale:** `Option<String>` — The primary locale for the store. For example: `en`, `de`, etc.
    
</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<String>` — The timezone for the store.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">list_store_carts</a>(store_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListStoreCartsEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a store's carts.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .list_store_carts(
            &"store_id".to_string(),
            &ListStoreCartsQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_store_cart</a>(store_id: String, request: CreateStoreCartEcommerceRequest) -> Result&lt;ECommerceCart, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a new cart to a store.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .create_store_cart(
            &"store_id".to_string(),
            &CreateStoreCartEcommerceRequest {
                currency_code: "currency_code".to_string(),
                customer: EcommerceStoresCartsPost {
                    id: "id".to_string(),
                    ..Default::default()
                },
                id: CreateStoreCartEcommerceRequestID::String("id".to_string()),
                lines: vec![CreateStoreCartEcommerceRequestLinesItem {
                    id: "id".to_string(),
                    price: CreateStoreCartEcommerceRequestLinesItemPrice::Double(1.1),
                    product_id: "product_id".to_string(),
                    product_variant_id: "product_variant_id".to_string(),
                    quantity: 1,
                }],
                order_total: CreateStoreCartEcommerceRequestOrderTotal::Double(1.1),
                campaign_id: None,
                checkout_url: None,
                tax_total: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**campaign_id:** `Option<String>` — A string that uniquely identifies the campaign for a cart.
    
</dd>
</dl>

<dl>
<dd>

**checkout_url:** `Option<String>` — The URL for the cart. This parameter is required for [Abandoned Cart](https://mailchimp.com/help/create-a-classic-abandoned-cart-email/) automations.
    
</dd>
</dl>

<dl>
<dd>

**currency_code:** `String` — The three-letter ISO 4217 code for the currency that the cart uses.
    
</dd>
</dl>

<dl>
<dd>

**customer:** `EcommerceStoresCartsPost` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `CreateStoreCartEcommerceRequestId` — A unique identifier for the cart.
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Vec<CreateStoreCartEcommerceRequestLinesItem>` — An array of the cart's line items.
    
</dd>
</dl>

<dl>
<dd>

**order_total:** `CreateStoreCartEcommerceRequestOrderTotal` 
    
</dd>
</dl>

<dl>
<dd>

**tax_total:** `Option<CreateStoreCartEcommerceRequestTaxTotal>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_store_cart</a>(store_id: String, cart_id: String) -> Result&lt;ECommerceCart, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific cart.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_store_cart(
            &"store_id".to_string(),
            &"cart_id".to_string(),
            &GetStoreCartQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**cart_id:** `String` — The id for the cart.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">delete_store_cart</a>(store_id: String, cart_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a cart.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .delete_store_cart(&"store_id".to_string(), &"cart_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**cart_id:** `String` — The id for the cart.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">update_store_cart</a>(store_id: String, cart_id: String, request: UpdateStoreCartEcommerceRequest) -> Result&lt;ECommerceCart, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a specific cart.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .update_store_cart(
            &"store_id".to_string(),
            &"cart_id".to_string(),
            &UpdateStoreCartEcommerceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**cart_id:** `String` — The id for the cart.
    
</dd>
</dl>

<dl>
<dd>

**campaign_id:** `Option<String>` — A string that uniquely identifies the campaign associated with a cart.
    
</dd>
</dl>

<dl>
<dd>

**checkout_url:** `Option<String>` — The URL for the cart. This parameter is required for [Abandoned Cart](https://mailchimp.com/help/create-a-classic-abandoned-cart-email/) automations.
    
</dd>
</dl>

<dl>
<dd>

**currency_code:** `Option<String>` — The three-letter ISO 4217 code for the currency that the cart uses.
    
</dd>
</dl>

<dl>
<dd>

**customer:** `Option<EcommerceStoresCartsPatch>` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `Option<UpdateStoreCartEcommerceRequestId>` — A unique identifier for the cart.
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Option<Vec<UpdateStoreCartEcommerceRequestLinesItem>>` — An array of the cart's line items.
    
</dd>
</dl>

<dl>
<dd>

**order_total:** `Option<UpdateStoreCartEcommerceRequestOrderTotal>` 
    
</dd>
</dl>

<dl>
<dd>

**tax_total:** `Option<UpdateStoreCartEcommerceRequestTaxTotal>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">list_store_cart_lines</a>(store_id: String, cart_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListStoreCartLinesEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a cart's line items.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .list_store_cart_lines(
            &"store_id".to_string(),
            &"cart_id".to_string(),
            &ListStoreCartLinesQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**cart_id:** `String` — The id for the cart.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_store_cart_line</a>(store_id: String, cart_id: String, request: CreateStoreCartLineEcommerceRequest) -> Result&lt;ECommerceCartLineItem, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a new line item to an existing cart.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .create_store_cart_line(
            &"store_id".to_string(),
            &"cart_id".to_string(),
            &CreateStoreCartLineEcommerceRequest {
                id: "id".to_string(),
                price: CreateStoreCartLineEcommerceRequestPrice::Double(1.1),
                product_id: "product_id".to_string(),
                product_variant_id: "product_variant_id".to_string(),
                quantity: 1,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**cart_id:** `String` — The id for the cart.
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` — A unique identifier for the cart line item.
    
</dd>
</dl>

<dl>
<dd>

**price:** `CreateStoreCartLineEcommerceRequestPrice` 
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — A unique identifier for the product associated with the cart line item.
    
</dd>
</dl>

<dl>
<dd>

**product_variant_id:** `String` — A unique identifier for the product variant associated with the cart line item.
    
</dd>
</dl>

<dl>
<dd>

**quantity:** `i64` — The quantity of a cart line item.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_store_cart_line</a>(store_id: String, cart_id: String, line_id: String) -> Result&lt;ECommerceCartLineItem, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific cart line item.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_store_cart_line(
            &"store_id".to_string(),
            &"cart_id".to_string(),
            &"line_id".to_string(),
            &GetStoreCartLineQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**cart_id:** `String` — The id for the cart.
    
</dd>
</dl>

<dl>
<dd>

**line_id:** `String` — The id for the line item of a cart.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">delete_store_cart_line</a>(store_id: String, cart_id: String, line_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a specific cart line item.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .delete_store_cart_line(
            &"store_id".to_string(),
            &"cart_id".to_string(),
            &"line_id".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**cart_id:** `String` — The id for the cart.
    
</dd>
</dl>

<dl>
<dd>

**line_id:** `String` — The id for the line item of a cart.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">update_store_cart_line</a>(store_id: String, cart_id: String, line_id: String, request: UpdateStoreCartLineEcommerceRequest) -> Result&lt;ECommerceCartLineItem, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a specific cart line item.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .update_store_cart_line(
            &"store_id".to_string(),
            &"cart_id".to_string(),
            &"line_id".to_string(),
            &UpdateStoreCartLineEcommerceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**cart_id:** `String` — The id for the cart.
    
</dd>
</dl>

<dl>
<dd>

**line_id:** `String` — The id for the line item of a cart.
    
</dd>
</dl>

<dl>
<dd>

**price:** `Option<UpdateStoreCartLineEcommerceRequestPrice>` 
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `Option<String>` — A unique identifier for the product associated with the cart line item.
    
</dd>
</dl>

<dl>
<dd>

**product_variant_id:** `Option<String>` — A unique identifier for the product variant associated with the cart line item.
    
</dd>
</dl>

<dl>
<dd>

**quantity:** `Option<i64>` — The quantity of a cart line item.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">list_store_customers</a>(store_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, email_address: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListStoreCustomersEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a store's customers.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .list_store_customers(
            &"store_id".to_string(),
            &ListStoreCustomersQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                email_address: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**email_address:** `Option<String>` — Restrict the response to customers with the email address.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_store_customer</a>(store_id: String, request: CreateStoreCustomerEcommerceRequest) -> Result&lt;ECommerceCustomer, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a new customer to a store.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .create_store_customer(
            &"store_id".to_string(),
            &CreateStoreCustomerEcommerceRequest {
                id: "id".to_string(),
                opt_in_status: true,
                address: None,
                company: None,
                email_address: None,
                first_name: None,
                last_name: None,
                sms_phone_number: None,
                total_spent: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**address:** `Option<CreateStoreCustomerEcommerceRequestAddress>` — The customer's address.
    
</dd>
</dl>

<dl>
<dd>

**company:** `Option<String>` — The customer's company.
    
</dd>
</dl>

<dl>
<dd>

**email_address:** `Option<String>` — The customer's email address.
    
</dd>
</dl>

<dl>
<dd>

**first_name:** `Option<String>` — The customer's first name.
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` — A unique identifier for the customer. Limited to 50 characters.
    
</dd>
</dl>

<dl>
<dd>

**last_name:** `Option<String>` — The customer's last name.
    
</dd>
</dl>

<dl>
<dd>

**opt_in_status:** `bool` — The customer's opt-in status. This value will never overwrite the opt-in status of a pre-existing Mailchimp list member, but will apply to list members that are added through the e-commerce API endpoints. Customers who don't opt in to your Mailchimp list [will be added as `Transactional` members](https://mailchimp.com/developer/marketing/docs/e-commerce/#customers).
    
</dd>
</dl>

<dl>
<dd>

**sms_phone_number:** `Option<String>` — A US phone number for SMS contact.
    
</dd>
</dl>

<dl>
<dd>

**total_spent:** `Option<CreateStoreCustomerEcommerceRequestTotalSpent>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_store_customer</a>(store_id: String, customer_id: String) -> Result&lt;ECommerceCustomer, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific customer.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_store_customer(
            &"store_id".to_string(),
            &"customer_id".to_string(),
            &GetStoreCustomerQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**customer_id:** `String` — The id for the customer of a store.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">upsert_store_customer</a>(store_id: String, customer_id: String, request: UpsertStoreCustomerEcommerceRequest) -> Result&lt;ECommerceCustomer, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add or update a customer.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .upsert_store_customer(
            &"store_id".to_string(),
            &"customer_id".to_string(),
            &UpsertStoreCustomerEcommerceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**customer_id:** `String` — The id for the customer of a store.
    
</dd>
</dl>

<dl>
<dd>

**address:** `Option<UpsertStoreCustomerEcommerceRequestAddress>` — The customer's address.
    
</dd>
</dl>

<dl>
<dd>

**company:** `Option<String>` — The customer's company.
    
</dd>
</dl>

<dl>
<dd>

**email_address:** `Option<String>` — The customer's email address.
    
</dd>
</dl>

<dl>
<dd>

**first_name:** `Option<String>` — The customer's first name.
    
</dd>
</dl>

<dl>
<dd>

**id:** `Option<String>` — A unique identifier for the customer. Limited to 50 characters.
    
</dd>
</dl>

<dl>
<dd>

**last_name:** `Option<String>` — The customer's last name.
    
</dd>
</dl>

<dl>
<dd>

**opt_in_status:** `Option<bool>` — The customer's opt-in status. This value will never overwrite the opt-in status of a pre-existing Mailchimp list member, but will apply to list members that are added through the e-commerce API endpoints. Customers who don't opt in to your Mailchimp list [will be added as `Transactional` members](https://mailchimp.com/developer/marketing/docs/e-commerce/#customers).
    
</dd>
</dl>

<dl>
<dd>

**sms_phone_number:** `Option<String>` — A US phone number for SMS contact.
    
</dd>
</dl>

<dl>
<dd>

**total_spent:** `Option<UpsertStoreCustomerEcommerceRequestTotalSpent>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">delete_store_customer</a>(store_id: String, customer_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a customer from a store.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .delete_store_customer(&"store_id".to_string(), &"customer_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**customer_id:** `String` — The id for the customer of a store.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">update_store_customer</a>(store_id: String, customer_id: String, request: EcommerceStoresCartsPatch) -> Result&lt;ECommerceCustomer, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a customer.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .update_store_customer(
            &"store_id".to_string(),
            &"customer_id".to_string(),
            &EcommerceStoresCartsPatch {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**customer_id:** `String` — The id for the customer of a store.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">list_store_orders</a>(store_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, customer_id: Option&lt;Option&lt;String&gt;&gt;, has_outreach: Option&lt;Option&lt;bool&gt;&gt;, campaign_id: Option&lt;Option&lt;String&gt;&gt;, outreach_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListStoreOrdersEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a store's orders.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .list_store_orders(
            &"store_id".to_string(),
            &ListStoreOrdersQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                customer_id: None,
                has_outreach: None,
                campaign_id: None,
                outreach_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**customer_id:** `Option<String>` — Restrict results to orders made by a specific customer.
    
</dd>
</dl>

<dl>
<dd>

**has_outreach:** `Option<bool>` — Restrict results to orders that have an outreach attached. For example, an email campaign or Facebook ad.
    
</dd>
</dl>

<dl>
<dd>

**campaign_id:** `Option<String>` — Restrict results to orders with a specific `campaign_id` value.
    
</dd>
</dl>

<dl>
<dd>

**outreach_id:** `Option<String>` — Restrict results to orders with a specific `outreach_id` value.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_store_order</a>(store_id: String, request: CreateStoreOrderEcommerceRequest) -> Result&lt;ECommerceOrder, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a new order to a store.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .create_store_order(
            &"store_id".to_string(),
            &CreateStoreOrderEcommerceRequest {
                currency_code: "currency_code".to_string(),
                customer: EcommerceStoresCartsPost {
                    id: "id".to_string(),
                    ..Default::default()
                },
                id: "id".to_string(),
                lines: vec![CreateStoreOrderEcommerceRequestLinesItem {
                    discount: None,
                    id: "id".to_string(),
                    price: CreateStoreOrderEcommerceRequestLinesItemPrice::Double(1.1),
                    product: None,
                    product_id: "product_id".to_string(),
                    product_variant_id: "product_variant_id".to_string(),
                    quantity: 1,
                }],
                order_total: CreateStoreOrderEcommerceRequestOrderTotal::Double(1.1),
                billing_address: None,
                campaign_id: None,
                cart_id: None,
                cancelled_at_foreign: None,
                discount_total: None,
                financial_status: None,
                fulfillment_status: None,
                landing_site: None,
                order_url: None,
                outreach: None,
                processed_at_foreign: None,
                promos: None,
                shipping_address: None,
                shipping_total: None,
                tax_total: None,
                tracking_carrier: None,
                tracking_code: None,
                tracking_number: None,
                tracking_url: None,
                updated_at_foreign: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**billing_address:** `Option<CreateStoreOrderEcommerceRequestBillingAddress>` — The billing address for the order.
    
</dd>
</dl>

<dl>
<dd>

**campaign_id:** `Option<String>` — A string that uniquely identifies the campaign for an order.
    
</dd>
</dl>

<dl>
<dd>

**cart_id:** `Option<CreateStoreOrderEcommerceRequestCartId>` — A cart id that the order was placed for.
    
</dd>
</dl>

<dl>
<dd>

**cancelled_at_foreign:** `Option<String>` — The date and time the order was cancelled in ISO 8601 format. Note: passing a value for this parameter will cancel the order being created.
    
</dd>
</dl>

<dl>
<dd>

**currency_code:** `String` — The three-letter ISO 4217 code for the currency that the store accepts.
    
</dd>
</dl>

<dl>
<dd>

**customer:** `EcommerceStoresCartsPost` 
    
</dd>
</dl>

<dl>
<dd>

**discount_total:** `Option<CreateStoreOrderEcommerceRequestDiscountTotal>` 
    
</dd>
</dl>

<dl>
<dd>

**financial_status:** `Option<String>` — The order status. Use this parameter to trigger [Order Notifications](https://mailchimp.com/developer/marketing/docs/e-commerce/#order-notifications).
    
</dd>
</dl>

<dl>
<dd>

**fulfillment_status:** `Option<String>` — The fulfillment status for the order. Use this parameter to trigger [Order Notifications](https://mailchimp.com/developer/marketing/docs/e-commerce/#order-notifications).
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` — A unique identifier for the order.
    
</dd>
</dl>

<dl>
<dd>

**landing_site:** `Option<String>` — The URL for the page where the buyer landed when entering the shop.
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Vec<CreateStoreOrderEcommerceRequestLinesItem>` — An array of the order's line items.
    
</dd>
</dl>

<dl>
<dd>

**order_total:** `CreateStoreOrderEcommerceRequestOrderTotal` 
    
</dd>
</dl>

<dl>
<dd>

**order_url:** `Option<String>` — The URL for the order.
    
</dd>
</dl>

<dl>
<dd>

**outreach:** `Option<CreateStoreOrderEcommerceRequestOutreach>` — The outreach associated with this order. For example, an email campaign or Facebook ad.
    
</dd>
</dl>

<dl>
<dd>

**processed_at_foreign:** `Option<String>` — The date and time the order was processed in ISO 8601 format.
    
</dd>
</dl>

<dl>
<dd>

**promos:** `Option<Vec<CreateStoreOrderEcommerceRequestPromosItem>>` — The promo codes applied on the order
    
</dd>
</dl>

<dl>
<dd>

**shipping_address:** `Option<CreateStoreOrderEcommerceRequestShippingAddress>` — The shipping address for the order.
    
</dd>
</dl>

<dl>
<dd>

**shipping_total:** `Option<CreateStoreOrderEcommerceRequestShippingTotal>` 
    
</dd>
</dl>

<dl>
<dd>

**tax_total:** `Option<CreateStoreOrderEcommerceRequestTaxTotal>` 
    
</dd>
</dl>

<dl>
<dd>

**tracking_carrier:** `Option<String>` — The tracking carrier associated with the order.
    
</dd>
</dl>

<dl>
<dd>

**tracking_code:** `Option<CreateStoreOrderEcommerceRequestTrackingCode>` — The Mailchimp tracking code for the order. Uses the 'mc_tc' parameter in E-Commerce tracking URLs.
    
</dd>
</dl>

<dl>
<dd>

**tracking_number:** `Option<String>` — The tracking number associated with the order.
    
</dd>
</dl>

<dl>
<dd>

**tracking_url:** `Option<String>` — The tracking URL associated with the order.
    
</dd>
</dl>

<dl>
<dd>

**updated_at_foreign:** `Option<String>` — The date and time the order was updated in ISO 8601 format.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_store_order</a>(store_id: String, order_id: String) -> Result&lt;ECommerceOrder, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific order.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_store_order(
            &"store_id".to_string(),
            &"order_id".to_string(),
            &GetStoreOrderQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**order_id:** `String` — The id for the order in a store.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">delete_store_order</a>(store_id: String, order_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete an order.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .delete_store_order(&"store_id".to_string(), &"order_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**order_id:** `String` — The id for the order in a store.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">update_store_order</a>(store_id: String, order_id: String, request: UpdateStoreOrderEcommerceRequest) -> Result&lt;ECommerceOrder, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a specific order.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .update_store_order(
            &"store_id".to_string(),
            &"order_id".to_string(),
            &UpdateStoreOrderEcommerceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**order_id:** `String` — The id for the order in a store.
    
</dd>
</dl>

<dl>
<dd>

**billing_address:** `Option<UpdateStoreOrderEcommerceRequestBillingAddress>` — The billing address for the order.
    
</dd>
</dl>

<dl>
<dd>

**campaign_id:** `Option<String>` — A string that uniquely identifies the campaign associated with an order.
    
</dd>
</dl>

<dl>
<dd>

**cart_id:** `Option<UpdateStoreOrderEcommerceRequestCartId>` — A cart id that the order was placed for.
    
</dd>
</dl>

<dl>
<dd>

**cancelled_at_foreign:** `Option<String>` — The date and time the order was cancelled in ISO 8601 format. Note: passing a value for this parameter will cancel the order being edited.
    
</dd>
</dl>

<dl>
<dd>

**currency_code:** `Option<String>` — The three-letter ISO 4217 code for the currency that the store accepts.
    
</dd>
</dl>

<dl>
<dd>

**customer:** `Option<EcommerceStoresCartsPatch>` 
    
</dd>
</dl>

<dl>
<dd>

**discount_total:** `Option<UpdateStoreOrderEcommerceRequestDiscountTotal>` 
    
</dd>
</dl>

<dl>
<dd>

**financial_status:** `Option<String>` — The order status. Use this parameter to trigger [Order Notifications](https://mailchimp.com/developer/marketing/docs/e-commerce/#order-notifications).
    
</dd>
</dl>

<dl>
<dd>

**fulfillment_status:** `Option<String>` — The fulfillment status for the order. Use this parameter to trigger [Order Notifications](https://mailchimp.com/developer/marketing/docs/e-commerce/#order-notifications).
    
</dd>
</dl>

<dl>
<dd>

**id:** `Option<String>` — A unique identifier for the order.
    
</dd>
</dl>

<dl>
<dd>

**landing_site:** `Option<String>` — The URL for the page where the buyer landed when entering the shop.
    
</dd>
</dl>

<dl>
<dd>

**lines:** `Option<Vec<UpdateStoreOrderEcommerceRequestLinesItem>>` — An array of the order's line items.
    
</dd>
</dl>

<dl>
<dd>

**order_total:** `Option<UpdateStoreOrderEcommerceRequestOrderTotal>` 
    
</dd>
</dl>

<dl>
<dd>

**order_url:** `Option<String>` — The URL for the order.
    
</dd>
</dl>

<dl>
<dd>

**outreach:** `Option<UpdateStoreOrderEcommerceRequestOutreach>` — The outreach associated with this order. For example, an email campaign or Facebook ad.
    
</dd>
</dl>

<dl>
<dd>

**processed_at_foreign:** `Option<String>` — The date and time the order was processed in ISO 8601 format.
    
</dd>
</dl>

<dl>
<dd>

**promos:** `Option<Vec<UpdateStoreOrderEcommerceRequestPromosItem>>` — The promo codes applied on the order. Note: Patch will completely replace the value of promos with the new one provided.
    
</dd>
</dl>

<dl>
<dd>

**shipping_address:** `Option<UpdateStoreOrderEcommerceRequestShippingAddress>` — The shipping address for the order.
    
</dd>
</dl>

<dl>
<dd>

**shipping_total:** `Option<UpdateStoreOrderEcommerceRequestShippingTotal>` 
    
</dd>
</dl>

<dl>
<dd>

**tax_total:** `Option<UpdateStoreOrderEcommerceRequestTaxTotal>` 
    
</dd>
</dl>

<dl>
<dd>

**tracking_carrier:** `Option<String>` — The tracking carrier associated with the order.
    
</dd>
</dl>

<dl>
<dd>

**tracking_code:** `Option<UpdateStoreOrderEcommerceRequestTrackingCode>` — The Mailchimp tracking code for the order. Uses the 'mc_tc' parameter in E-Commerce tracking URLs.
    
</dd>
</dl>

<dl>
<dd>

**tracking_number:** `Option<String>` — The tracking number associated with the order.
    
</dd>
</dl>

<dl>
<dd>

**tracking_url:** `Option<String>` — The tracking URL associated with the order.
    
</dd>
</dl>

<dl>
<dd>

**updated_at_foreign:** `Option<String>` — The date and time the order was updated in ISO 8601 format.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">list_store_order_lines</a>(store_id: String, order_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListStoreOrderLinesEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about an order's line items.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .list_store_order_lines(
            &"store_id".to_string(),
            &"order_id".to_string(),
            &ListStoreOrderLinesQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**order_id:** `String` — The id for the order in a store.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_store_order_line</a>(store_id: String, order_id: String, request: CreateStoreOrderLineEcommerceRequest) -> Result&lt;ECommerceOrderLineItem, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a new line item to an existing order.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .create_store_order_line(
            &"store_id".to_string(),
            &"order_id".to_string(),
            &CreateStoreOrderLineEcommerceRequest {
                id: "id".to_string(),
                price: CreateStoreOrderLineEcommerceRequestPrice::Double(1.1),
                product_id: "product_id".to_string(),
                product_variant_id: "product_variant_id".to_string(),
                quantity: 1,
                discount: None,
                product: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**order_id:** `String` — The id for the order in a store.
    
</dd>
</dl>

<dl>
<dd>

**discount:** `Option<CreateStoreOrderLineEcommerceRequestDiscount>` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` — A unique identifier for the order line item.
    
</dd>
</dl>

<dl>
<dd>

**price:** `CreateStoreOrderLineEcommerceRequestPrice` 
    
</dd>
</dl>

<dl>
<dd>

**product:** `Option<EcommerceStoresOrdersPost>` 
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — A unique identifier for the product associated with the order line item.
    
</dd>
</dl>

<dl>
<dd>

**product_variant_id:** `String` — A unique identifier for the product variant associated with the order line item.
    
</dd>
</dl>

<dl>
<dd>

**quantity:** `i64` — The quantity of an order line item.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_store_order_line</a>(store_id: String, order_id: String, line_id: String) -> Result&lt;ECommerceOrderLineItem, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific order line item.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_store_order_line(
            &"store_id".to_string(),
            &"order_id".to_string(),
            &"line_id".to_string(),
            &GetStoreOrderLineQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**order_id:** `String` — The id for the order in a store.
    
</dd>
</dl>

<dl>
<dd>

**line_id:** `String` — The id for the line item of an order.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">delete_store_order_line</a>(store_id: String, order_id: String, line_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a specific order line item.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .delete_store_order_line(
            &"store_id".to_string(),
            &"order_id".to_string(),
            &"line_id".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**order_id:** `String` — The id for the order in a store.
    
</dd>
</dl>

<dl>
<dd>

**line_id:** `String` — The id for the line item of an order.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">update_store_order_line</a>(store_id: String, order_id: String, line_id: String, request: UpdateStoreOrderLineEcommerceRequest) -> Result&lt;ECommerceOrderLineItem, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a specific order line item.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .update_store_order_line(
            &"store_id".to_string(),
            &"order_id".to_string(),
            &"line_id".to_string(),
            &UpdateStoreOrderLineEcommerceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**order_id:** `String` — The id for the order in a store.
    
</dd>
</dl>

<dl>
<dd>

**line_id:** `String` — The id for the line item of an order.
    
</dd>
</dl>

<dl>
<dd>

**discount:** `Option<UpdateStoreOrderLineEcommerceRequestDiscount>` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `Option<String>` — A unique identifier for the order line item.
    
</dd>
</dl>

<dl>
<dd>

**price:** `Option<UpdateStoreOrderLineEcommerceRequestPrice>` 
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `Option<String>` — A unique identifier for the product associated with the order line item.
    
</dd>
</dl>

<dl>
<dd>

**product_variant_id:** `Option<String>` — A unique identifier for the product variant associated with the order line item.
    
</dd>
</dl>

<dl>
<dd>

**quantity:** `Option<i64>` — The quantity of an order line item.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">list_store_products</a>(store_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListStoreProductsEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a store's products.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .list_store_products(
            &"store_id".to_string(),
            &ListStoreProductsQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_store_product</a>(store_id: String, request: EcommerceStoresOrdersPost) -> Result&lt;ECommerceProduct, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a new product to a store.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .create_store_product(
            &"store_id".to_string(),
            &EcommerceStoresOrdersPost {
                description: None,
                handle: None,
                id: EcommerceStoresOrdersPostID::String("id".to_string()),
                image_url: None,
                images: None,
                published_at_foreign: None,
                title: "Cat Hat".to_string(),
                r#type: None,
                url: None,
                variants: vec![EcommerceStoresOrdersPostVariantsItem {
                    backorders: None,
                    id: EcommerceStoresOrdersPostVariantsItemID::String("id".to_string()),
                    image_url: None,
                    inventory_quantity: None,
                    price: None,
                    sku: None,
                    title: "Cat Hat".to_string(),
                    url: None,
                    visibility: None,
                }],
                vendor: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_store_product</a>(store_id: String, product_id: String) -> Result&lt;ECommerceProduct, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific product.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_store_product(
            &"store_id".to_string(),
            &"product_id".to_string(),
            &GetStoreProductQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The id for the product of a store.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">upsert_store_product</a>(store_id: String, product_id: String, request: UpsertStoreProductEcommerceRequest) -> Result&lt;ECommerceProduct, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a specific product.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .upsert_store_product(
            &"store_id".to_string(),
            &"product_id".to_string(),
            &UpsertStoreProductEcommerceRequest {
                id: UpsertStoreProductEcommerceRequestID::String("id".to_string()),
                description: None,
                handle: None,
                image_url: None,
                images: None,
                published_at_foreign: None,
                title: None,
                r#type: None,
                url: None,
                variants: None,
                vendor: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The id for the product of a store.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — The description of a product.
    
</dd>
</dl>

<dl>
<dd>

**handle:** `Option<String>` — The handle of a product.
    
</dd>
</dl>

<dl>
<dd>

**id:** `UpsertStoreProductEcommerceRequestId` — A unique identifier for the product.
    
</dd>
</dl>

<dl>
<dd>

**image_url:** `Option<String>` — The image URL for a product.
    
</dd>
</dl>

<dl>
<dd>

**images:** `Option<Vec<UpsertStoreProductEcommerceRequestImagesItem>>` — An array of the product's images.
    
</dd>
</dl>

<dl>
<dd>

**published_at_foreign:** `Option<String>` — The date and time the product was published.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The title of a product.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<String>` — The type of product.
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — The URL for a product.
    
</dd>
</dl>

<dl>
<dd>

**variants:** `Option<Vec<UpsertStoreProductEcommerceRequestVariantsItem>>` — An array of the product's variants. At least one variant is required for each product. A variant can use the same `id` and `title` as the parent product.
    
</dd>
</dl>

<dl>
<dd>

**vendor:** `Option<String>` — The vendor for a product.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">delete_store_product</a>(store_id: String, product_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a product.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .delete_store_product(&"store_id".to_string(), &"product_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The id for the product of a store.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">update_store_product</a>(store_id: String, product_id: String, request: UpdateStoreProductEcommerceRequest) -> Result&lt;ECommerceProduct, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a specific product.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .update_store_product(
            &"store_id".to_string(),
            &"product_id".to_string(),
            &UpdateStoreProductEcommerceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The id for the product of a store.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — The description of a product.
    
</dd>
</dl>

<dl>
<dd>

**handle:** `Option<String>` — The handle of a product.
    
</dd>
</dl>

<dl>
<dd>

**id:** `Option<UpdateStoreProductEcommerceRequestId>` — A unique identifier for the product.
    
</dd>
</dl>

<dl>
<dd>

**image_url:** `Option<String>` — The image URL for a product.
    
</dd>
</dl>

<dl>
<dd>

**images:** `Option<Vec<UpdateStoreProductEcommerceRequestImagesItem>>` — An array of the product's images.
    
</dd>
</dl>

<dl>
<dd>

**published_at_foreign:** `Option<String>` — The date and time the product was published in ISO 8601 format.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The title of a product.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<String>` — The type of product.
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — The URL for a product.
    
</dd>
</dl>

<dl>
<dd>

**variants:** `Option<Vec<UpdateStoreProductEcommerceRequestVariantsItem>>` — An array of the product's variants. At least one variant is required for each product. A variant can use the same `id` and `title` as the parent product.
    
</dd>
</dl>

<dl>
<dd>

**vendor:** `Option<String>` — The vendor for a product.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">list_store_product_images</a>(store_id: String, product_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListStoreProductImagesEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a product's images.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .list_store_product_images(
            &"store_id".to_string(),
            &"product_id".to_string(),
            &ListStoreProductImagesQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The id for the product of a store.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_store_product_image</a>(store_id: String, product_id: String, request: CreateStoreProductImageEcommerceRequest) -> Result&lt;CreateStoreProductImageEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a new image to the product.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .create_store_product_image(
            &"store_id".to_string(),
            &"product_id".to_string(),
            &CreateStoreProductImageEcommerceRequest {
                id: "id".to_string(),
                url: "url".to_string(),
                variant_ids: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The id for the product of a store.
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` — A unique identifier for the product image.
    
</dd>
</dl>

<dl>
<dd>

**url:** `String` — The URL for a product image.
    
</dd>
</dl>

<dl>
<dd>

**variant_ids:** `Option<Vec<CreateStoreProductImageEcommerceRequestVariantIdsItem>>` — The list of product variants using the image.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_store_product_image</a>(store_id: String, product_id: String, image_id: String) -> Result&lt;GetStoreProductImageEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific product image.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_store_product_image(
            &"store_id".to_string(),
            &"product_id".to_string(),
            &"image_id".to_string(),
            &GetStoreProductImageQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The id for the product of a store.
    
</dd>
</dl>

<dl>
<dd>

**image_id:** `String` — The id for the product image.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">delete_store_product_image</a>(store_id: String, product_id: String, image_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a product image.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .delete_store_product_image(
            &"store_id".to_string(),
            &"product_id".to_string(),
            &"image_id".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The id for the product of a store.
    
</dd>
</dl>

<dl>
<dd>

**image_id:** `String` — The id for the product image.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">update_store_product_image</a>(store_id: String, product_id: String, image_id: String, request: UpdateStoreProductImageEcommerceRequest) -> Result&lt;UpdateStoreProductImageEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a product image.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .update_store_product_image(
            &"store_id".to_string(),
            &"product_id".to_string(),
            &"image_id".to_string(),
            &UpdateStoreProductImageEcommerceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The id for the product of a store.
    
</dd>
</dl>

<dl>
<dd>

**image_id:** `String` — The id for the product image.
    
</dd>
</dl>

<dl>
<dd>

**id:** `Option<String>` — A unique identifier for the product image.
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — The URL for a product image.
    
</dd>
</dl>

<dl>
<dd>

**variant_ids:** `Option<Vec<UpdateStoreProductImageEcommerceRequestVariantIdsItem>>` — The list of product variants using the image.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">list_store_product_variants</a>(store_id: String, product_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListStoreProductVariantsEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a product's variants.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .list_store_product_variants(
            &"store_id".to_string(),
            &"product_id".to_string(),
            &ListStoreProductVariantsQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The id for the product of a store.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_store_product_variant</a>(store_id: String, product_id: String, request: CreateStoreProductVariantEcommerceRequest) -> Result&lt;ECommerceProductVariant, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a new variant to the product.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .create_store_product_variant(
            &"store_id".to_string(),
            &"product_id".to_string(),
            &CreateStoreProductVariantEcommerceRequest {
                id: CreateStoreProductVariantEcommerceRequestID::String("id".to_string()),
                title: "Cat Hat".to_string(),
                backorders: None,
                image_url: None,
                inventory_quantity: None,
                price: None,
                sku: None,
                url: None,
                visibility: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The id for the product of a store.
    
</dd>
</dl>

<dl>
<dd>

**backorders:** `Option<String>` — The backorders of a product variant.
    
</dd>
</dl>

<dl>
<dd>

**id:** `CreateStoreProductVariantEcommerceRequestId` — A unique identifier for the product variant.
    
</dd>
</dl>

<dl>
<dd>

**image_url:** `Option<String>` — The image URL for a product variant.
    
</dd>
</dl>

<dl>
<dd>

**inventory_quantity:** `Option<i64>` — The inventory quantity of a product variant.
    
</dd>
</dl>

<dl>
<dd>

**price:** `Option<CreateStoreProductVariantEcommerceRequestPrice>` 
    
</dd>
</dl>

<dl>
<dd>

**sku:** `Option<String>` — The stock keeping unit (SKU) of a product variant.
    
</dd>
</dl>

<dl>
<dd>

**title:** `String` — The title of a product variant.
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — The URL for a product variant.
    
</dd>
</dl>

<dl>
<dd>

**visibility:** `Option<String>` — The visibility of a product variant.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_store_product_variant</a>(store_id: String, product_id: String, variant_id: String) -> Result&lt;ECommerceProductVariant, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific product variant.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_store_product_variant(
            &"store_id".to_string(),
            &"product_id".to_string(),
            &"variant_id".to_string(),
            &GetStoreProductVariantQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The id for the product of a store.
    
</dd>
</dl>

<dl>
<dd>

**variant_id:** `String` — The id for the product variant.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">upsert_store_product_variant</a>(store_id: String, product_id: String, variant_id: String, request: UpsertStoreProductVariantEcommerceRequest) -> Result&lt;ECommerceProductVariant, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add or update a product variant.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .upsert_store_product_variant(
            &"store_id".to_string(),
            &"product_id".to_string(),
            &"variant_id".to_string(),
            &UpsertStoreProductVariantEcommerceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The id for the product of a store.
    
</dd>
</dl>

<dl>
<dd>

**variant_id:** `String` — The id for the product variant.
    
</dd>
</dl>

<dl>
<dd>

**backorders:** `Option<String>` — The backorders of a product variant.
    
</dd>
</dl>

<dl>
<dd>

**id:** `Option<String>` — A unique identifier for the product variant.
    
</dd>
</dl>

<dl>
<dd>

**image_url:** `Option<String>` — The image URL for a product variant.
    
</dd>
</dl>

<dl>
<dd>

**inventory_quantity:** `Option<i64>` — The inventory quantity of a product variant.
    
</dd>
</dl>

<dl>
<dd>

**price:** `Option<UpsertStoreProductVariantEcommerceRequestPrice>` 
    
</dd>
</dl>

<dl>
<dd>

**sku:** `Option<String>` — The stock keeping unit (SKU) of a product variant.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The title of a product variant.
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — The URL for a product variant.
    
</dd>
</dl>

<dl>
<dd>

**visibility:** `Option<String>` — The visibility of a product variant.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">delete_store_product_variant</a>(store_id: String, product_id: String, variant_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a product variant.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .delete_store_product_variant(
            &"store_id".to_string(),
            &"product_id".to_string(),
            &"variant_id".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The id for the product of a store.
    
</dd>
</dl>

<dl>
<dd>

**variant_id:** `String` — The id for the product variant.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">update_store_product_variant</a>(store_id: String, product_id: String, variant_id: String, request: UpdateStoreProductVariantEcommerceRequest) -> Result&lt;ECommerceProductVariant, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a product variant.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .update_store_product_variant(
            &"store_id".to_string(),
            &"product_id".to_string(),
            &"variant_id".to_string(),
            &UpdateStoreProductVariantEcommerceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The id for the product of a store.
    
</dd>
</dl>

<dl>
<dd>

**variant_id:** `String` — The id for the product variant.
    
</dd>
</dl>

<dl>
<dd>

**backorders:** `Option<String>` — The backorders of a product variant.
    
</dd>
</dl>

<dl>
<dd>

**image_url:** `Option<String>` — The image URL for a product variant.
    
</dd>
</dl>

<dl>
<dd>

**inventory_quantity:** `Option<i64>` — The inventory quantity of a product variant.
    
</dd>
</dl>

<dl>
<dd>

**price:** `Option<UpdateStoreProductVariantEcommerceRequestPrice>` 
    
</dd>
</dl>

<dl>
<dd>

**sku:** `Option<String>` — The stock keeping unit (SKU) of a product variant.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The title of a product variant.
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — The URL for a product variant.
    
</dd>
</dl>

<dl>
<dd>

**visibility:** `Option<String>` — The visibility of a product variant.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">list_store_promo_rules</a>(store_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListStorePromoRulesEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a store's promo rules.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .list_store_promo_rules(
            &"store_id".to_string(),
            &ListStorePromoRulesQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_store_promo_rule</a>(store_id: String, request: CreateStorePromoRuleEcommerceRequest) -> Result&lt;ECommercePromoRule, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a new promo rule to a store.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .create_store_promo_rule(
            &"store_id".to_string(),
            &CreateStorePromoRuleEcommerceRequest {
                amount: CreateStorePromoRuleEcommerceRequestAmount::Double(1.1),
                description: "Save BIG during our summer sale!".to_string(),
                id: "id".to_string(),
                target: CreateStorePromoRuleEcommerceRequestTarget::PerItem,
                r#type: CreateStorePromoRuleEcommerceRequestType::Fixed,
                created_at_foreign: None,
                enabled: None,
                ends_at: None,
                starts_at: None,
                title: None,
                updated_at_foreign: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**amount:** `CreateStorePromoRuleEcommerceRequestAmount` 
    
</dd>
</dl>

<dl>
<dd>

**created_at_foreign:** `Option<String>` — The date and time the promotion was created in ISO 8601 format.
    
</dd>
</dl>

<dl>
<dd>

**description:** `String` — The description of a promotion restricted to UTF-8 characters with max length 255.
    
</dd>
</dl>

<dl>
<dd>

**enabled:** `Option<bool>` — Whether the promo rule is currently enabled.
    
</dd>
</dl>

<dl>
<dd>

**ends_at:** `Option<CreateStorePromoRuleEcommerceRequestEndsAt>` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` — A unique identifier for the promo rule. If Ecommerce platform does not support promo rule, use promo code id as promo rule id. Restricted to UTF-8 characters with max length 50.
    
</dd>
</dl>

<dl>
<dd>

**starts_at:** `Option<CreateStorePromoRuleEcommerceRequestStartsAt>` 
    
</dd>
</dl>

<dl>
<dd>

**target:** `CreateStorePromoRuleEcommerceRequestTarget` — The target that the discount applies to.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The title that will show up in promotion campaign. Restricted to UTF-8 characters with max length of 100 bytes.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `CreateStorePromoRuleEcommerceRequestType` — Type of discount. For free shipping set type to fixed.
    
</dd>
</dl>

<dl>
<dd>

**updated_at_foreign:** `Option<String>` — The date and time the promotion was updated in ISO 8601 format.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_store_promo_rule</a>(store_id: String, promo_rule_id: String) -> Result&lt;ECommercePromoRule, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific promo rule.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_store_promo_rule(
            &"store_id".to_string(),
            &"promo_rule_id".to_string(),
            &GetStorePromoRuleQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**promo_rule_id:** `String` — The id for the promo rule of a store.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">delete_store_promo_rule</a>(store_id: String, promo_rule_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a promo rule from a store.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .delete_store_promo_rule(&"store_id".to_string(), &"promo_rule_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**promo_rule_id:** `String` — The id for the promo rule of a store.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">update_store_promo_rule</a>(store_id: String, promo_rule_id: String, request: UpdateStorePromoRuleEcommerceRequest) -> Result&lt;ECommercePromoRule, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a promo rule.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .update_store_promo_rule(
            &"store_id".to_string(),
            &"promo_rule_id".to_string(),
            &UpdateStorePromoRuleEcommerceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**promo_rule_id:** `String` — The id for the promo rule of a store.
    
</dd>
</dl>

<dl>
<dd>

**amount:** `Option<UpdateStorePromoRuleEcommerceRequestAmount>` 
    
</dd>
</dl>

<dl>
<dd>

**created_at_foreign:** `Option<String>` — The date and time the promotion was created in ISO 8601 format.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — The description of a promotion restricted to UTF-8 characters with max length 255.
    
</dd>
</dl>

<dl>
<dd>

**enabled:** `Option<bool>` — Whether the promo rule is currently enabled.
    
</dd>
</dl>

<dl>
<dd>

**ends_at:** `Option<UpdateStorePromoRuleEcommerceRequestEndsAt>` 
    
</dd>
</dl>

<dl>
<dd>

**id:** `Option<String>` — A unique identifier for the promo rule. If Ecommerce platform does not support promo rule, use promo code id as promo rule id. Restricted to UTF-8 characters with max length 50.
    
</dd>
</dl>

<dl>
<dd>

**starts_at:** `Option<UpdateStorePromoRuleEcommerceRequestStartsAt>` 
    
</dd>
</dl>

<dl>
<dd>

**target:** `Option<UpdateStorePromoRuleEcommerceRequestTarget>` — The target that the discount applies to.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The title that will show up in promotion campaign. Restricted to UTF-8 characters with max length of 100 bytes.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<UpdateStorePromoRuleEcommerceRequestType>` — Type of discount. For free shipping set type to fixed.
    
</dd>
</dl>

<dl>
<dd>

**updated_at_foreign:** `Option<String>` — The date and time the promotion was updated in ISO 8601 format.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">list_store_promo_rule_promo_codes</a>(store_id: String, promo_rule_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListStorePromoRulePromoCodesEcommerceResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a store's promo codes.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .list_store_promo_rule_promo_codes(
            &"store_id".to_string(),
            &"promo_rule_id".to_string(),
            &ListStorePromoRulePromoCodesQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**promo_rule_id:** `String` — The id for the promo rule of a store.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">create_store_promo_rule_promo_code</a>(store_id: String, promo_rule_id: String, request: CreateStorePromoRulePromoCodeEcommerceRequest) -> Result&lt;ECommercePromoCode, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a new promo code to a store.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client.ecommerce.create_store_promo_rule_promo_code(&"store_id".to_string(), &"promo_rule_id".to_string(), &CreateStorePromoRulePromoCodeEcommerceRequest {
        code: "summersale".to_string(),
        id: "id".to_string(),
        redemption_url: "A url that applies promo code directly at checkout or a url that points to sale page or store url".to_string(),
        created_at_foreign: None,
        enabled: None,
        updated_at_foreign: None,
        usage_count: None
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**promo_rule_id:** `String` — The id for the promo rule of a store.
    
</dd>
</dl>

<dl>
<dd>

**code:** `String` — The discount code. Restricted to UTF-8 characters with max length 50.
    
</dd>
</dl>

<dl>
<dd>

**created_at_foreign:** `Option<String>` — The date and time the promotion was created in ISO 8601 format.
    
</dd>
</dl>

<dl>
<dd>

**enabled:** `Option<bool>` — Whether the promo code is currently enabled.
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` — A unique identifier for the promo code. Restricted to UTF-8 characters with max length 50.
    
</dd>
</dl>

<dl>
<dd>

**redemption_url:** `String` — The url that should be used in the promotion campaign restricted to UTF-8 characters with max length 2000.
    
</dd>
</dl>

<dl>
<dd>

**updated_at_foreign:** `Option<String>` — The date and time the promotion was updated in ISO 8601 format.
    
</dd>
</dl>

<dl>
<dd>

**usage_count:** `Option<i64>` — Number of times promo code has been used.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">get_store_promo_rule_promo_code</a>(store_id: String, promo_rule_id: String, promo_code_id: String) -> Result&lt;ECommercePromoCode, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific promo code.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .get_store_promo_rule_promo_code(
            &"store_id".to_string(),
            &"promo_rule_id".to_string(),
            &"promo_code_id".to_string(),
            &GetStorePromoRulePromoCodeQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**promo_rule_id:** `String` — The id for the promo rule of a store.
    
</dd>
</dl>

<dl>
<dd>

**promo_code_id:** `String` — The id for the promo code of a store.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">delete_store_promo_rule_promo_code</a>(store_id: String, promo_rule_id: String, promo_code_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a promo code from a store.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .delete_store_promo_rule_promo_code(
            &"store_id".to_string(),
            &"promo_rule_id".to_string(),
            &"promo_code_id".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**promo_rule_id:** `String` — The id for the promo rule of a store.
    
</dd>
</dl>

<dl>
<dd>

**promo_code_id:** `String` — The id for the promo code of a store.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ecommerce.<a href="/src/api/resources/ecommerce/client.rs">update_store_promo_rule_promo_code</a>(store_id: String, promo_rule_id: String, promo_code_id: String, request: UpdateStorePromoRulePromoCodeEcommerceRequest) -> Result&lt;ECommercePromoCode, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a promo code.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .ecommerce
        .update_store_promo_rule_promo_code(
            &"store_id".to_string(),
            &"promo_rule_id".to_string(),
            &"promo_code_id".to_string(),
            &UpdateStorePromoRulePromoCodeEcommerceRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**store_id:** `String` — The store id.
    
</dd>
</dl>

<dl>
<dd>

**promo_rule_id:** `String` — The id for the promo rule of a store.
    
</dd>
</dl>

<dl>
<dd>

**promo_code_id:** `String` — The id for the promo code of a store.
    
</dd>
</dl>

<dl>
<dd>

**code:** `Option<String>` — The discount code. Restricted to UTF-8 characters with max length 50.
    
</dd>
</dl>

<dl>
<dd>

**created_at_foreign:** `Option<String>` — The date and time the promotion was created in ISO 8601 format.
    
</dd>
</dl>

<dl>
<dd>

**enabled:** `Option<bool>` — Whether the promo code is currently enabled.
    
</dd>
</dl>

<dl>
<dd>

**id:** `Option<String>` — A unique identifier for the promo code. Restricted to UTF-8 characters with max length 50.
    
</dd>
</dl>

<dl>
<dd>

**redemption_url:** `Option<String>` — The url that should be used in the promotion campaign restricted to UTF-8 characters with max length 2000.
    
</dd>
</dl>

<dl>
<dd>

**updated_at_foreign:** `Option<String>` — The date and time the promotion was updated in ISO 8601 format.
    
</dd>
</dl>

<dl>
<dd>

**usage_count:** `Option<i64>` — Number of times promo code has been used.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## FacebookAds
<details><summary><code>client.facebook_ads.<a href="/src/api/resources/facebook_ads/client.rs">list</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, sort_field: Option&lt;Option&lt;ListFacebookAdsRequestSortField&gt;&gt;, sort_dir: Option&lt;Option&lt;ListFacebookAdsRequestSortDir&gt;&gt;) -> Result&lt;ListFacebookAdsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get list of Facebook ads.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .facebook_ads
        .list(
            &FacebookAdsListQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                sort_field: None,
                sort_dir: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<ListFacebookAdsRequestSortField>` — Returns files sorted by the specified field.
    
</dd>
</dl>

<dl>
<dd>

**sort_dir:** `Option<ListFacebookAdsRequestSortDir>` — Determines the order direction for sorted results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.facebook_ads.<a href="/src/api/resources/facebook_ads/client.rs">get</a>(outreach_id: String) -> Result&lt;FacebookAds, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get details of a Facebook ad.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .facebook_ads
        .get(
            &"outreach_id".to_string(),
            &FacebookAdsGetQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**outreach_id:** `String` — The outreach id.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## FileManager
<details><summary><code>client.file_manager.<a href="/src/api/resources/file_manager/client.rs">list</a>() -> Result&lt;Vec&lt;ListFileManagerResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about the file-manager endpoint's resources
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client.file_manager.list(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.file_manager.<a href="/src/api/resources/file_manager/client.rs">list_files</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, type_: Option&lt;Option&lt;String&gt;&gt;, created_by: Option&lt;Option&lt;String&gt;&gt;, before_created_at: Option&lt;Option&lt;String&gt;&gt;, since_created_at: Option&lt;Option&lt;String&gt;&gt;, sort_field: Option&lt;Option&lt;ListFilesFileManagerRequestSortField&gt;&gt;, sort_dir: Option&lt;Option&lt;ListFilesFileManagerRequestSortDir&gt;&gt;) -> Result&lt;ListFilesFileManagerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of available images and files stored in the File Manager for the account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .file_manager
        .list_files(
            &ListFilesQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                r#type: None,
                created_by: None,
                before_created_at: None,
                since_created_at: None,
                sort_field: None,
                sort_dir: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<String>` — The file type for the File Manager file.
    
</dd>
</dl>

<dl>
<dd>

**created_by:** `Option<String>` — The Mailchimp account user who created the File Manager file.
    
</dd>
</dl>

<dl>
<dd>

**before_created_at:** `Option<String>` — Restrict the response to files created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**since_created_at:** `Option<String>` — Restrict the response to files created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<ListFilesFileManagerRequestSortField>` — Returns files sorted by the specified field.
    
</dd>
</dl>

<dl>
<dd>

**sort_dir:** `Option<ListFilesFileManagerRequestSortDir>` — Determines the order direction for sorted results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.file_manager.<a href="/src/api/resources/file_manager/client.rs">create_file</a>(request: CreateFileFileManagerRequest) -> Result&lt;GalleryFile, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Upload a new image or file to the File Manager.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .file_manager
        .create_file(
            &CreateFileFileManagerRequest {
                file_data: "file_data".to_string(),
                name: "name".to_string(),
                folder_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**file_data:** `String` — The base64-encoded contents of the file.
    
</dd>
</dl>

<dl>
<dd>

**folder_id:** `Option<i64>` — The id of the folder.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — The name of the file.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.file_manager.<a href="/src/api/resources/file_manager/client.rs">get_file</a>(file_id: String) -> Result&lt;GalleryFile, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific file in the File Manager.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .file_manager
        .get_file(
            &"file_id".to_string(),
            &GetFileQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**file_id:** `String` — The unique id for the File Manager file.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.file_manager.<a href="/src/api/resources/file_manager/client.rs">delete_file</a>(file_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Remove a specific file from the File Manager.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .file_manager
        .delete_file(&"file_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**file_id:** `String` — The unique id for the File Manager file.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.file_manager.<a href="/src/api/resources/file_manager/client.rs">update_file</a>(file_id: String, request: UpdateFileFileManagerRequest) -> Result&lt;GalleryFile, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a file in the File Manager.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .file_manager
        .update_file(
            &"file_id".to_string(),
            &UpdateFileFileManagerRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**file_id:** `String` — The unique id for the File Manager file.
    
</dd>
</dl>

<dl>
<dd>

**folder_id:** `Option<i64>` — The id of the folder. Setting `folder_id` to `0` will remove a file from its current folder.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — The name of the file.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.file_manager.<a href="/src/api/resources/file_manager/client.rs">list_folders</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, created_by: Option&lt;Option&lt;String&gt;&gt;, before_created_at: Option&lt;Option&lt;String&gt;&gt;, since_created_at: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListFoldersFileManagerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of all folders in the File Manager.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .file_manager
        .list_folders(
            &ListFoldersQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                created_by: None,
                before_created_at: None,
                since_created_at: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**created_by:** `Option<String>` — The Mailchimp account user who created the File Manager file.
    
</dd>
</dl>

<dl>
<dd>

**before_created_at:** `Option<String>` — Restrict the response to files created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**since_created_at:** `Option<String>` — Restrict the response to files created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.file_manager.<a href="/src/api/resources/file_manager/client.rs">create_folder</a>(request: CreateFolderFileManagerRequest) -> Result&lt;CreateFolderFileManagerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new folder in the File Manager.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .file_manager
        .create_folder(
            &CreateFolderFileManagerRequest {
                name: "name".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` — The name of the folder.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.file_manager.<a href="/src/api/resources/file_manager/client.rs">get_folder</a>(folder_id: String) -> Result&lt;GetFolderFileManagerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific folder in the File Manager.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .file_manager
        .get_folder(
            &"folder_id".to_string(),
            &GetFolderQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**folder_id:** `String` — The unique id for the File Manager folder.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.file_manager.<a href="/src/api/resources/file_manager/client.rs">delete_folder</a>(folder_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a specific folder in the File Manager.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .file_manager
        .delete_folder(&"folder_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**folder_id:** `String` — The unique id for the File Manager folder.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.file_manager.<a href="/src/api/resources/file_manager/client.rs">update_folder</a>(folder_id: String, request: UpdateFolderFileManagerRequest) -> Result&lt;UpdateFolderFileManagerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a specific File Manager folder.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .file_manager
        .update_folder(
            &"folder_id".to_string(),
            &UpdateFolderFileManagerRequest {
                name: "name".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**folder_id:** `String` — The unique id for the File Manager folder.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — The name of the folder.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.file_manager.<a href="/src/api/resources/file_manager/client.rs">list_folder_files</a>(folder_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, type_: Option&lt;Option&lt;String&gt;&gt;, created_by: Option&lt;Option&lt;String&gt;&gt;, before_created_at: Option&lt;Option&lt;String&gt;&gt;, since_created_at: Option&lt;Option&lt;String&gt;&gt;, sort_field: Option&lt;Option&lt;ListFolderFilesFileManagerRequestSortField&gt;&gt;, sort_dir: Option&lt;Option&lt;ListFolderFilesFileManagerRequestSortDir&gt;&gt;) -> Result&lt;ListFolderFilesFileManagerResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of available images and files stored in this folder.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .file_manager
        .list_folder_files(
            &"folder_id".to_string(),
            &ListFolderFilesQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                r#type: None,
                created_by: None,
                before_created_at: None,
                since_created_at: None,
                sort_field: None,
                sort_dir: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**folder_id:** `String` — The unique id for the File Manager folder.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<String>` — The file type for the File Manager file.
    
</dd>
</dl>

<dl>
<dd>

**created_by:** `Option<String>` — The Mailchimp account user who created the File Manager file.
    
</dd>
</dl>

<dl>
<dd>

**before_created_at:** `Option<String>` — Restrict the response to files created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**since_created_at:** `Option<String>` — Restrict the response to files created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<ListFolderFilesFileManagerRequestSortField>` — Returns files sorted by the specified field.
    
</dd>
</dl>

<dl>
<dd>

**sort_dir:** `Option<ListFolderFilesFileManagerRequestSortDir>` — Determines the order direction for sorted results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## LandingPages
<details><summary><code>client.landing_pages.<a href="/src/api/resources/landing_pages/client.rs">list</a>(sort_dir: Option&lt;Option&lt;ListLandingPagesRequestSortDir&gt;&gt;, sort_field: Option&lt;Option&lt;ListLandingPagesRequestSortField&gt;&gt;, count: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListLandingPagesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get all landing pages.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .landing_pages
        .list(
            &LandingPagesListQueryRequest {
                sort_dir: None,
                sort_field: None,
                fields: vec![],
                exclude_fields: vec![],
                count: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sort_dir:** `Option<ListLandingPagesRequestSortDir>` — Determines the order direction for sorted results.
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<ListLandingPagesRequestSortField>` — Returns files sorted by the specified field.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.landing_pages.<a href="/src/api/resources/landing_pages/client.rs">create</a>(request: CreateLandingPagesRequest, use_default_list: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;LandingPage, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create an unpublished and contentless Mailchimp landing page.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .landing_pages
        .create(
            &CreateLandingPagesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**description:** `Option<String>` — The description of this landing page.
    
</dd>
</dl>

<dl>
<dd>

**list_id:** `Option<String>` — The list's ID associated with this landing page.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — The name of this landing page.
    
</dd>
</dl>

<dl>
<dd>

**store_id:** `Option<String>` — The ID of the store associated with this landing page.
    
</dd>
</dl>

<dl>
<dd>

**template_id:** `Option<i64>` — The template_id of this landing page.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The title of this landing page seen in the browser's title bar.
    
</dd>
</dl>

<dl>
<dd>

**tracking:** `Option<CreateLandingPagesRequestTracking>` — The tracking settings applied to this landing page.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<CreateLandingPagesRequestType>` — The type of template the landing page has.
    
</dd>
</dl>

<dl>
<dd>

**use_default_list:** `Option<bool>` — Will create the Landing Page using the account's Default List instead of requiring a list_id.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.landing_pages.<a href="/src/api/resources/landing_pages/client.rs">get</a>(page_id: String) -> Result&lt;LandingPage, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific page.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .landing_pages
        .get(
            &"page_id".to_string(),
            &LandingPagesGetQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page_id:** `String` — The unique id for the page.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.landing_pages.<a href="/src/api/resources/landing_pages/client.rs">delete</a>(page_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a landing page.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .landing_pages
        .delete(&"page_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page_id:** `String` — The unique id for the page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.landing_pages.<a href="/src/api/resources/landing_pages/client.rs">update</a>(page_id: String, request: UpdateLandingPagesRequest) -> Result&lt;LandingPage, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a landing page.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .landing_pages
        .update(
            &"page_id".to_string(),
            &UpdateLandingPagesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page_id:** `String` — The unique id for the page.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — The description of this landing page.
    
</dd>
</dl>

<dl>
<dd>

**list_id:** `Option<String>` — The list's ID associated with this landing page.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — The name of this landing page.
    
</dd>
</dl>

<dl>
<dd>

**store_id:** `Option<String>` — The ID of the store associated with this landing page.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The title of this landing page seen in the browser's title bar.
    
</dd>
</dl>

<dl>
<dd>

**tracking:** `Option<UpdateLandingPagesRequestTracking>` — The tracking settings applied to this landing page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.landing_pages.<a href="/src/api/resources/landing_pages/client.rs">create_action_publish</a>(page_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Publish a landing page that is in draft, unpublished, or has been previously published and edited.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .landing_pages
        .create_action_publish(&"page_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page_id:** `String` — The unique id for the page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.landing_pages.<a href="/src/api/resources/landing_pages/client.rs">create_action_unpublish</a>(page_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Unpublish a landing page that is in draft or has been published.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .landing_pages
        .create_action_unpublish(&"page_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page_id:** `String` — The unique id for the page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.landing_pages.<a href="/src/api/resources/landing_pages/client.rs">list_content</a>(page_id: String) -> Result&lt;ListContentLandingPagesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the the HTML for your landing page.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .landing_pages
        .list_content(
            &"page_id".to_string(),
            &ListContentQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page_id:** `String` — The unique id for the page.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## lists
<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, before_date_created: Option&lt;Option&lt;String&gt;&gt;, since_date_created: Option&lt;Option&lt;String&gt;&gt;, before_campaign_last_sent: Option&lt;Option&lt;String&gt;&gt;, since_campaign_last_sent: Option&lt;Option&lt;String&gt;&gt;, email: Option&lt;Option&lt;String&gt;&gt;, sort_field: Option&lt;Option&lt;ListListsRequestSortField&gt;&gt;, sort_dir: Option&lt;Option&lt;ListListsRequestSortDir&gt;&gt;, has_ecommerce_store: Option&lt;Option&lt;bool&gt;&gt;, include_total_contacts: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;ListListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about all lists in the account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list(
            &ListsListQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                before_date_created: None,
                since_date_created: None,
                before_campaign_last_sent: None,
                since_campaign_last_sent: None,
                email: None,
                sort_field: None,
                sort_dir: None,
                has_ecommerce_store: None,
                include_total_contacts: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**before_date_created:** `Option<String>` — Restrict response to lists created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**since_date_created:** `Option<String>` — Restrict results to lists created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**before_campaign_last_sent:** `Option<String>` — Restrict results to lists created before the last campaign send date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**since_campaign_last_sent:** `Option<String>` — Restrict results to lists created after the last campaign send date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` — Restrict results to lists that include a specific subscriber's email address.
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<ListListsRequestSortField>` — Returns files sorted by the specified field.
    
</dd>
</dl>

<dl>
<dd>

**sort_dir:** `Option<ListListsRequestSortDir>` — Determines the order direction for sorted results.
    
</dd>
</dl>

<dl>
<dd>

**has_ecommerce_store:** `Option<bool>` — Restrict results to lists that contain an active, connected, undeleted ecommerce store.
    
</dd>
</dl>

<dl>
<dd>

**include_total_contacts:** `Option<bool>` — Deprecated. Return the total_contacts field in the stats response, which contains an approximate count of subscribed, unsubscribed, and transactional contacts. For a complete audience contact count, use the /audiences endpoint instead.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">create</a>(request: CreateListsRequest) -> Result&lt;SubscriberList, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new list in your Mailchimp account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .create(
            &CreateListsRequest {
                campaign_defaults: CreateListsRequestCampaignDefaults {
                    from_email: "from_email".to_string(),
                    from_name: "from_name".to_string(),
                    language: "language".to_string(),
                    subject: "subject".to_string(),
                    ..Default::default()
                },
                contact: CreateListsRequestContact {
                    address1: "address1".to_string(),
                    city: "city".to_string(),
                    company: "company".to_string(),
                    country: "country".to_string(),
                    ..Default::default()
                },
                email_type_option: true,
                name: "name".to_string(),
                permission_reminder: "permission_reminder".to_string(),
                double_optin: None,
                marketing_permissions: None,
                notify_on_subscribe: None,
                notify_on_unsubscribe: None,
                use_archive_bar: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_defaults:** `CreateListsRequestCampaignDefaults` — [Default values for campaigns](https://mailchimp.com/help/edit-your-emails-subject-preview-text-from-name-or-from-email-address/) created for this list.
    
</dd>
</dl>

<dl>
<dd>

**contact:** `CreateListsRequestContact` — [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    
</dd>
</dl>

<dl>
<dd>

**double_optin:** `Option<bool>` — Whether or not to require the subscriber to confirm subscription via email.
    
</dd>
</dl>

<dl>
<dd>

**email_type_option:** `bool` — Whether the list supports [multiple formats for emails](https://mailchimp.com/help/audience-settings-and-defaults/). When set to `true`, subscribers can choose whether they want to receive HTML or plain-text emails. When set to `false`, subscribers will receive HTML emails, with a plain-text alternative backup.
    
</dd>
</dl>

<dl>
<dd>

**marketing_permissions:** `Option<bool>` — Whether or not the list has marketing permissions (eg. GDPR) enabled.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — The name of the list.
    
</dd>
</dl>

<dl>
<dd>

**notify_on_subscribe:** `Option<String>` — The email address to send [subscribe notifications](https://mailchimp.com/help/change-subscribe-and-unsubscribe-notifications/) to.
    
</dd>
</dl>

<dl>
<dd>

**notify_on_unsubscribe:** `Option<String>` — The email address to send [unsubscribe notifications](https://mailchimp.com/help/change-subscribe-and-unsubscribe-notifications/) to.
    
</dd>
</dl>

<dl>
<dd>

**permission_reminder:** `String` — The [permission reminder](https://mailchimp.com/help/edit-the-permission-reminder/) for the list.
    
</dd>
</dl>

<dl>
<dd>

**use_archive_bar:** `Option<bool>` — Whether campaigns for this list use the [Archive Bar](https://mailchimp.com/help/about-email-campaign-archives-and-pages/) in archives by default.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">get</a>(list_id: String, include_total_contacts: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;SubscriberList, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific list in your Mailchimp account. Results include list members who have signed up but haven't confirmed their subscription yet and unsubscribed or cleaned.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .get(
            &"list_id".to_string(),
            &ListsGetQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                include_total_contacts: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**include_total_contacts:** `Option<bool>` — Deprecated. Return the total_contacts field in the stats response, which contains an approximate count of subscribed, unsubscribed, and transactional contacts. For a complete audience contact count, use the /audiences endpoint instead.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">batch_subscribe_or_unsubscribe</a>(list_id: String, request: BatchSubscribeOrUnsubscribeListsRequest, skip_merge_validation: Option&lt;Option&lt;bool&gt;&gt;, skip_duplicate_check: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;BatchSubscribeOrUnsubscribeListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Batch subscribe or unsubscribe list members.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .batch_subscribe_or_unsubscribe(
            &"list_id".to_string(),
            &BatchSubscribeOrUnsubscribeListsRequest {
                members: vec![],
                skip_merge_validation: None,
                skip_duplicate_check: None,
                sync_tags: None,
                update_existing: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**members:** `Vec<BatchSubscribeOrUnsubscribeListsRequestMembersItem>` — An array of objects, each representing an email address and the subscription status for a specific list. Up to 500 members may be added or updated with each API call.
    
</dd>
</dl>

<dl>
<dd>

**sync_tags:** `Option<Option<bool>>` — Whether this batch operation will replace all existing tags with tags in request.
    
</dd>
</dl>

<dl>
<dd>

**update_existing:** `Option<Option<bool>>` — Whether this batch operation will change existing members' subscription status.
    
</dd>
</dl>

<dl>
<dd>

**skip_merge_validation:** `Option<bool>` — If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
    
</dd>
</dl>

<dl>
<dd>

**skip_duplicate_check:** `Option<bool>` — If skip_duplicate_check is true, we will ignore duplicates sent in the request when using the batch sub/unsub on the lists endpoint. The status of the first appearance in the request will be saved. This defaults to false.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">delete</a>(list_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a list from your Mailchimp account. If you delete a list, you'll lose the list history—including subscriber activity, unsubscribes, complaints, and bounces. You’ll also lose subscribers’ email addresses, unless you exported and backed up your list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client.lists.delete(&"list_id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">update</a>(list_id: String, request: UpdateListsRequest) -> Result&lt;SubscriberList, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update the settings for a specific list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .update(
            &"list_id".to_string(),
            &UpdateListsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**campaign_defaults:** `Option<UpdateListsRequestCampaignDefaults>` — [Default values for campaigns](https://mailchimp.com/help/edit-your-emails-subject-preview-text-from-name-or-from-email-address/) created for this list.
    
</dd>
</dl>

<dl>
<dd>

**contact:** `Option<UpdateListsRequestContact>` — [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    
</dd>
</dl>

<dl>
<dd>

**double_optin:** `Option<bool>` — Whether or not to require the subscriber to confirm subscription via email.
    
</dd>
</dl>

<dl>
<dd>

**email_type_option:** `Option<bool>` — Whether the list supports [multiple formats for emails](https://mailchimp.com/help/audience-settings-and-defaults/). When set to `true`, subscribers can choose whether they want to receive HTML or plain-text emails. When set to `false`, subscribers will receive HTML emails, with a plain-text alternative backup.
    
</dd>
</dl>

<dl>
<dd>

**marketing_permissions:** `Option<bool>` — Whether or not the list has marketing permissions (eg. GDPR) enabled.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — The name of the list.
    
</dd>
</dl>

<dl>
<dd>

**notify_on_subscribe:** `Option<String>` — The email address to send [subscribe notifications](https://mailchimp.com/help/change-subscribe-and-unsubscribe-notifications/) to.
    
</dd>
</dl>

<dl>
<dd>

**notify_on_unsubscribe:** `Option<String>` — The email address to send [unsubscribe notifications](https://mailchimp.com/help/change-subscribe-and-unsubscribe-notifications/) to.
    
</dd>
</dl>

<dl>
<dd>

**permission_reminder:** `Option<String>` — The [permission reminder](https://mailchimp.com/help/edit-the-permission-reminder/) for the list.
    
</dd>
</dl>

<dl>
<dd>

**use_archive_bar:** `Option<bool>` — Whether campaigns for this list use the [Archive Bar](https://mailchimp.com/help/about-email-campaign-archives-and-pages/) in archives by default.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_abuse_reports</a>(list_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListAbuseReportsListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get all abuse reports for a specific list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_abuse_reports(
            &"list_id".to_string(),
            &ListsListAbuseReportsQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">get_abuse_report</a>(list_id: String, report_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListsAbuseReports, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get details about a specific abuse report.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .get_abuse_report(
            &"list_id".to_string(),
            &"report_id".to_string(),
            &ListsGetAbuseReportQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**report_id:** `String` — The id for the abuse report.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_activity</a>(list_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListActivityListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get up to the previous 180 days of daily detailed aggregated activity stats for a list, not including Automation activity.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_activity(
            &"list_id".to_string(),
            &ListActivityQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_clients</a>(list_id: String) -> Result&lt;ListClientsListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of the top email clients based on user-agent strings.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_clients(
            &"list_id".to_string(),
            &ListClientsQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_growth_history</a>(list_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, sort_field: Option&lt;Option&lt;ListGrowthHistoryListsRequestSortField&gt;&gt;, sort_dir: Option&lt;Option&lt;ListGrowthHistoryListsRequestSortDir&gt;&gt;) -> Result&lt;ListGrowthHistoryListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a month-by-month summary of a specific list's growth activity.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_growth_history(
            &"list_id".to_string(),
            &ListGrowthHistoryQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                sort_field: None,
                sort_dir: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<ListGrowthHistoryListsRequestSortField>` — Returns files sorted by the specified field.
    
</dd>
</dl>

<dl>
<dd>

**sort_dir:** `Option<ListGrowthHistoryListsRequestSortDir>` — Determines the order direction for sorted results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">get_growth_history</a>(list_id: String, month: String) -> Result&lt;GrowthHistory, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a summary of a specific list's growth activity for a specific month and year.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .get_growth_history(
            &"list_id".to_string(),
            &"month".to_string(),
            &GetGrowthHistoryQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**month:** `String` — A specific month of list growth history.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_interest_categories</a>(list_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, type_: Option&lt;Option&lt;String&gt;&gt;, sort_field: Option&lt;Option&lt;ListInterestCategoriesListsRequestSortField&gt;&gt;, sort_dir: Option&lt;Option&lt;ListInterestCategoriesListsRequestSortDir&gt;&gt;) -> Result&lt;ListInterestCategoriesListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a list's interest categories.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_interest_categories(
            &"list_id".to_string(),
            &ListInterestCategoriesQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                r#type: None,
                sort_field: None,
                sort_dir: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<String>` — Restrict results a type of interest group
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<ListInterestCategoriesListsRequestSortField>` — Returns interest categories sorted by the specified field. Defaults to display_order.
    
</dd>
</dl>

<dl>
<dd>

**sort_dir:** `Option<ListInterestCategoriesListsRequestSortDir>` — Determines the order direction for sorted results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">create_interest_category</a>(list_id: String, request: CreateInterestCategoryListsRequest) -> Result&lt;InterestCategory, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new interest category.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .create_interest_category(
            &"list_id".to_string(),
            &CreateInterestCategoryListsRequest {
                title: "title".to_string(),
                r#type: CreateInterestCategoryListsRequestType::Checkboxes,
                display_order: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**display_order:** `Option<i64>` — The order that the categories are displayed in the list. Lower numbers display first.
    
</dd>
</dl>

<dl>
<dd>

**title:** `String` — The text description of this category. This field appears on signup forms and is often phrased as a question.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `CreateInterestCategoryListsRequestType` — Determines how this category’s interests appear on signup forms.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">get_interest_category</a>(list_id: String, interest_category_id: String) -> Result&lt;InterestCategory, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific interest category.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .get_interest_category(
            &"list_id".to_string(),
            &"interest_category_id".to_string(),
            &GetInterestCategoryQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**interest_category_id:** `String` — The unique ID for the interest category.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">delete_interest_category</a>(list_id: String, interest_category_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a specific interest category.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .delete_interest_category(
            &"list_id".to_string(),
            &"interest_category_id".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**interest_category_id:** `String` — The unique ID for the interest category.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">update_interest_category</a>(list_id: String, interest_category_id: String, request: UpdateInterestCategoryListsRequest) -> Result&lt;InterestCategory, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a specific interest category.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .update_interest_category(
            &"list_id".to_string(),
            &"interest_category_id".to_string(),
            &UpdateInterestCategoryListsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**interest_category_id:** `String` — The unique ID for the interest category.
    
</dd>
</dl>

<dl>
<dd>

**display_order:** `Option<i64>` — The order that the categories are displayed in the list. Lower numbers display first.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The text description of this category. This field appears on signup forms and is often phrased as a question.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<UpdateInterestCategoryListsRequestType>` — Determines how this category’s interests appear on signup forms.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_interest_category_interests</a>(list_id: String, interest_category_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListInterestCategoryInterestsListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of this category's interests.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_interest_category_interests(
            &"list_id".to_string(),
            &"interest_category_id".to_string(),
            &ListInterestCategoryInterestsQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**interest_category_id:** `String` — The unique ID for the interest category.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">create_interest_category_interest</a>(list_id: String, interest_category_id: String, request: CreateInterestCategoryInterestListsRequest) -> Result&lt;Interest, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new interest or 'group name' for a specific category.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .create_interest_category_interest(
            &"list_id".to_string(),
            &"interest_category_id".to_string(),
            &CreateInterestCategoryInterestListsRequest {
                name: "name".to_string(),
                display_order: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**interest_category_id:** `String` — The unique ID for the interest category.
    
</dd>
</dl>

<dl>
<dd>

**display_order:** `Option<i64>` — The display order for interests.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — The name of the interest. This can be shown publicly on a subscription form.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">get_interest_category_interest</a>(list_id: String, interest_category_id: String, interest_id: String) -> Result&lt;Interest, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get interests or 'group names' for a specific category.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .get_interest_category_interest(
            &"list_id".to_string(),
            &"interest_category_id".to_string(),
            &"interest_id".to_string(),
            &GetInterestCategoryInterestQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**interest_category_id:** `String` — The unique ID for the interest category.
    
</dd>
</dl>

<dl>
<dd>

**interest_id:** `String` — The specific interest or 'group name'.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">delete_interest_category_interest</a>(list_id: String, interest_category_id: String, interest_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete interests or group names in a specific category.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .delete_interest_category_interest(
            &"list_id".to_string(),
            &"interest_category_id".to_string(),
            &"interest_id".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**interest_category_id:** `String` — The unique ID for the interest category.
    
</dd>
</dl>

<dl>
<dd>

**interest_id:** `String` — The specific interest or 'group name'.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">update_interest_category_interest</a>(list_id: String, interest_category_id: String, interest_id: String, request: UpdateInterestCategoryInterestListsRequest) -> Result&lt;Interest, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update interests or 'group names' for a specific category.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .update_interest_category_interest(
            &"list_id".to_string(),
            &"interest_category_id".to_string(),
            &"interest_id".to_string(),
            &UpdateInterestCategoryInterestListsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**interest_category_id:** `String` — The unique ID for the interest category.
    
</dd>
</dl>

<dl>
<dd>

**interest_id:** `String` — The specific interest or 'group name'.
    
</dd>
</dl>

<dl>
<dd>

**display_order:** `Option<i64>` — The display order for interests.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — The name of the interest. This can be shown publicly on a subscription form.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_locations</a>(list_id: String) -> Result&lt;ListLocationsListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the locations (countries) that the list's subscribers have been tagged to based on geocoding their IP address.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_locations(
            &"list_id".to_string(),
            &ListsListLocationsQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_members</a>(list_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, email_type: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;ListMembersListsRequestStatus&gt;&gt;, since_timestamp_opt: Option&lt;Option&lt;String&gt;&gt;, before_timestamp_opt: Option&lt;Option&lt;String&gt;&gt;, since_last_changed: Option&lt;Option&lt;String&gt;&gt;, before_last_changed: Option&lt;Option&lt;String&gt;&gt;, unique_email_id: Option&lt;Option&lt;String&gt;&gt;, vip_only: Option&lt;Option&lt;bool&gt;&gt;, interest_category_id: Option&lt;Option&lt;String&gt;&gt;, interest_ids: Option&lt;Option&lt;String&gt;&gt;, interest_match: Option&lt;Option&lt;ListMembersListsRequestInterestMatch&gt;&gt;, sort_field: Option&lt;Option&lt;ListMembersListsRequestSortField&gt;&gt;, sort_dir: Option&lt;Option&lt;ListMembersListsRequestSortDir&gt;&gt;, since_last_campaign: Option&lt;Option&lt;bool&gt;&gt;, unsubscribed_since: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListMembersListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about members in a specific Mailchimp list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_members(
            &"list_id".to_string(),
            &ListMembersQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                email_type: None,
                status: None,
                since_timestamp_opt: None,
                before_timestamp_opt: None,
                since_last_changed: None,
                before_last_changed: None,
                unique_email_id: None,
                vip_only: None,
                interest_category_id: None,
                interest_ids: None,
                interest_match: None,
                sort_field: None,
                sort_dir: None,
                since_last_campaign: None,
                unsubscribed_since: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**email_type:** `Option<String>` — The email type.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListMembersListsRequestStatus>` — The subscriber's status.
    
</dd>
</dl>

<dl>
<dd>

**since_timestamp_opt:** `Option<String>` — Restrict results to subscribers who opted-in after the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**before_timestamp_opt:** `Option<String>` — Restrict results to subscribers who opted-in before the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**since_last_changed:** `Option<String>` — Restrict results to subscribers whose information changed after the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**before_last_changed:** `Option<String>` — Restrict results to subscribers whose information changed before the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**unique_email_id:** `Option<String>` — A unique identifier for the email address across all Mailchimp lists.
    
</dd>
</dl>

<dl>
<dd>

**vip_only:** `Option<bool>` — A filter to return only the list's VIP members. Passing `true` will restrict results to VIP list members, passing `false` will return all list members.
    
</dd>
</dl>

<dl>
<dd>

**interest_category_id:** `Option<String>` — The unique id for the interest category.
    
</dd>
</dl>

<dl>
<dd>

**interest_ids:** `Option<String>` — Used to filter list members by interests. Must be accompanied by interest_category_id and interest_match. The value must be a comma separated list of interest ids present for any supplied interest categories.
    
</dd>
</dl>

<dl>
<dd>

**interest_match:** `Option<ListMembersListsRequestInterestMatch>` — Used to filter list members by interests. Must be accompanied by interest_category_id and interest_ids. "any" will match a member with any of the interest supplied, "all" will only match members with every interest supplied, and "none" will match members without any of the interest supplied.
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<ListMembersListsRequestSortField>` — Returns files sorted by the specified field.
    
</dd>
</dl>

<dl>
<dd>

**sort_dir:** `Option<ListMembersListsRequestSortDir>` — Determines the order direction for sorted results.
    
</dd>
</dl>

<dl>
<dd>

**since_last_campaign:** `Option<bool>` — Filter subscribers by those subscribed/unsubscribed/pending/cleaned since last email campaign send. Member status is required to use this filter.
    
</dd>
</dl>

<dl>
<dd>

**unsubscribed_since:** `Option<String>` — Filter subscribers by those unsubscribed since a specific date. Using any status other than unsubscribed with this filter will result in an error.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">create_member</a>(list_id: String, request: CreateMemberListsRequest, skip_merge_validation: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;ListMembers, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a new member to the list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .create_member(
            &"list_id".to_string(),
            &CreateMemberListsRequest {
                email_address: "email_address".to_string(),
                status: CreateMemberListsRequestStatus::Subscribed,
                skip_merge_validation: None,
                email_type: None,
                interests: None,
                ip_opt: None,
                ip_signup: None,
                language: None,
                location: None,
                marketing_permissions: None,
                merge_fields: None,
                tags: None,
                timestamp_opt: None,
                timestamp_signup: None,
                vip: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**email_address:** `String` — Email address for a subscriber.
    
</dd>
</dl>

<dl>
<dd>

**email_type:** `Option<String>` — Type of email this member asked to get ('html' or 'text').
    
</dd>
</dl>

<dl>
<dd>

**interests:** `Option<std::collections::HashMap<String, bool>>` — The key of this object's properties is the ID of the interest in question.
    
</dd>
</dl>

<dl>
<dd>

**ip_opt:** `Option<String>` — The IP address the subscriber used to confirm their opt-in status.
    
</dd>
</dl>

<dl>
<dd>

**ip_signup:** `Option<String>` — IP address the subscriber signed up from.
    
</dd>
</dl>

<dl>
<dd>

**language:** `Option<String>` — If set/detected, the [subscriber's language](https://mailchimp.com/help/view-and-edit-contact-languages/).
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<CreateMemberListsRequestLocation>` — Subscriber location information.
    
</dd>
</dl>

<dl>
<dd>

**marketing_permissions:** `Option<Vec<CreateMemberListsRequestMarketingPermissionsItem>>` — The marketing permissions for the subscriber.
    
</dd>
</dl>

<dl>
<dd>

**merge_fields:** `Option<std::collections::HashMap<String, CreateMemberListsRequestMergeFieldsValue>>` — A dictionary of merge fields where the keys are the merge tags. See the [Merge Fields documentation](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for more about the structure.
    
</dd>
</dl>

<dl>
<dd>

**status:** `CreateMemberListsRequestStatus` — Subscriber's current status.
    
</dd>
</dl>

<dl>
<dd>

**tags:** `Option<Vec<String>>` — The tags that are associated with a member.
    
</dd>
</dl>

<dl>
<dd>

**timestamp_opt:** `Option<CreateMemberListsRequestTimestampOpt>` 
    
</dd>
</dl>

<dl>
<dd>

**timestamp_signup:** `Option<CreateMemberListsRequestTimestampSignup>` 
    
</dd>
</dl>

<dl>
<dd>

**vip:** `Option<bool>` — [VIP status](https://mailchimp.com/help/designate-and-send-to-vip-contacts/) for subscriber.
    
</dd>
</dl>

<dl>
<dd>

**skip_merge_validation:** `Option<bool>` — If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">get_member</a>(list_id: String, subscriber_hash: String) -> Result&lt;ListMembers, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific list member, including a currently subscribed, unsubscribed, or bounced member.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .get_member(
            &"list_id".to_string(),
            &"subscriber_hash".to_string(),
            &GetMemberQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">upsert_member</a>(list_id: String, subscriber_hash: String, request: UpsertMemberListsRequest, skip_merge_validation: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;ListMembers, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add or update a list member.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .upsert_member(
            &"list_id".to_string(),
            &"subscriber_hash".to_string(),
            &UpsertMemberListsRequest {
                email_address: "email_address".to_string(),
                skip_merge_validation: None,
                email_type: None,
                interests: None,
                ip_opt: None,
                ip_signup: None,
                language: None,
                location: None,
                marketing_permissions: None,
                merge_fields: None,
                status: None,
                status_if_new: None,
                tags: None,
                timestamp_opt: None,
                timestamp_signup: None,
                vip: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    
</dd>
</dl>

<dl>
<dd>

**email_address:** `String` — Email address for a subscriber. This value is required only if the email address is not already present on the list.
    
</dd>
</dl>

<dl>
<dd>

**email_type:** `Option<String>` — Type of email this member asked to get ('html' or 'text').
    
</dd>
</dl>

<dl>
<dd>

**interests:** `Option<std::collections::HashMap<String, bool>>` — The key of this object's properties is the ID of the interest in question.
    
</dd>
</dl>

<dl>
<dd>

**ip_opt:** `Option<String>` — The IP address the subscriber used to confirm their opt-in status.
    
</dd>
</dl>

<dl>
<dd>

**ip_signup:** `Option<String>` — IP address the subscriber signed up from.
    
</dd>
</dl>

<dl>
<dd>

**language:** `Option<String>` — If set/detected, the [subscriber's language](https://mailchimp.com/help/view-and-edit-contact-languages/).
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<UpsertMemberListsRequestLocation>` — Subscriber location information.
    
</dd>
</dl>

<dl>
<dd>

**marketing_permissions:** `Option<Vec<UpsertMemberListsRequestMarketingPermissionsItem>>` — The marketing permissions for the subscriber.
    
</dd>
</dl>

<dl>
<dd>

**merge_fields:** `Option<std::collections::HashMap<String, UpsertMemberListsRequestMergeFieldsValue>>` — A dictionary of merge fields where the keys are the merge tags. See the [Merge Fields documentation](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for more about the structure.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<UpsertMemberListsRequestStatus>` — Subscriber's current status.
    
</dd>
</dl>

<dl>
<dd>

**status_if_new:** `Option<UpsertMemberListsRequestStatusIfNew>` — Subscriber's status. This value is required only if the email address is not already present on the list.
    
</dd>
</dl>

<dl>
<dd>

**tags:** `Option<Vec<String>>` — The tags that are associated with a member.
    
</dd>
</dl>

<dl>
<dd>

**timestamp_opt:** `Option<UpsertMemberListsRequestTimestampOpt>` 
    
</dd>
</dl>

<dl>
<dd>

**timestamp_signup:** `Option<UpsertMemberListsRequestTimestampSignup>` 
    
</dd>
</dl>

<dl>
<dd>

**vip:** `Option<bool>` — [VIP status](https://mailchimp.com/help/designate-and-send-to-vip-contacts/) for subscriber.
    
</dd>
</dl>

<dl>
<dd>

**skip_merge_validation:** `Option<bool>` — If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">delete_member</a>(list_id: String, subscriber_hash: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Archive a list member. To permanently delete, use the delete-permanent action.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .delete_member(&"list_id".to_string(), &"subscriber_hash".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">update_member</a>(list_id: String, subscriber_hash: String, request: UpdateMemberListsRequest, skip_merge_validation: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;ListMembers, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update information for a specific list member.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .update_member(
            &"list_id".to_string(),
            &"subscriber_hash".to_string(),
            &UpdateMemberListsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    
</dd>
</dl>

<dl>
<dd>

**email_address:** `Option<String>` — Email address for a subscriber.
    
</dd>
</dl>

<dl>
<dd>

**email_type:** `Option<String>` — Type of email this member asked to get ('html' or 'text').
    
</dd>
</dl>

<dl>
<dd>

**interests:** `Option<std::collections::HashMap<String, bool>>` — The key of this object's properties is the ID of the interest in question.
    
</dd>
</dl>

<dl>
<dd>

**ip_opt:** `Option<String>` — The IP address the subscriber used to confirm their opt-in status.
    
</dd>
</dl>

<dl>
<dd>

**ip_signup:** `Option<String>` — IP address the subscriber signed up from.
    
</dd>
</dl>

<dl>
<dd>

**language:** `Option<String>` — If set/detected, the [subscriber's language](https://mailchimp.com/help/view-and-edit-contact-languages/).
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<UpdateMemberListsRequestLocation>` — Subscriber location information.
    
</dd>
</dl>

<dl>
<dd>

**marketing_permissions:** `Option<Vec<UpdateMemberListsRequestMarketingPermissionsItem>>` — The marketing permissions for the subscriber.
    
</dd>
</dl>

<dl>
<dd>

**merge_fields:** `Option<std::collections::HashMap<String, UpdateMemberListsRequestMergeFieldsValue>>` — A dictionary of merge fields where the keys are the merge tags. See the [Merge Fields documentation](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for more about the structure.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<UpdateMemberListsRequestStatus>` — Subscriber's current status.
    
</dd>
</dl>

<dl>
<dd>

**timestamp_opt:** `Option<UpdateMemberListsRequestTimestampOpt>` 
    
</dd>
</dl>

<dl>
<dd>

**timestamp_signup:** `Option<UpdateMemberListsRequestTimestampSignup>` 
    
</dd>
</dl>

<dl>
<dd>

**vip:** `Option<bool>` — [VIP status](https://mailchimp.com/help/designate-and-send-to-vip-contacts/) for subscriber.
    
</dd>
</dl>

<dl>
<dd>

**skip_merge_validation:** `Option<bool>` — If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">create_member_action_delete_permanent</a>(list_id: String, subscriber_hash: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete all personally identifiable information related to a list member, and remove them from a list. This will make it impossible to re-import the list member.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .create_member_action_delete_permanent(
            &"list_id".to_string(),
            &"subscriber_hash".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_member_activity</a>(list_id: String, subscriber_hash: String) -> Result&lt;ListMemberActivityListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the last 50 events of a member's activity on a specific list, including opens, clicks, and unsubscribes.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_member_activity(
            &"list_id".to_string(),
            &"subscriber_hash".to_string(),
            &ListMemberActivityQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                action: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**action:** `Option<ListMemberActivityListsRequestActionItem>` — A comma seperated list of actions to return.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_member_activity_feed</a>(list_id: String, subscriber_hash: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListMemberActivityFeedListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a member's activity on a specific list, including opens, clicks, and unsubscribes.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_member_activity_feed(
            &"list_id".to_string(),
            &"subscriber_hash".to_string(),
            &ListMemberActivityFeedQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                activity_filters: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**activity_filters:** `Option<ListMemberActivityFeedListsRequestActivityFiltersItem>` — A comma-separated list of activity filters that correspond to a set of activity types, e.g "?activity_filters=open,bounce,click".
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_member_events</a>(list_id: String, subscriber_hash: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListMemberEventsListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get events for a contact.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_member_events(
            &"list_id".to_string(),
            &"subscriber_hash".to_string(),
            &ListMemberEventsQueryRequest {
                count: None,
                offset: None,
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">create_member_event</a>(list_id: String, subscriber_hash: String, request: CreateMemberEventListsRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add an event for a list member.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .create_member_event(
            &"list_id".to_string(),
            &"subscriber_hash".to_string(),
            &CreateMemberEventListsRequest {
                name: "name".to_string(),
                is_syncing: None,
                occurred_at: None,
                properties: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    
</dd>
</dl>

<dl>
<dd>

**is_syncing:** `Option<bool>` — Events created with the is_syncing value set to `true` will not trigger automations.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — The name for this type of event ('purchased', 'visited', etc). Must be 2-30 characters in length
    
</dd>
</dl>

<dl>
<dd>

**occurred_at:** `Option<String>` — The date and time the event occurred in ISO 8601 format.
    
</dd>
</dl>

<dl>
<dd>

**properties:** `Option<std::collections::HashMap<String, String>>` — An optional list of properties
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_member_goals</a>(list_id: String, subscriber_hash: String) -> Result&lt;ListMemberGoalsListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the last 50 Goal events for a member on a specific list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_member_goals(
            &"list_id".to_string(),
            &"subscriber_hash".to_string(),
            &ListMemberGoalsQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_member_notes</a>(list_id: String, subscriber_hash: String, sort_field: Option&lt;Option&lt;ListMemberNotesListsRequestSortField&gt;&gt;, sort_dir: Option&lt;Option&lt;ListMemberNotesListsRequestSortDir&gt;&gt;, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListMemberNotesListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get recent notes for a specific list member.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_member_notes(
            &"list_id".to_string(),
            &"subscriber_hash".to_string(),
            &ListMemberNotesQueryRequest {
                sort_field: None,
                sort_dir: None,
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address.
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<ListMemberNotesListsRequestSortField>` — Returns notes sorted by the specified field.
    
</dd>
</dl>

<dl>
<dd>

**sort_dir:** `Option<ListMemberNotesListsRequestSortDir>` — Determines the order direction for sorted results.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">create_member_note</a>(list_id: String, subscriber_hash: String, request: CreateMemberNoteListsRequest) -> Result&lt;MemberNotes, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a new note for a specific subscriber.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .create_member_note(
            &"list_id".to_string(),
            &"subscriber_hash".to_string(),
            &CreateMemberNoteListsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address.
    
</dd>
</dl>

<dl>
<dd>

**note:** `Option<String>` — The content of the note. Note length is limited to 1,000 characters.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">get_member_note</a>(list_id: String, subscriber_hash: String, note_id: String) -> Result&lt;MemberNotes, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a specific note for a specific list member.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .get_member_note(
            &"list_id".to_string(),
            &"subscriber_hash".to_string(),
            &"note_id".to_string(),
            &GetMemberNoteQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    
</dd>
</dl>

<dl>
<dd>

**note_id:** `String` — The id for the note.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">delete_member_note</a>(list_id: String, subscriber_hash: String, note_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a specific note for a specific list member.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .delete_member_note(
            &"list_id".to_string(),
            &"subscriber_hash".to_string(),
            &"note_id".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    
</dd>
</dl>

<dl>
<dd>

**note_id:** `String` — The id for the note.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">update_member_note</a>(list_id: String, subscriber_hash: String, note_id: String, request: UpdateMemberNoteListsRequest) -> Result&lt;MemberNotes, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a specific note for a specific list member.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .update_member_note(
            &"list_id".to_string(),
            &"subscriber_hash".to_string(),
            &"note_id".to_string(),
            &UpdateMemberNoteListsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    
</dd>
</dl>

<dl>
<dd>

**note_id:** `String` — The id for the note.
    
</dd>
</dl>

<dl>
<dd>

**note:** `Option<String>` — The content of the note. Note length is limited to 1,000 characters.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_member_tags</a>(list_id: String, subscriber_hash: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListMemberTagsListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the tags on a list member.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_member_tags(
            &"list_id".to_string(),
            &"subscriber_hash".to_string(),
            &ListMemberTagsQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address. This endpoint also accepts a list member's email address or contact_id.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">create_member_tag</a>(list_id: String, subscriber_hash: String, request: CreateMemberTagListsRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add or remove tags from a list member. If a tag that does not exist is passed in and set as 'active', a new tag will be created.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .create_member_tag(
            &"list_id".to_string(),
            &"subscriber_hash".to_string(),
            &CreateMemberTagListsRequest {
                tags: vec![CreateMemberTagListsRequestTagsItem {
                    name: "name".to_string(),
                    status: CreateMemberTagListsRequestTagsItemStatus::Inactive,
                }],
                is_syncing: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address.
    
</dd>
</dl>

<dl>
<dd>

**is_syncing:** `Option<bool>` — When is_syncing is true, automations based on the tags in the request will not fire
    
</dd>
</dl>

<dl>
<dd>

**tags:** `Vec<CreateMemberTagListsRequestTagsItem>` — A list of tags assigned to the list member.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_merge_fields</a>(list_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, type_: Option&lt;Option&lt;String&gt;&gt;, required: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;ListMergeFieldsListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of all merge fields for an audience.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_merge_fields(
            &"list_id".to_string(),
            &ListMergeFieldsQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                r#type: None,
                required: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<String>` — The merge field type.
    
</dd>
</dl>

<dl>
<dd>

**required:** `Option<bool>` — Whether it's a required merge field.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">create_merge_field</a>(list_id: String, request: CreateMergeFieldListsRequest) -> Result&lt;MergeField, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a new merge field for a specific audience.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .create_merge_field(
            &"list_id".to_string(),
            &CreateMergeFieldListsRequest {
                name: "name".to_string(),
                r#type: CreateMergeFieldListsRequestType::Text,
                default_value: None,
                display_order: None,
                help_text: None,
                options: None,
                public: None,
                required: None,
                tag: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**default_value:** `Option<String>` — The default value for the merge field if `null`.
    
</dd>
</dl>

<dl>
<dd>

**display_order:** `Option<i64>` — The order that the merge field displays on the list signup form.
    
</dd>
</dl>

<dl>
<dd>

**help_text:** `Option<String>` — Extra text to help the subscriber fill out the form.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — The name of the merge field (audience field).
    
</dd>
</dl>

<dl>
<dd>

**options:** `Option<CreateMergeFieldListsRequestOptions>` — Extra options for some merge field types.
    
</dd>
</dl>

<dl>
<dd>

**public:** `Option<bool>` — Whether the merge field is displayed on the signup form.
    
</dd>
</dl>

<dl>
<dd>

**required:** `Option<bool>` — Whether the merge field is required to import a contact.
    
</dd>
</dl>

<dl>
<dd>

**tag:** `Option<String>` — The merge tag used for Mailchimp campaigns and [adding contact information](https://mailchimp.com/developer/marketing/docs/merge-fields/#add-merge-data-to-contacts).
    
</dd>
</dl>

<dl>
<dd>

**type_:** `CreateMergeFieldListsRequestType` — The [type](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for the merge field.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">get_merge_field</a>(list_id: String, merge_id: String) -> Result&lt;MergeField, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific merge field.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .get_merge_field(
            &"list_id".to_string(),
            &"merge_id".to_string(),
            &GetMergeFieldQueryRequest {
                exclude_fields: vec![],
                fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**merge_id:** `String` — The id for the merge field.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">delete_merge_field</a>(list_id: String, merge_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a specific merge field.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .delete_merge_field(&"list_id".to_string(), &"merge_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**merge_id:** `String` — The id for the merge field.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">update_merge_field</a>(list_id: String, merge_id: String, request: UpdateMergeFieldListsRequest) -> Result&lt;MergeField, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a specific merge field.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .update_merge_field(
            &"list_id".to_string(),
            &"merge_id".to_string(),
            &UpdateMergeFieldListsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**merge_id:** `String` — The id for the merge field.
    
</dd>
</dl>

<dl>
<dd>

**default_value:** `Option<String>` — The default value for the merge field if `null`.
    
</dd>
</dl>

<dl>
<dd>

**display_order:** `Option<i64>` — The order that the merge field displays on the list signup form.
    
</dd>
</dl>

<dl>
<dd>

**help_text:** `Option<String>` — Extra text to help the subscriber fill out the form.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — The name of the merge field (audience field).
    
</dd>
</dl>

<dl>
<dd>

**options:** `Option<UpdateMergeFieldListsRequestOptions>` — Extra options for some merge field types.
    
</dd>
</dl>

<dl>
<dd>

**public:** `Option<bool>` — Whether the merge field is displayed on the signup form.
    
</dd>
</dl>

<dl>
<dd>

**required:** `Option<bool>` — Whether the merge field is required to import a contact.
    
</dd>
</dl>

<dl>
<dd>

**tag:** `Option<String>` — The merge tag used for Mailchimp campaigns and [adding contact information](https://mailchimp.com/developer/marketing/docs/merge-fields/#add-merge-data-to-contacts).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_segments</a>(list_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, type_: Option&lt;Option&lt;String&gt;&gt;, since_created_at: Option&lt;Option&lt;String&gt;&gt;, before_created_at: Option&lt;Option&lt;String&gt;&gt;, include_cleaned: Option&lt;Option&lt;bool&gt;&gt;, include_transactional: Option&lt;Option&lt;bool&gt;&gt;, include_unsubscribed: Option&lt;Option&lt;bool&gt;&gt;, since_updated_at: Option&lt;Option&lt;String&gt;&gt;, before_updated_at: Option&lt;Option&lt;String&gt;&gt;, exclude_type: Option&lt;Option&lt;ListSegmentsListsRequestExcludeType&gt;&gt;) -> Result&lt;ListSegmentsListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about all available segments for a specific list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_segments(
            &"list_id".to_string(),
            &ListSegmentsQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                r#type: None,
                since_created_at: None,
                before_created_at: None,
                include_cleaned: None,
                include_transactional: None,
                include_unsubscribed: None,
                since_updated_at: None,
                before_updated_at: None,
                exclude_type: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<String>` — Limit results based on segment type.
    
</dd>
</dl>

<dl>
<dd>

**since_created_at:** `Option<String>` — Restrict results to segments created after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**before_created_at:** `Option<String>` — Restrict results to segments created before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**include_cleaned:** `Option<bool>` — Include cleaned members in response
    
</dd>
</dl>

<dl>
<dd>

**include_transactional:** `Option<bool>` — Include transactional members in response
    
</dd>
</dl>

<dl>
<dd>

**include_unsubscribed:** `Option<bool>` — Include unsubscribed members in response
    
</dd>
</dl>

<dl>
<dd>

**since_updated_at:** `Option<String>` — Restrict results to segments update after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**before_updated_at:** `Option<String>` — Restrict results to segments update before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**exclude_type:** `Option<ListSegmentsListsRequestExcludeType>` — Exclude results based on segment type. For example, use `exclude_type=static` to exclude tags from the response.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">create_segment</a>(list_id: String, request: CreateSegmentListsRequest) -> Result&lt;List, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new segment in a specific list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .create_segment(
            &"list_id".to_string(),
            &CreateSegmentListsRequest {
                name: "name".to_string(),
                options: None,
                static_segment: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — The name of the segment.
    
</dd>
</dl>

<dl>
<dd>

**options:** `Option<CreateSegmentListsRequestOptions>` — The [conditions of the segment](https://mailchimp.com/help/save-and-manage-segments/). Static and fuzzy segments don't have conditions.
    
</dd>
</dl>

<dl>
<dd>

**static_segment:** `Option<Vec<String>>` — An array of emails to be used for a static segment. Any emails provided that are not present on the list will be ignored. Passing an empty array will create a static segment without any subscribers. This field cannot be provided with the options field.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">get_segment</a>(list_id: String, segment_id: String, include_cleaned: Option&lt;Option&lt;bool&gt;&gt;, include_transactional: Option&lt;Option&lt;bool&gt;&gt;, include_unsubscribed: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;List, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific segment.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .get_segment(
            &"list_id".to_string(),
            &"segment_id".to_string(),
            &GetSegmentQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                include_cleaned: None,
                include_transactional: None,
                include_unsubscribed: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**segment_id:** `String` — The unique id for the segment.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**include_cleaned:** `Option<bool>` — Include cleaned members in response
    
</dd>
</dl>

<dl>
<dd>

**include_transactional:** `Option<bool>` — Include transactional members in response
    
</dd>
</dl>

<dl>
<dd>

**include_unsubscribed:** `Option<bool>` — Include unsubscribed members in response
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">batch_add_or_remove_members</a>(list_id: String, segment_id: String, request: BatchAddOrRemoveMembersListsRequest) -> Result&lt;BatchAddOrRemoveMembersListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Batch add/remove list members to static segment
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .batch_add_or_remove_members(
            &"list_id".to_string(),
            &"segment_id".to_string(),
            &BatchAddOrRemoveMembersListsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**segment_id:** `String` — The unique id for the segment.
    
</dd>
</dl>

<dl>
<dd>

**members_to_add:** `Option<Vec<String>>` — An array of emails to be used for a static segment. Any emails provided that are not present on the list will be ignored. A maximum of 500 members can be sent.
    
</dd>
</dl>

<dl>
<dd>

**members_to_remove:** `Option<Vec<String>>` — An array of emails to be used for a static segment. Any emails provided that are not present on the list will be ignored. A maximum of 500 members can be sent.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">delete_segment</a>(list_id: String, segment_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a specific segment in a list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .delete_segment(&"list_id".to_string(), &"segment_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**segment_id:** `String` — The unique id for the segment.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">update_segment</a>(list_id: String, segment_id: String, request: UpdateSegmentListsRequest) -> Result&lt;List, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a specific segment in a list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .update_segment(
            &"list_id".to_string(),
            &"segment_id".to_string(),
            &UpdateSegmentListsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**segment_id:** `String` — The unique id for the segment.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — The name of the segment.
    
</dd>
</dl>

<dl>
<dd>

**options:** `Option<UpdateSegmentListsRequestOptions>` — The [conditions of the segment](https://mailchimp.com/help/save-and-manage-segments/). Static and fuzzy segments don't have conditions.
    
</dd>
</dl>

<dl>
<dd>

**static_segment:** `Option<Vec<String>>` — An array of emails to be used for a static segment. Any emails provided that are not present on the list will be ignored. Passing an empty array for an existing static segment will reset that segment and remove all members. This field cannot be provided with the `options` field.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_segment_members</a>(list_id: String, segment_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, include_cleaned: Option&lt;Option&lt;bool&gt;&gt;, include_transactional: Option&lt;Option&lt;bool&gt;&gt;, include_unsubscribed: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;ListSegmentMembersListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about members in a saved segment.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_segment_members(
            &"list_id".to_string(),
            &"segment_id".to_string(),
            &ListSegmentMembersQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                include_cleaned: None,
                include_transactional: None,
                include_unsubscribed: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**segment_id:** `String` — The unique id for the segment.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**include_cleaned:** `Option<bool>` — Include cleaned members in response
    
</dd>
</dl>

<dl>
<dd>

**include_transactional:** `Option<bool>` — Include transactional members in response
    
</dd>
</dl>

<dl>
<dd>

**include_unsubscribed:** `Option<bool>` — Include unsubscribed members in response
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">create_segment_member</a>(list_id: String, segment_id: String, request: CreateSegmentMemberListsRequest) -> Result&lt;ListsSegmentsMembers, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a member to a static segment.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .create_segment_member(
            &"list_id".to_string(),
            &"segment_id".to_string(),
            &CreateSegmentMemberListsRequest {
                email_address: "email_address".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**segment_id:** `String` — The unique id for the segment.
    
</dd>
</dl>

<dl>
<dd>

**email_address:** `String` — Email address for a subscriber.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">delete_segment_member</a>(list_id: String, segment_id: String, subscriber_hash: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Remove a member from the specified static segment.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .delete_segment_member(
            &"list_id".to_string(),
            &"segment_id".to_string(),
            &"subscriber_hash".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**segment_id:** `String` — The unique id for the segment.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_signup_forms</a>(list_id: String) -> Result&lt;ListSignupFormsListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get signup forms for a specific list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_signup_forms(&"list_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">create_signup_form</a>(list_id: String, request: CreateSignupFormListsRequest) -> Result&lt;SignupForm, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Customize a list's default signup form.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .create_signup_form(
            &"list_id".to_string(),
            &CreateSignupFormListsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**contents:** `Option<Vec<CreateSignupFormListsRequestContentsItem>>` — The signup form body content.
    
</dd>
</dl>

<dl>
<dd>

**header:** `Option<CreateSignupFormListsRequestHeader>` — Options for customizing your signup form header.
    
</dd>
</dl>

<dl>
<dd>

**styles:** `Option<Vec<CreateSignupFormListsRequestStylesItem>>` — An array of objects, each representing an element style for the signup form.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_surveys</a>(list_id: String) -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about all available surveys for a specific list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_surveys(&"list_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">create_survey</a>(list_id: String, request: CreateSurveyListsRequest) -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a draft survey for an audience.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .create_survey(
            &"list_id".to_string(),
            &CreateSurveyListsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The title of the survey.
    
</dd>
</dl>

<dl>
<dd>

**sections:** `Option<Vec<SurveySectionRequest>>` — Initial survey sections.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">get_survey</a>(list_id: String, survey_id: String) -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get details about a specific survey.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .get_survey(&"list_id".to_string(), &"survey_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**survey_id:** `String` — The ID of the survey.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">delete_survey</a>(list_id: String, survey_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a survey.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .delete_survey(&"list_id".to_string(), &"survey_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**survey_id:** `String` — The ID of the survey.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">update_survey</a>(list_id: String, survey_id: String, request: UpdateSurveyListsRequest) -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a survey. When sections is provided, send the complete section list in display order. Any existing section not included is deleted.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .update_survey(
            &"list_id".to_string(),
            &"survey_id".to_string(),
            &UpdateSurveyListsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**survey_id:** `String` — The ID of the survey.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The title of the survey.
    
</dd>
</dl>

<dl>
<dd>

**is_piped_to_inbox:** `Option<bool>` — Whether responses are sent to Mailchimp Inbox.
    
</dd>
</dl>

<dl>
<dd>

**sections:** `Option<Vec<SurveySectionRequest>>` — The complete survey section list in display order. On update, sections omitted from this array are deleted. Include section id to update an existing section; omit section id to add a new section.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">create_list_survey_action_replicate</a>(list_id_path_param: String, survey_id: String, request: CreateListSurveyActionReplicateListsRequest) -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Replicate a survey.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .create_list_survey_action_replicate(
            &"list_id".to_string(),
            &"survey_id".to_string(),
            &CreateListSurveyActionReplicateListsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id_path_param:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**survey_id:** `String` — The ID of the survey.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The title for the replicated survey.
    
</dd>
</dl>

<dl>
<dd>

**list_id:** `Option<String>` — The unique ID of the audience for the replicated survey. Defaults to the source survey audience.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_tag_search</a>(list_id: String, name: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListTagSearchListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Search for tags on a list by name. If no name is provided, will return all tags on the list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_tag_search(
            &"list_id".to_string(),
            &ListTagSearchQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — The search query used to filter tags.  The search query will be compared to each tag as a prefix, so all tags that have a name starting with this field will be returned.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list_webhooks</a>(list_id: String) -> Result&lt;ListWebhooksListsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about all webhooks for a specific list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .list_webhooks(&"list_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">create_webhook</a>(list_id: String, request: AddWebhook) -> Result&lt;ListWebhooks, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new webhook for a specific list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .create_webhook(
            &"list_id".to_string(),
            &AddWebhook {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">get_webhook</a>(list_id: String, webhook_id: String) -> Result&lt;ListWebhooks, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific webhook.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .get_webhook(&"list_id".to_string(), &"webhook_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**webhook_id:** `String` — The webhook's id.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">delete_webhook</a>(list_id: String, webhook_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a specific webhook in a list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .delete_webhook(&"list_id".to_string(), &"webhook_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**webhook_id:** `String` — The webhook's id.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">update_webhook</a>(list_id: String, webhook_id: String, request: AddWebhook) -> Result&lt;ListWebhooks, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update the settings for an existing webhook.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .lists
        .update_webhook(
            &"list_id".to_string(),
            &"webhook_id".to_string(),
            &AddWebhook {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**webhook_id:** `String` — The webhook's id.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## surveys
<details><summary><code>client.surveys.<a href="/src/api/resources/surveys/client.rs">create_list_survey_action_create_email</a>(list_id: String, survey_id: String) -> Result&lt;Campaign, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Utilize the List ID and Survey ID to generate a Campaign that links to your survey.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .surveys
        .create_list_survey_action_create_email(
            &"list_id".to_string(),
            &"survey_id".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**survey_id:** `String` — The ID of the survey.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.surveys.<a href="/src/api/resources/surveys/client.rs">create_list_survey_action_publish</a>(list_id: String, survey_id: String) -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Publish a survey that is in draft, unpublished, or has been previously published and edited.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .surveys
        .create_list_survey_action_publish(&"list_id".to_string(), &"survey_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**survey_id:** `String` — The ID of the survey.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.surveys.<a href="/src/api/resources/surveys/client.rs">create_list_survey_action_unpublish</a>(list_id: String, survey_id: String) -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Unpublish a survey that has been published.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .surveys
        .create_list_survey_action_unpublish(&"list_id".to_string(), &"survey_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**list_id:** `String` — The unique ID for the list.
    
</dd>
</dl>

<dl>
<dd>

**survey_id:** `String` — The ID of the survey.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ping
<details><summary><code>client.ping.<a href="/src/api/resources/ping/client.rs">list</a>() -> Result&lt;ListPingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

A health check for the API that won't return any account-specific information.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client.ping.list(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## reporting
<details><summary><code>client.reporting.<a href="/src/api/resources/reporting/client.rs">list</a>() -> Result&lt;Vec&lt;ListReportingResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about the reporting endpoint's resources.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client.reporting.list(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reporting.<a href="/src/api/resources/reporting/client.rs">list_facebook_ads</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, sort_field: Option&lt;Option&lt;ListFacebookAdsReportingRequestSortField&gt;&gt;, sort_dir: Option&lt;Option&lt;ListFacebookAdsReportingRequestSortDir&gt;&gt;) -> Result&lt;ListFacebookAdsReportingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get reports of Facebook ads.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reporting
        .list_facebook_ads(
            &ListFacebookAdsQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                sort_field: None,
                sort_dir: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<ListFacebookAdsReportingRequestSortField>` — Returns files sorted by the specified field.
    
</dd>
</dl>

<dl>
<dd>

**sort_dir:** `Option<ListFacebookAdsReportingRequestSortDir>` — Determines the order direction for sorted results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reporting.<a href="/src/api/resources/reporting/client.rs">get_facebook_ad</a>(outreach_id: String) -> Result&lt;ReportingFacebookAd, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get report of a Facebook ad.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reporting
        .get_facebook_ad(
            &"outreach_id".to_string(),
            &GetFacebookAdQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**outreach_id:** `String` — The outreach id.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reporting.<a href="/src/api/resources/reporting/client.rs">list_facebook_ad_ecommerce_product_activity</a>(outreach_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, sort_field: Option&lt;Option&lt;ListFacebookAdEcommerceProductActivityReportingRequestSortField&gt;&gt;) -> Result&lt;ListFacebookAdEcommerceProductActivityReportingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get breakdown of product activity for an outreach.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reporting
        .list_facebook_ad_ecommerce_product_activity(
            &"outreach_id".to_string(),
            &ListFacebookAdEcommerceProductActivityQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                sort_field: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**outreach_id:** `String` — The outreach id.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<ListFacebookAdEcommerceProductActivityReportingRequestSortField>` — Returns files sorted by the specified field.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reporting.<a href="/src/api/resources/reporting/client.rs">list_landing_pages</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListLandingPagesReportingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get reports of landing pages.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reporting
        .list_landing_pages(
            &ListLandingPagesQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reporting.<a href="/src/api/resources/reporting/client.rs">get_landing_page</a>(outreach_id: String) -> Result&lt;LandingPageReport, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get report of a landing page.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reporting
        .get_landing_page(
            &"outreach_id".to_string(),
            &GetLandingPageQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**outreach_id:** `String` — The outreach id.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reporting.<a href="/src/api/resources/reporting/client.rs">list_surveys</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListSurveysReportingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get reports for surveys.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reporting
        .list_surveys(
            &ListSurveysQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reporting.<a href="/src/api/resources/reporting/client.rs">get_survey</a>(survey_id: String) -> Result&lt;GetSurveyReportingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get report for a survey.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reporting
        .get_survey(
            &"survey_id".to_string(),
            &GetSurveyQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**survey_id:** `String` — The ID of the survey.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reporting.<a href="/src/api/resources/reporting/client.rs">list_survey_questions</a>(survey_id: String) -> Result&lt;ListSurveyQuestionsReportingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get reports for survey questions.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reporting
        .list_survey_questions(
            &"survey_id".to_string(),
            &ListSurveyQuestionsQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**survey_id:** `String` — The ID of the survey.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reporting.<a href="/src/api/resources/reporting/client.rs">get_survey_question</a>(survey_id: String, question_id: String) -> Result&lt;SurveyQuestionReport, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get report for a survey question.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reporting
        .get_survey_question(
            &"survey_id".to_string(),
            &"question_id".to_string(),
            &GetSurveyQuestionQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**survey_id:** `String` — The ID of the survey.
    
</dd>
</dl>

<dl>
<dd>

**question_id:** `String` — The ID of the survey question.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reporting.<a href="/src/api/resources/reporting/client.rs">list_survey_question_answers</a>(survey_id: String, question_id: String, respondent_familiarity_is: Option&lt;Option&lt;ListSurveyQuestionAnswersReportingRequestRespondentFamiliarityIs&gt;&gt;) -> Result&lt;ListSurveyQuestionAnswersReportingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get answers for a survey question.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reporting
        .list_survey_question_answers(
            &"survey_id".to_string(),
            &"question_id".to_string(),
            &ListSurveyQuestionAnswersQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                respondent_familiarity_is: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**survey_id:** `String` — The ID of the survey.
    
</dd>
</dl>

<dl>
<dd>

**question_id:** `String` — The ID of the survey question.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**respondent_familiarity_is:** `Option<ListSurveyQuestionAnswersReportingRequestRespondentFamiliarityIs>` — Filter survey responses by familiarity of the respondents.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reporting.<a href="/src/api/resources/reporting/client.rs">list_survey_responses</a>(survey_id: String, answered_question: Option&lt;Option&lt;i64&gt;&gt;, chose_answer: Option&lt;Option&lt;String&gt;&gt;, respondent_familiarity_is: Option&lt;Option&lt;ListSurveyResponsesReportingRequestRespondentFamiliarityIs&gt;&gt;) -> Result&lt;ListSurveyResponsesReportingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get responses to a survey.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reporting
        .list_survey_responses(
            &"survey_id".to_string(),
            &ListSurveyResponsesQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                answered_question: None,
                chose_answer: None,
                respondent_familiarity_is: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**survey_id:** `String` — The ID of the survey.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**answered_question:** `Option<i64>` — The ID of the question that was answered.
    
</dd>
</dl>

<dl>
<dd>

**chose_answer:** `Option<String>` — The ID of the option chosen to filter responses on.
    
</dd>
</dl>

<dl>
<dd>

**respondent_familiarity_is:** `Option<ListSurveyResponsesReportingRequestRespondentFamiliarityIs>` — Filter survey responses by familiarity of the respondents.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reporting.<a href="/src/api/resources/reporting/client.rs">get_survey_respons</a>(survey_id: String, response_id: String) -> Result&lt;GetSurveyResponsReportingResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a single survey response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reporting
        .get_survey_respons(&"survey_id".to_string(), &"response_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**survey_id:** `String` — The ID of the survey.
    
</dd>
</dl>

<dl>
<dd>

**response_id:** `String` — The ID of the survey response.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## reports
<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">list</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, type_: Option&lt;Option&lt;ListReportsRequestType&gt;&gt;, before_send_time: Option&lt;Option&lt;String&gt;&gt;, since_send_time: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get campaign reports.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .list(
            &ReportsListQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                r#type: None,
                before_send_time: None,
                since_send_time: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<ListReportsRequestType>` — The campaign type.
    
</dd>
</dl>

<dl>
<dd>

**before_send_time:** `Option<String>` — Restrict the response to campaigns sent before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**since_send_time:** `Option<String>` — Restrict the response to campaigns sent after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">get</a>(campaign_id: String) -> Result&lt;CampaignReport, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get report details for a specific sent campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .get(
            &"campaign_id".to_string(),
            &ReportsGetQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">list_abuse_reports</a>(campaign_id: String) -> Result&lt;ListAbuseReportsReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of abuse complaints for a specific campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .list_abuse_reports(
            &"campaign_id".to_string(),
            &ReportsListAbuseReportsQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">get_abuse_report</a>(campaign_id: String, report_id: String) -> Result&lt;AbuseComplaint, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific abuse report for a campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .get_abuse_report(
            &"campaign_id".to_string(),
            &"report_id".to_string(),
            &ReportsGetAbuseReportQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**report_id:** `String` — The id for the abuse report.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">list_advice</a>(campaign_id: String) -> Result&lt;ListAdviceReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get feedback based on a campaign's statistics. Advice feedback is based on campaign stats like opens, clicks, unsubscribes, bounces, and more.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .list_advice(
            &"campaign_id".to_string(),
            &ListAdviceQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">list_click_details</a>(campaign_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, sort_field: Option&lt;Option&lt;ListClickDetailsReportsRequestSortField&gt;&gt;, sort_dir: Option&lt;Option&lt;ListClickDetailsReportsRequestSortDir&gt;&gt;, filter_bots: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;ListClickDetailsReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about clicks on specific links in your Mailchimp campaigns.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .list_click_details(
            &"campaign_id".to_string(),
            &ListClickDetailsQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                sort_field: None,
                sort_dir: None,
                filter_bots: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<ListClickDetailsReportsRequestSortField>` — Returns click reports sorted by the specified field.
    
</dd>
</dl>

<dl>
<dd>

**sort_dir:** `Option<ListClickDetailsReportsRequestSortDir>` — Determines the order direction for sorted results.
    
</dd>
</dl>

<dl>
<dd>

**filter_bots:** `Option<bool>` — When true, exclude automated bot clicks so the returned click counts reflect human clicks only, matching the in-app Recipient Activity view. Filtering changes a link's counts, but never removes a link from the response. Defaults to false (all clicks).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">get_click_detail</a>(campaign_id: String, link_id: String, filter_bots: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;ClickDetailReport, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get click details for a specific link in a campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .get_click_detail(
            &"campaign_id".to_string(),
            &"link_id".to_string(),
            &GetClickDetailQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                filter_bots: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**link_id:** `String` — The id for the link.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**filter_bots:** `Option<bool>` — When true, exclude automated bot clicks so the returned click counts reflect human clicks only, matching the in-app Recipient Activity view. Filtering changes a link's counts, but never removes a link from the response. Defaults to false (all clicks).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">list_click_detail_members</a>(campaign_id: String, link_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListClickDetailMembersReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about list members who clicked on a specific link in a campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .list_click_detail_members(
            &"campaign_id".to_string(),
            &"link_id".to_string(),
            &ListClickDetailMembersQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**link_id:** `String` — The id for the link.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">get_click_detail_member</a>(campaign_id: String, link_id: String, subscriber_hash: String) -> Result&lt;ClickDetailMember, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific subscriber who clicked a link in a specific campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .get_click_detail_member(
            &"campaign_id".to_string(),
            &"link_id".to_string(),
            &"subscriber_hash".to_string(),
            &GetClickDetailMemberQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**link_id:** `String` — The id for the link.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">list_domain_performance</a>(campaign_id: String) -> Result&lt;ListDomainPerformanceReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get statistics for the top-performing email domains in a campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .list_domain_performance(
            &"campaign_id".to_string(),
            &ListDomainPerformanceQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">list_ecommerce_product_activity</a>(campaign_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, sort_field: Option&lt;Option&lt;ListEcommerceProductActivityReportsRequestSortField&gt;&gt;) -> Result&lt;ListEcommerceProductActivityReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get breakdown of product activity for a campaign
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .list_ecommerce_product_activity(
            &"campaign_id".to_string(),
            &ListEcommerceProductActivityQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                sort_field: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<ListEcommerceProductActivityReportsRequestSortField>` — Returns files sorted by the specified field.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">list_eepurl</a>(campaign_id: String) -> Result&lt;ListEepurlReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a summary of social activity for the campaign, tracked by EepURL.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .list_eepurl(
            &"campaign_id".to_string(),
            &ListEepurlQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">list_email_activity</a>(campaign_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, since: Option&lt;Option&lt;String&gt;&gt;, filter_bots: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;ListEmailActivityReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of member's subscriber activity in a specific campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .list_email_activity(
            &"campaign_id".to_string(),
            &ListEmailActivityQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                since: None,
                filter_bots: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**since:** `Option<String>` — Restrict results to email activity events that occur after a specific time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**filter_bots:** `Option<bool>` — When true, exclude automated bot and Apple Mail Privacy Protection (MPP) proxy activity so the returned activity reflects human-only opens and clicks, matching the in-app Recipient Activity view. Filtering removes events from a member's activity, but never removes the member from the response. Defaults to false (all activity).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">get_email_activity</a>(campaign_id: String, subscriber_hash: String, since: Option&lt;Option&lt;String&gt;&gt;, filter_bots: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;EmailActivity, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a specific list member's activity in a campaign including opens, clicks, and bounces.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .get_email_activity(
            &"campaign_id".to_string(),
            &"subscriber_hash".to_string(),
            &GetEmailActivityQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                since: None,
                filter_bots: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**since:** `Option<String>` — Restrict results to email activity events that occur after a specific time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**filter_bots:** `Option<bool>` — When true, exclude automated bot and Apple Mail Privacy Protection (MPP) proxy activity so the returned activity reflects human-only opens and clicks, matching the in-app Recipient Activity view. Filtering removes events from a member's activity, but never removes the member from the response. Defaults to false (all activity).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">list_locations</a>(campaign_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListLocationsReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get top open locations for a specific campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .list_locations(
            &"campaign_id".to_string(),
            &ReportsListLocationsQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">list_open_details</a>(campaign_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, since: Option&lt;Option&lt;String&gt;&gt;, sort_field: Option&lt;Option&lt;ListOpenDetailsReportsRequestSortField&gt;&gt;, sort_dir: Option&lt;Option&lt;ListOpenDetailsReportsRequestSortDir&gt;&gt;, filter_bots: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;ListOpenDetailsReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get detailed information about any campaign emails that were opened by a list member.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .list_open_details(
            &"campaign_id".to_string(),
            &ListOpenDetailsQueryRequest {
                since: Some("2016-04-12 12:00:00".to_string()),
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                sort_field: None,
                sort_dir: None,
                filter_bots: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**since:** `Option<String>` — Restrict results to campaign open events that occur after a specific time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<ListOpenDetailsReportsRequestSortField>` — Returns open reports sorted by the specified field.
    
</dd>
</dl>

<dl>
<dd>

**sort_dir:** `Option<ListOpenDetailsReportsRequestSortDir>` — Determines the order direction for sorted results.
    
</dd>
</dl>

<dl>
<dd>

**filter_bots:** `Option<bool>` — When true, exclude automated (proxy/bot) opens so the returned open counts reflect human opens only, matching the in-app Recipient Activity view. A member whose opens are all automated is excluded from the human-only view. Defaults to false (all opens).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">get_open_detail</a>(campaign_id: String, subscriber_hash: String, filter_bots: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;OpenActivity, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific subscriber who opened a campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .get_open_detail(
            &"campaign_id".to_string(),
            &"subscriber_hash".to_string(),
            &GetOpenDetailQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                filter_bots: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**filter_bots:** `Option<bool>` — When true, exclude automated (proxy/bot) opens so the returned open counts reflect human opens only, matching the in-app Recipient Activity view. A member whose opens are all automated is excluded from the human-only view. Defaults to false (all opens).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">list_sent_to</a>(campaign_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListSentToReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about campaign recipients.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .list_sent_to(
            &"campaign_id".to_string(),
            &ListSentToQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">get_sent_to</a>(campaign_id: String, subscriber_hash: String) -> Result&lt;SentTo, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific campaign recipient.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .get_sent_to(
            &"campaign_id".to_string(),
            &"subscriber_hash".to_string(),
            &GetSentToQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">list_sub_reports</a>(campaign_id: String) -> Result&lt;ListSubReportsReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of reports with child campaigns for a specific parent campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .list_sub_reports(
            &"campaign_id".to_string(),
            &ListSubReportsQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">list_unsubscribed</a>(campaign_id: String, count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListUnsubscribedReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about members who have unsubscribed from a specific campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .list_unsubscribed(
            &"campaign_id".to_string(),
            &ListUnsubscribedQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reports.<a href="/src/api/resources/reports/client.rs">get_unsubscribed</a>(campaign_id: String, subscriber_hash: String) -> Result&lt;Unsubscribes, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific list member who unsubscribed from a campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .reports
        .get_unsubscribed(
            &"campaign_id".to_string(),
            &"subscriber_hash".to_string(),
            &GetUnsubscribedQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**campaign_id:** `String` — The unique id for the campaign.
    
</dd>
</dl>

<dl>
<dd>

**subscriber_hash:** `String` — The MD5 hash of the lowercase version of the list member's email address.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## SearchCampaigns
<details><summary><code>client.search_campaigns.<a href="/src/api/resources/search_campaigns/client.rs">list</a>(query: Option&lt;String&gt;) -> Result&lt;ListSearchCampaignsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Search all campaigns for the specified query terms.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .search_campaigns
        .list(
            &SearchCampaignsListQueryRequest {
                query: "query".to_string(),
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**query:** `String` — The search query used to filter results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## SmsCampaigns
<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">list</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListSmsCampaignsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get all SMS campaigns in an account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .list(
            &SmsCampaignsListQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">create</a>(request: CreateSmsCampaignsRequest) -> Result&lt;SmsCampaign, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new SMS campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .create(
            &CreateSmsCampaignsRequest {
                name: "name".to_string(),
                list_id: None,
                folder_id: None,
                segments: None,
                excluded_segments: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` — The name of the campaign.
    
</dd>
</dl>

<dl>
<dd>

**list_id:** `Option<i64>` — The numeric ID of the list to send the campaign to.
    
</dd>
</dl>

<dl>
<dd>

**folder_id:** `Option<String>` — The ID of the folder to place this campaign in.
    
</dd>
</dl>

<dl>
<dd>

**segments:** `Option<Vec<i64>>` — The segment IDs to target for this campaign.
    
</dd>
</dl>

<dl>
<dd>

**excluded_segments:** `Option<Vec<i64>>` — The segment IDs to exclude from this campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">get</a>(sms_campaign_id: String) -> Result&lt;SmsCampaign, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the details for a single SMS campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .get(
            &"sms_campaign_id".to_string(),
            &SmsCampaignsGetQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sms_campaign_id:** `String` — The unique id for the SMS campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">delete</a>(sms_campaign_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Remove a campaign from your Mailchimp account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .delete(&"sms_campaign_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sms_campaign_id:** `String` — The unique id for the SMS campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">update</a>(sms_campaign_id: String, request: UpdateSmsCampaignsRequest) -> Result&lt;SmsCampaign, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update an SMS campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .update(
            &"sms_campaign_id".to_string(),
            &UpdateSmsCampaignsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sms_campaign_id:** `String` — The unique id for the SMS campaign.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — The name of the campaign.
    
</dd>
</dl>

<dl>
<dd>

**folder_id:** `Option<String>` — The ID of the folder to place this campaign in.
    
</dd>
</dl>

<dl>
<dd>

**segments:** `Option<Vec<i64>>` — The segment IDs to target for this campaign.
    
</dd>
</dl>

<dl>
<dd>

**excluded_segments:** `Option<Vec<i64>>` — The segment IDs to exclude from this campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">create_action_cancel_send</a>(sms_campaign_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cancel a scheduled or sending SMS campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .create_action_cancel_send(&"sms_campaign_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sms_campaign_id:** `String` — The unique id for the SMS campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">create_action_schedule</a>(sms_campaign_id: String, request: CreateActionScheduleSmsCampaignsRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Schedule an SMS campaign for delivery.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .create_action_schedule(
            &"sms_campaign_id".to_string(),
            &CreateActionScheduleSmsCampaignsRequest {
                schedule_time: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sms_campaign_id:** `String` — The unique id for the SMS campaign.
    
</dd>
</dl>

<dl>
<dd>

**schedule_time:** `String` — The UTC date and time to schedule the campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">create_action_send</a>(sms_campaign_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send an SMS campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .create_action_send(&"sms_campaign_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sms_campaign_id:** `String` — The unique id for the SMS campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">get_content</a>(sms_campaign_id: String) -> Result&lt;SmsCampaignContent, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the content for an SMS campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .get_content(
            &"sms_campaign_id".to_string(),
            &SmsCampaignsGetContentQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sms_campaign_id:** `String` — The unique id for the SMS campaign.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.sms_campaigns.<a href="/src/api/resources/sms_campaigns/client.rs">upsert_content</a>(sms_campaign_id: String, request: UpsertContentSmsCampaignsRequest) -> Result&lt;SmsCampaignContent, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Set the content for an SMS campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .sms_campaigns
        .upsert_content(
            &"sms_campaign_id".to_string(),
            &UpsertContentSmsCampaignsRequest {
                message_body: "message_body".to_string(),
                media: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sms_campaign_id:** `String` — The unique id for the SMS campaign.
    
</dd>
</dl>

<dl>
<dd>

**message_body:** `String` — The SMS message body.
    
</dd>
</dl>

<dl>
<dd>

**media:** `Option<Vec<UpsertContentSmsCampaignsRequestMediaItem>>` — Attached images or files. Limited to one item. Omitting this field or sending an empty array removes any existing media; to keep the current media while updating other fields, re-send the media array.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## SearchMembers
<details><summary><code>client.search_members.<a href="/src/api/resources/search_members/client.rs">list</a>(query: Option&lt;String&gt;, list_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListSearchMembersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Search for list members. This search can be restricted to a specific list, or can be used to search across all lists in an account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .search_members
        .list(
            &SearchMembersListQueryRequest {
                query: "query".to_string(),
                fields: vec![],
                exclude_fields: vec![],
                list_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**query:** `String` — The search query used to filter results. Query should be a valid email, or a string representing a contact's first or last name.
    
</dd>
</dl>

<dl>
<dd>

**list_id:** `Option<String>` — The unique id for the list.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## TemplateFolders
<details><summary><code>client.template_folders.<a href="/src/api/resources/template_folders/client.rs">list</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListTemplateFoldersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get all folders used to organize templates.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .template_folders
        .list(
            &TemplateFoldersListQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.template_folders.<a href="/src/api/resources/template_folders/client.rs">create</a>(request: CreateTemplateFoldersRequest) -> Result&lt;CreateTemplateFoldersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new template folder.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .template_folders
        .create(
            &CreateTemplateFoldersRequest {
                name: "name".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `String` — The name of the folder.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.template_folders.<a href="/src/api/resources/template_folders/client.rs">get</a>(folder_id: String) -> Result&lt;GetTemplateFoldersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific folder used to organize templates.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .template_folders
        .get(
            &"folder_id".to_string(),
            &TemplateFoldersGetQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**folder_id:** `String` — The unique id for the template folder.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.template_folders.<a href="/src/api/resources/template_folders/client.rs">delete</a>(folder_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a specific template folder, and mark all the templates in the folder as 'unfiled'.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .template_folders
        .delete(&"folder_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**folder_id:** `String` — The unique id for the template folder.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.template_folders.<a href="/src/api/resources/template_folders/client.rs">update</a>(folder_id: String, request: UpdateTemplateFoldersRequest) -> Result&lt;UpdateTemplateFoldersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a specific folder used to organize templates.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .template_folders
        .update(
            &"folder_id".to_string(),
            &UpdateTemplateFoldersRequest {
                name: "name".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**folder_id:** `String` — The unique id for the template folder.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — The name of the folder.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## templates
<details><summary><code>client.templates.<a href="/src/api/resources/templates/client.rs">list</a>(count: Option&lt;Option&lt;i64&gt;&gt;, offset: Option&lt;Option&lt;i64&gt;&gt;, created_by: Option&lt;Option&lt;String&gt;&gt;, since_date_created: Option&lt;Option&lt;String&gt;&gt;, before_date_created: Option&lt;Option&lt;String&gt;&gt;, type_: Option&lt;Option&lt;String&gt;&gt;, category: Option&lt;Option&lt;String&gt;&gt;, folder_id: Option&lt;Option&lt;String&gt;&gt;, sort_field: Option&lt;Option&lt;ListTemplatesRequestSortField&gt;&gt;, content_type: Option&lt;Option&lt;ListTemplatesRequestContentType&gt;&gt;, sort_dir: Option&lt;Option&lt;ListTemplatesRequestSortDir&gt;&gt;) -> Result&lt;ListTemplatesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of an account's available templates.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .templates
        .list(
            &TemplatesListQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
                count: None,
                offset: None,
                created_by: None,
                since_date_created: None,
                before_date_created: None,
                r#type: None,
                category: None,
                folder_id: None,
                sort_field: None,
                content_type: None,
                sort_dir: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — The number of records to return. Default value is 10. Maximum value is 1000
    
</dd>
</dl>

<dl>
<dd>

**offset:** `Option<i64>` — Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    
</dd>
</dl>

<dl>
<dd>

**created_by:** `Option<String>` — The Mailchimp account user who created the template.
    
</dd>
</dl>

<dl>
<dd>

**since_date_created:** `Option<String>` — Restrict the response to templates created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**before_date_created:** `Option<String>` — Restrict the response to templates created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<String>` — Limit results based on template type.
    
</dd>
</dl>

<dl>
<dd>

**category:** `Option<String>` — Limit results based on category.
    
</dd>
</dl>

<dl>
<dd>

**folder_id:** `Option<String>` — The unique folder id.
    
</dd>
</dl>

<dl>
<dd>

**sort_field:** `Option<ListTemplatesRequestSortField>` — Returns user templates sorted by the specified field.
    
</dd>
</dl>

<dl>
<dd>

**content_type:** `Option<ListTemplatesRequestContentType>` — Limit results based on how the template's content is put together. Only templates of type `user` can be filtered by `content_type`. If you want to retrieve saved templates created with the legacy email editor, then filter `content_type` to `template`. If you'd rather pull your saved templates for the new editor, filter to `multichannel`. For code your own templates, filter to `html`.
    
</dd>
</dl>

<dl>
<dd>

**sort_dir:** `Option<ListTemplatesRequestSortDir>` — Determines the order direction for sorted results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.templates.<a href="/src/api/resources/templates/client.rs">create</a>(request: CreateTemplatesRequest) -> Result&lt;TemplateInstance, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new template for the account. Only Classic templates are supported.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .templates
        .create(
            &CreateTemplatesRequest {
                html: "html".to_string(),
                name: "Freddie's Jokes".to_string(),
                folder_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**folder_id:** `Option<String>` — The id of the folder the template is currently in.
    
</dd>
</dl>

<dl>
<dd>

**html:** `String` — The raw HTML for the template. We  support the Mailchimp [Template Language](https://mailchimp.com/help/getting-started-with-mailchimps-template-language/) in any HTML code passed via the API.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — The name of the template.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.templates.<a href="/src/api/resources/templates/client.rs">get</a>(template_id: String) -> Result&lt;TemplateInstance, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get information about a specific template.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .templates
        .get(
            &"template_id".to_string(),
            &TemplatesGetQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**template_id:** `String` — The unique id for the template.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.templates.<a href="/src/api/resources/templates/client.rs">delete</a>(template_id: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a specific template.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .templates
        .delete(&"template_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**template_id:** `String` — The unique id for the template.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.templates.<a href="/src/api/resources/templates/client.rs">update</a>(template_id: String, request: UpdateTemplatesRequest) -> Result&lt;TemplateInstance, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update the name, HTML, or `folder_id` of an existing template.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .templates
        .update(
            &"template_id".to_string(),
            &UpdateTemplatesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**template_id:** `String` — The unique id for the template.
    
</dd>
</dl>

<dl>
<dd>

**folder_id:** `Option<String>` — The id of the folder the template is currently in.
    
</dd>
</dl>

<dl>
<dd>

**html:** `Option<String>` — The raw HTML for the template. We  support the Mailchimp [Template Language](https://mailchimp.com/help/getting-started-with-mailchimps-template-language/) in any HTML code passed via the API.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — The name of the template.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.templates.<a href="/src/api/resources/templates/client.rs">list_default_content</a>(template_id: String) -> Result&lt;ListDefaultContentTemplatesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the sections that you can edit in a template, including each section's default content.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .templates
        .list_default_content(
            &"template_id".to_string(),
            &ListDefaultContentQueryRequest {
                fields: vec![],
                exclude_fields: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**template_id:** `String` — The unique id for the template.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<String>` — A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>

<dl>
<dd>

**exclude_fields:** `Option<String>` — A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## VerifiedDomains
<details><summary><code>client.verified_domains.<a href="/src/api/resources/verified_domains/client.rs">list</a>() -> Result&lt;ListVerifiedDomainsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get all of the sending domains on the account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client.verified_domains.list(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.verified_domains.<a href="/src/api/resources/verified_domains/client.rs">create</a>(request: CreateVerifiedDomainsRequest) -> Result&lt;CreateVerifiedDomainsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a domain to the account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .verified_domains
        .create(
            &CreateVerifiedDomainsRequest {
                verification_email: "verification_email".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**verification_email:** `String` — The e-mail address at the domain you want to verify. This will receive a two-factor challenge to be used in the verify action.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.verified_domains.<a href="/src/api/resources/verified_domains/client.rs">get</a>(domain_name: String) -> Result&lt;GetVerifiedDomainsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the details for a single domain on the account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .verified_domains
        .get(&"domain_name".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_name:** `String` — The domain name.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.verified_domains.<a href="/src/api/resources/verified_domains/client.rs">delete</a>(domain_name: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a verified domain from the account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .verified_domains
        .delete(&"domain_name".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_name:** `String` — The domain name.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.verified_domains.<a href="/src/api/resources/verified_domains/client.rs">create_action_verify</a>(domain_name: String, request: CreateActionVerifyVerifiedDomainsRequest) -> Result&lt;CreateActionVerifyVerifiedDomainsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Verify a domain for sending.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use mailchimp_marketing::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = MailchimpClient::new(config).expect("Failed to build client");
    client
        .verified_domains
        .create_action_verify(
            &"domain_name".to_string(),
            &CreateActionVerifyVerifiedDomainsRequest {
                code: "code".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_name:** `String` — The domain name.
    
</dd>
</dl>

<dl>
<dd>

**code:** `String` — The code that was sent to the email address provided when adding a new domain to verify.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

