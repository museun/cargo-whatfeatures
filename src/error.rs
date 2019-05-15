pub enum Error {
    NoNameProvided,
    CannotLookup {
        name: String,
        version: Option<String>,
        error: String,
    },
    NoVersions(String),
    InvalidVersion(String, String),
}
