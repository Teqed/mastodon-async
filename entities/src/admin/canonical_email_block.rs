//! Module representing a canonical email block.

use crate::prelude::CanonicalEmailBlockId;
use serde::{Deserialize, Serialize};
use serde_with::{hex::Hex, serde_as};

/// A canonical email block.
/// https://docs.joinmastodon.org/entities/Admin_CanonicalEmailBlock/
#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CanonicalEmailBlock {
    /// The ID of the canonical email block.
    pub id: CanonicalEmailBlockId,
    /// The SHA256 hash of the canonical email address.
    #[serde_as(as = "Hex")]
    pub canonical_email_hash: Vec<u8>,
}
