use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrepError {
    #[error("Glob pattern error")]
    GlobPatternError(#[from] glob::PatternError),

    #[error("Regex pattern error")]
    RegexPatternError(#[from] regex::Error),

    #[error("IO error")]
    IoError(#[from] std::io::Error),
}
