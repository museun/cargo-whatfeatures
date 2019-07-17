#[derive(Debug)]
pub enum Error {
    NoNameProvided,
    CannotLookup {
        name: String,
        version: Option<String>,
        error: Box<Error>,
    },
    NoVersions(String),
    InvalidVersion(String, String),
    Json(serde_json::Error),
    Http(attohttpc::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Json(err) => write!(f, "json deserialization error: {}", err),
            Error::Http(err) => write!(f, "http get error: {}", err),
            _ => unreachable!(),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Json(err) => Some(err),
            Error::Http(err) => Some(err),
            _ => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
