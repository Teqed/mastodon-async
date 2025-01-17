use is_variant::IsVariant;

/// Error type
#[derive(Debug, thiserror::Error, IsVariant)]
pub enum Error {
    #[error("unrecognized visibility '{invalid}'")]
    VisibilityParsingError { invalid: String },
}
