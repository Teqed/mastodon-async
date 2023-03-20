//! Module representing an email domain block.

use crate::prelude::EmailDomainBlockId;
use serde::{Deserialize, Serialize};
use serde_with::{formats::Flexible, serde_as, DisplayFromStr, PickFirst, TimestampSeconds};
use time::{serde::iso8601, OffsetDateTime};

/// An email domain block.
/// https://docs.joinmastodon.org/entities/Admin_EmailDomainBlock/
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct EmailDomainBlock {
    /// The ID of the email domain block.
    pub id: EmailDomainBlockId,
    /// The email domain that is not allowed to be used for signups.
    pub domain: String,
    /// The time the block was created.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// Usage statistics for given days (typically the past week).
    pub history: Vec<HistoryDay>,
}

/// A day of usage statistics for an email domain.
/// https://docs.joinmastodon.org/entities/Admin_EmailDomainBlock/#history
#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct HistoryDay {
    /// Midnight of the given day.
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub day: OffsetDateTime,
    /// The counted accounts signup attempts using that email domain within that day.
    #[serde_as(as = "PickFirst<(DisplayFromStr, _)>")]
    pub accounts: i64,
    /// The counted IP signup attempts of that email domain within that day.
    #[serde_as(as = "PickFirst<(DisplayFromStr, _)>")]
    pub uses: i64,
}
