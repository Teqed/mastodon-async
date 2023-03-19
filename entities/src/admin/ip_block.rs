//! Module representing an IP block.

use super::ids::IpBlockId;
use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// An IP block.
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

pub enum Severity {}
