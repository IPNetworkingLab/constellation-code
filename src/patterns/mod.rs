pub mod walker;
pub use walker::{WalkerError, WalkerPattern};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum Pattern {
    Walker(WalkerPattern),
}

pub enum PatternError {
    Unsupported,
}
impl TryFrom<&str> for Pattern {
    type Error = PatternError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // TODO: Update this parser when more patterns are added.
        WalkerPattern::try_from(value)
            .map_err(|err| match err {
                WalkerError::Unsupported => Self::Error::Unsupported,
            })
            .map(|walker| Self::Walker(walker))
    }
}
