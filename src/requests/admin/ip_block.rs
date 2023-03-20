use ipnet::IpNet;
use mastodon_async_entities::admin::IpBlockSeverity;
use time::{serde::rfc3339, OffsetDateTime};

/// Create a new IP range block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AddIpBlockRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<IpNet>,
    severity: IpBlockSeverity,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(serialize_with = "rfc3339::option::serialize")]
    expires_at: Option<OffsetDateTime>,
}

impl AddIpBlockRequest {
    pub fn new(severity: IpBlockSeverity) -> Self {
        AddIpBlockRequest {
            ip: None,
            severity,
            comment: None,
            expires_at: None,
        }
    }

    pub fn ip(&mut self, ip: IpNet) -> &mut Self {
        self.ip = Some(ip);
        self
    }

    pub fn comment(&mut self, comment: String) -> &mut Self {
        self.comment = Some(comment);
        self
    }

    pub fn expires_at(&mut self, dt: OffsetDateTime) -> &mut Self {
        self.expires_at = Some(dt);
        self
    }
}

/// Update an existing IP range block.
/// Differs from [`AddIpBlockRequest`] only in that all parameters are optional.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpdateIpBlockRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<IpNet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severity: Option<IpBlockSeverity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "rfc3339::option::serialize")]
    expires_at: Option<OffsetDateTime>,
}

impl UpdateIpBlockRequest {
    pub fn new() -> Self {
        UpdateIpBlockRequest {
            ip: None,
            severity: None,
            comment: None,
            expires_at: None,
        }
    }

    pub fn ip(&mut self, ip: IpNet) -> &mut Self {
        self.ip = Some(ip);
        self
    }

    pub fn severity(&mut self, severity: IpBlockSeverity) -> &mut Self {
        self.severity = Some(severity);
        self
    }

    pub fn comment(&mut self, comment: String) -> &mut Self {
        self.comment = Some(comment);
        self
    }

    pub fn expires_at(&mut self, dt: OffsetDateTime) -> &mut Self {
        self.expires_at = Some(dt);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ipnet::IpNet;
    use serde_json;
    use std::str::FromStr;
    use time::OffsetDateTime;

    #[test]
    fn test_serialize_add_request() {
        let mut request = AddIpBlockRequest::new(IpBlockSeverity::SignUpRequiresApproval);
        request.ip(IpNet::from_str("192.168.0.0/16").unwrap());
        request.comment("test comment".to_string());
        request.expires_at(OffsetDateTime::from_unix_timestamp(1679261134).unwrap());
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(
            ser,
            r#"{"ip":"192.168.0.0/16","severity":"sign_up_requires_approval","comment":"test comment","expires_at":"2023-03-19T21:25:34Z"}"#
        )
    }

    #[test]
    fn test_serialize_update_request() {
        let mut request = UpdateIpBlockRequest::new();
        request.severity(IpBlockSeverity::NoAccess);
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(ser, r#"{"severity":"no_access"}"#)
    }
}
