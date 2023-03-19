//! Module representing an IP address with metadata.

use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use time::{serde::iso8601, OffsetDateTime};

/// An IP address with metadata related to the history of a given account.
///
/// https://docs.joinmastodon.org/entities/Admin_Ip/
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Ip {
    /// The IP address itself.
    pub ip: IpAddr,
    /// The time this address was last used by this account.
    #[serde(with = "iso8601")]
    pub used_at: OffsetDateTime,
}
