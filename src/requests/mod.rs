/// Data structure for the MastodonClient::add_filter method
pub use self::filter::AddFilterRequest;
/// Data structure for the MastodonClient::add_push_subscription method
pub use self::push::{AddPushRequest, Keys, UpdatePushRequest};
/// Data structure for the MastodonClient::add_report method
pub use self::report::AddReportRequest;
/// Data structure for the MastodonClient::statuses method
pub use self::statuses::StatusesRequest;
/// Data structure for the MastodonClient::update_credentials method
pub use self::update_credentials::UpdateCredsRequest;

mod filter;
mod push;
mod report;
mod statuses;
mod update_credentials;

/// Data structures for the MastodonClient::admin_perform_account_action methods
pub use self::admin::account::{
    AccountActionRequest as AdminAccountActionRequest, AccountActionType as AdminAccountActionType,
};
/// Data structures for the MastodonClient::admin_add_canonical_email_block and ::admin_test_canonical_email_blocks methods
pub use self::admin::canonical_email_block::{
    AddCanonicalEmailBlockRequest, TestCanonicalEmailBlocksRequest,
};
/// Data structure for the MastodonClient::admin_add_domain_allow method
pub use self::admin::domain_allow::AddDomainAllowRequest;
/// Data structure for the MastodonClient::admin_add_domain_block and ::admin_update_domain_block methods
pub use self::admin::domain_block::AddDomainBlockRequest as AddAdminDomainBlockRequest;
/// Data structure for the MastodonClient::admin_add_email_domain_block method
pub use self::admin::email_domain_block::AddEmailDomainBlockRequest;
/// Data structures for the MastodonClient::admin_add_ip_block and ::admin_update_ip_block methods
pub use self::admin::ip_block::{AddIpBlockRequest, UpdateIpBlockRequest};
/// Data structure for the MastodonClient::admin_add_ip_block and ::admin_update_ip_block methods
pub use self::admin::report::UpdateReportRequest;

/// Unprefixed admin request data structures.
pub mod admin;
