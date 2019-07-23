#[derive(Debug)]
pub enum UserError {
    NoNameProvided,
    CannotLookup {
        name: String,
        version: Option<String>,
        error: InternalError,
    },
    NoVersions {
        name: String,
    },
    InvalidVersion {
        name: String,
        version: String,
    },
    MustOutputSomething,
}

#[derive(Debug)]
pub enum InternalError {
    Json(serde_json::Error),
    Http(attohttpc::Error),
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalError::Json(err) => write!(f, "json deserialization error: {}", err),
            InternalError::Http(err) => write!(f, "http get error: {}", err),
        }
    }
}

impl std::error::Error for InternalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InternalError::Json(err) => Some(err),
            InternalError::Http(err) => Some(err),
        }
    }
}
