//! A module containing everything relating to a report as seen by the admin API.

use crate::admin::Account;
use crate::report::Category;
use crate::rule::Rule;
use crate::status::Status;
use crate::ReportId;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// A report as seen by the admin API.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Report {
    pub id: ReportId,
    pub action_taken: bool,
    #[serde(with = "iso8601::option")]
    pub action_taken_at: Option<OffsetDateTime>,
    pub category: Category,
    pub comment: String,
    pub forwarded: bool,
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "iso8601")]
    pub updated_at: OffsetDateTime,
    /// The account which filed the report.
    pub account: Account,
    /// The account being reported.
    pub target_account: Account,
    /// The account of the moderator assigned to this report.
    pub assigned_account: Option<Account>,
    /// The account of the moderator who handled the report.
    pub action_taken_by_account: Option<Account>,
    pub statuses: Vec<Status>,
    pub rules: Vec<Rule>,
}
