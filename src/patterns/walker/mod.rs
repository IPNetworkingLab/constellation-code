use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum WalkerPattern {
    Delta,
    Star,
}

pub enum WalkerError {
    Unsupported,
}

impl TryFrom<&str> for WalkerPattern {
    type Error = WalkerError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "D" {
            Ok(Self::Delta)
        } else if value == "S" {
            Ok(Self::Star)
        } else {
            Err(Self::Error::Unsupported)
        }
    }
}
