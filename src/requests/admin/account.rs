use derive_is_enum_variant::is_enum_variant;
use mastodon_async_entities::ReportId;
use serde_with::skip_serializing_none;

/// Perform an admin action on an account.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AccountActionRequest {
    #[serde(rename = "type")]
    action_type: AccountActionType,
    report_id: Option<ReportId>,
    /// TODO: if warning presets are exposed anywhere in the API, they're not documented
    warning_preset_id: Option<String>,
    text: Option<String>,
    send_email_notification: Option<bool>,
}

impl AccountActionRequest {
    pub fn new(action_type: AccountActionType) -> Self {
        Self {
            action_type,
            report_id: None,
            warning_preset_id: None,
            text: None,
            send_email_notification: None,
        }
    }

    pub fn report_id(&mut self, report_id: ReportId) -> &mut Self {
        self.report_id = Some(report_id);
        self
    }

    pub fn warning_preset_id<T>(&mut self, warning_preset_id: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.warning_preset_id = Some(warning_preset_id.into());
        self
    }

    pub fn text<T>(&mut self, text: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.text = Some(text.into());
        self
    }

    pub fn send_email_notification(&mut self, send_email_notification: bool) -> &mut Self {
        self.send_email_notification = Some(send_email_notification);
        self
    }
}

/// Action to be performed on the account.
/// https://docs.joinmastodon.org/methods/admin/accounts/#form-data-parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, is_enum_variant)]
#[serde(rename_all = "snake_case")]
pub enum AccountActionType {
    None,
    Sensitive,
    Disable,
    Silence,
    Suspend,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_action_request() {
        let mut request = AccountActionRequest::new(AccountActionType::Suspend);
        request.report_id(ReportId::new("666"));
        request.text("you know what you did");
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(
            ser,
            r#"{"type":"suspend","report_id":"666","text":"you know what you did"}"#
        );
    }
}
