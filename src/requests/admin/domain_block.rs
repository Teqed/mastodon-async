use mastodon_async_entities::admin::domain_block::Severity;
use serde_with::skip_serializing_none;

/// Create a new domain block or update an existing one.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AddDomainBlockRequest {
    domain: String,
    severity: Option<Severity>,
    reject_media: Option<bool>,
    reject_reports: Option<bool>,
    private_comment: Option<String>,
    public_comment: Option<String>,
    obfuscate: Option<bool>,
}

impl AddDomainBlockRequest {
    pub fn new<T>(domain: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            domain: domain.into(),
            severity: None,
            reject_media: None,
            reject_reports: None,
            private_comment: None,
            public_comment: None,
            obfuscate: None,
        }
    }

    pub fn severity(&mut self, severity: Severity) -> &mut Self {
        self.severity = Some(severity);
        self
    }

    pub fn reject_media(&mut self, reject_media: bool) -> &mut Self {
        self.reject_media = Some(reject_media);
        self
    }

    pub fn reject_reports(&mut self, reject_reports: bool) -> &mut Self {
        self.reject_reports = Some(reject_reports);
        self
    }

    pub fn private_comment<T>(&mut self, private_comment: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.private_comment = Some(private_comment.into());
        self
    }

    pub fn public_comment<T>(&mut self, public_comment: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.public_comment = Some(public_comment.into());
        self
    }

    pub fn obfuscate(&mut self, obfuscate: bool) -> &mut Self {
        self.obfuscate = Some(obfuscate);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_add_request() {
        let mut request = AddDomainBlockRequest::new("example.org");
        request.severity(Severity::Silence);
        request.reject_media(true);
        request.public_comment("public comment");
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(
            ser,
            r#"{"domain":"example.org","severity":"silence","reject_media":true,"public comment":"public comment"}"#
        );
    }
}
