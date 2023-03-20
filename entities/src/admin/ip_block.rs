//! Module representing an IP range block.

use crate::prelude::IpBlockId;
use derive_is_enum_variant::is_enum_variant;
use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// An IP range block.
/// https://docs.joinmastodon.org/entities/Admin_IpBlock/
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct IpBlock {
    /// The ID of the IP range block.
    pub id: IpBlockId,
    /// The IP range affected.
    pub ip: IpNet,
    /// The level of restriction to be applied by this block.
    pub severity: Severity,
    /// The recorded reason for this block.
    pub comment: String,
    /// The time the block was created.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// The time the block will expire.
    #[serde(with = "iso8601::option")]
    pub expires_at: Option<OffsetDateTime>,
}

/// Level of restriction of the IP range.
/// https://docs.joinmastodon.org/entities/Admin_IpBlock/#severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, is_enum_variant)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// Any signup from this IP range will create a pending account.
    SignUpRequiresApproval,
    /// Any signup from this IP range will be rejected.
    SignUpBlock,
    /// Any activity from this IP range will be rejected entirely.
    NoAccess,
}
