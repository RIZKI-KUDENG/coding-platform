use std::fmt;

/// Opaque failure coming from infrastructure (database, container runtime, ...).
///
/// Application errors wrap this type so they do not depend on `sqlx` or any other
/// concrete driver. The detail is kept for logging only and must never be
/// returned to API clients.
#[derive(Debug)]
pub struct InfrastructureError(String);

impl InfrastructureError {
    pub fn new(detail: impl Into<String>) -> Self {
        Self(detail.into())
    }

    pub fn detail(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for InfrastructureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for InfrastructureError {}

impl From<sqlx::Error> for InfrastructureError {
    fn from(err: sqlx::Error) -> Self {
        Self(err.to_string())
    }
}
