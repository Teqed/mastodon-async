//! module containing information about a finished report of a user.
use crate::prelude::ReportId;
use derive_is_enum_variant::is_enum_variant;
use serde::{Deserialize, Serialize};

/// A struct containing info about a report.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Report {
    /// The ID of the report.
    pub id: ReportId,
    /// The action taken in response to the report.
    pub action_taken: bool,
}

/// Machine-readable category of a report.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, is_enum_variant)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    /// Spam
    Spam,
    /// Violation of enumerated instance rules
    Violation,
    /// Other
    Other,
}
