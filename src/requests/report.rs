use mastodon_async_entities::{report::Category, AccountId, RuleId, StatusId};
use serde_with::skip_serializing_none;

/// Create a report.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AddReportRequest {
    account_id: AccountId,
    status_ids: Option<Vec<StatusId>>,
    comment: Option<String>,
    forward: Option<bool>,
    category: Option<Category>,
    rule_ids: Option<Vec<RuleId>>,
}

impl AddReportRequest {
    /// Report an account.
    pub fn new(account_id: AccountId) -> Self {
        Self {
            account_id,
            status_ids: None,
            comment: None,
            forward: None,
            category: None,
            rule_ids: None,
        }
    }

    /// Attach statuses to the report to provide additional context.
    pub fn status_ids<T>(&mut self, status_ids: T) -> &mut Self
    where
        T: Into<Vec<StatusId>>,
    {
        self.status_ids = Some(status_ids.into());
        self
    }

    /// The reason for the report.
    pub fn comment<T>(&mut self, comment: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.comment = Some(comment.into());
        self
    }

    /// If the account is remote, should the report be forwarded to the remote admin?
    pub fn forward(&mut self, forward: bool) -> &mut Self {
        self.forward = Some(forward);
        self
    }

    /// Machine-readable category of the report.
    pub fn category(&mut self, category: Category) -> &mut Self {
        self.category = Some(category);
        self
    }

    /// Rules broken by a [`Category::Violation`] report.
    pub fn rule_ids<T>(&mut self, rule_ids: T) -> &mut Self
    where
        T: Into<Vec<RuleId>>,
    {
        self.rule_ids = Some(rule_ids.into());
        self
    }
}
