use mastodon_async_entities::{report::Category, RuleId};
use serde_with::skip_serializing_none;

/// Change metadata for a report.
/// https://docs.joinmastodon.org/methods/admin/reports/#path-parameters-1
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpdateReportRequest {
    category: Option<Category>,
    rule_ids: Option<Vec<RuleId>>,
}

impl UpdateReportRequest {
    pub fn new() -> Self {
        Self {
            category: None,
            rule_ids: None,
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_update_request() {
        let mut request = UpdateReportRequest::new();
        request.category(Category::Spam);
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(ser, r#"{"category":"spam"}"#);
    }
}
