//! Module representing an IP range block.

use super::ids::IpBlockId;
use ipnet::IpNet;
use is_variant::IsVariant;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// An IP range block.
///
/// https://docs.joinmastodon.org/entities/Admin_IpBlock/
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct IpBlock {
    /// The time this address was last used by this account.
    pub id: IpBlockId,
    /// The time this address was last used by this account.
    pub ip: IpNet,
    /// The time the block was created.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// The time the block will expire.
    #[serde(with = "iso8601::option")]
    pub expires_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, IsVariant)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    SignUpRequiresApproval,
    SignUpBlock,
    NoAccess,
}
