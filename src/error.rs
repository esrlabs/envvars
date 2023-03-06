use std::{io, path::PathBuf};

/// Definition of shell profile
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Output of extractor isn't found
    #[error("No extractor output")]
    NoExtractorOutput,
    /// PoisonError error
    #[error("PoisonError: {0:?}")]
    PoisonError(String),
    /// IO related error
    #[error("IO Error: {0:?}")]
    Io(io::Error),
    /// Happens if stdout has content, which isn't possible to parse as json
    /// string.
    #[error("Parsing error: {0:?}")]
    Parsing(String, Option<i32>, String, String),
    /// Any error during attempt to execute extractor as target shell command
    #[error("Fail to execute extractor: {0:?}")]
    Executing(io::Error),
    /// Happens if by some reasons isn't possible to create extractor in system
    /// temporary folder
    #[error("Fail to create extractor: {0:?}")]
    Create(io::Error),
    /// Will be dropped if attempt to decode stdout or stderr of shell child process
    /// is failed
    #[error("Fail to decode stdout/stderr: {0:?}")]
    Decoding(std::str::Utf8Error),
    /// Shell executable file doesn't exist
    #[error("Shell executor isn't found: {0:?}")]
    NotFound(PathBuf),
    /// Target platform isn't supported
    #[error("Platform isn't supported")]
    NotSupportedPlatform,
    /// Happends on errors related converting paths to strings
    #[error("Infallible: {0}")]
    Infallible(std::convert::Infallible),
    /// Some environment variables are needed to detect specific paths on windows,
    /// like system path, path to program files etc. This error happens if neened
    /// variables aren't found
    #[error("Fail to find envvar: {0}")]
    NotFoundEnvVar(String),
    /// Any other errors
    #[error("Other: {0}")]
    Other(String),
}
