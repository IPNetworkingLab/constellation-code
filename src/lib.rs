pub mod code;
pub mod patterns;

use std::{collections::HashMap, path::PathBuf};

use clap::builder::OsStr;
use serde::Deserialize;

use crate::code::*;

pub use code::Code;

#[allow(unused)]
#[derive(Deserialize, Debug, Clone)]
pub struct Gateway {
    pub name: String,
    #[serde(alias = "latitude_deg")]
    pub latitude: f64,
    #[serde(alias = "longitude_deg")]
    pub longitude: f64,
    #[serde(alias = "altitude_m")]
    pub altitude: f64,
    #[serde(alias = "min_elevation_deg")]
    pub min_elevation: f64,
}

#[allow(unused)]
#[derive(Deserialize, Debug, Clone)]
pub struct ConstellationShell {
    pub code: String,
    pub name: String,
    pub link_patterns: Vec<HashMap<String, usize>>,
}

impl ConstellationShell {
    pub fn get_code(&self) -> Result<Code, CodeError> {
        Code::try_from(self.code.as_str())
    }
}

#[allow(unused)]
#[derive(Deserialize, Debug, Clone)]
pub struct ConstellationCode {
    pub version: String,
    pub shells: Vec<ConstellationShell>,
    pub gateways: Vec<Gateway>,
}

impl TryFrom<&PathBuf> for ConstellationCode {
    // FIXME
    type Error = ();
    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        if !path.exists() || !path.is_file() || path.extension() != Some(&OsStr::from("yaml")) {
            return Err(());
        }
        let raw = std::fs::read(path).map_err(|_| ())?;
        yaml_serde::from_slice::<ConstellationCode>(&raw).map_err(|_| ())
    }
}
