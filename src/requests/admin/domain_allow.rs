/// Create a new domain allow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AddDomainAllowRequest {
    domain: String,
}

impl AddDomainAllowRequest {
    pub fn new<T>(domain: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            domain: domain.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_add_request() {
        let request = AddDomainAllowRequest::new("example.org");
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(ser, r#"{"domain":"example.org"}"#);
    }
}
