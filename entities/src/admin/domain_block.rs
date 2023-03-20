//! Module representing a domain block.

use crate::prelude::DomainBlockId;
use derive_is_enum_variant::is_enum_variant;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// A domain block.
/// https://docs.joinmastodon.org/entities/Admin_DomainBlock/
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DomainBlock {
    /// The ID of the domain block.
    pub id: DomainBlockId,
    /// The domain affected.
    pub domain: String,
    /// The time the block was created.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// The level of restriction to be applied by this block.
    pub severity: Severity,
    /// Whether to reject media attachments from this domain.
    pub reject_media: bool,
    /// Whether to reject reports from this domain.
    pub reject_reports: bool,
    /// Internally visible reason for the block.
    pub private_comment: Option<String>,
    /// Public reason for the block.
    pub public_comment: Option<String>,
    /// Whether to obfuscate public displays of this domain block.
    pub obfuscate: bool,
}

/// Level of restriction of the domain.
/// https://docs.joinmastodon.org/entities/Admin_DomainBlock/#severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, is_enum_variant)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// Do nothing. Allows for rejecting media or reports.
    Noop,
    /// Account statuses from this domain will be hidden by default.
    Silence,
    /// All incoming data from this domain will be rejected.
    Suspend,
}
