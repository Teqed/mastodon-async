//! Module representing a server rule.
use crate::RuleId;
use serde::{Deserialize, Serialize};

/// A server rule.
/// https://docs.joinmastodon.org/entities/Rule/
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Rule {
    /// The ID of the rule.
    pub id: RuleId,
    /// The text of the rule.
    pub text: String,
}
