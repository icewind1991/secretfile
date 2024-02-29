use std::borrow::Cow;
use std::env::var;
use std::fs::read_to_string;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecretError {
    #[error("failed to load token from {path}: {error:#}")]
    Load { path: String, error: std::io::Error },
    #[error("environment variable {0} referenced but not set")]
    MissingEnvVar(String),
}

/// Load a secret from the provided path
///
/// If the provided path includes the `$CREDENTIALS_DIRECTORY` placeholder, it will be replaced with the
/// systemd service credential directory.
///
/// any leading whitespace will be stripped from the returned secret.
pub fn load(path: &str) -> Result<String, SecretError> {
    let file = if path.contains("$CREDENTIALS_DIRECTORY") {
        let dir = var("CREDENTIALS_DIRECTORY")
            .map_err(|_| SecretError::MissingEnvVar("$CREDENTIALS_DIRECTORY".into()))?;
        Cow::Owned(path.replace("$CREDENTIALS_DIRECTORY", &dir))
    } else {
        Cow::Borrowed(path)
    };

    let mut content = read_to_string(file.as_ref()).map_err(|error| SecretError::Load {
        path: file.into(),
        error,
    })?;

    content.truncate(content.trim_end().len()); // trim in place
    Ok(content)
}
