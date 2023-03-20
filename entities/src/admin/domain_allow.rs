//! Module representing a domain allow.

use crate::prelude::DomainAllowId;
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// A domain allow.
/// https://docs.joinmastodon.org/entities/Admin_DomainAllow/
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DomainAllow {
    /// The ID of the domain allow.
    pub id: DomainAllowId,
    /// The domain affected.
    pub domain: String,
    /// The time the allow was created.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
}
