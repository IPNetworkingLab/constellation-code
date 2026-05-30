use serde::Deserialize;

use crate::patterns::*;

pub enum CodeError {
    CodeMalformed,
    Pattern(PatternError),
    AltitudeUnparsable,
    InclinationUnparsable,
    PlaneParameters(PlaneParametersError),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Code {
    pub pattern: Pattern,
    pub altitude: f64,
    pub inclination: f64,
    pub plane_parameters: PlaneParameters,
}

impl Code {
    pub fn sats_per_plane(&self) -> usize {
        (self.plane_parameters.sats / self.plane_parameters.planes) as usize
    }

    pub fn coordinates_to_id(&self, plane: usize, rank: usize) -> Result<usize, ()> {
        if plane >= self.plane_parameters.planes as usize {
            return Err(());
        }
        let sats_per_plane = self.sats_per_plane();
        if rank >= sats_per_plane {
            return Err(());
        }
        Ok(plane * sats_per_plane + rank)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PlaneParameters {
    pub sats: u32,
    pub planes: u32,
    pub phasing: u32,
}

pub enum PlaneParametersError {
    Malformed,
    SatsUnparsable,
    PlanesUnparsable,
    PhasingFactorUnparsable,
}

impl TryFrom<&str> for PlaneParameters {
    type Error = PlaneParametersError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('/').collect();
        if parts.len() != 3 {
            return Err(PlaneParametersError::Malformed);
        }

        let sats = str::parse::<u32>(parts[0]).map_err(|_| PlaneParametersError::SatsUnparsable)?;
        let planes =
            str::parse::<u32>(parts[1]).map_err(|_| PlaneParametersError::PlanesUnparsable)?;
        let phasing = str::parse::<u32>(parts[2])
            .map_err(|_| PlaneParametersError::PhasingFactorUnparsable)?;

        Ok(PlaneParameters {
            sats,
            planes,
            phasing,
        })
    }
}

impl TryFrom<String> for Code {
    type Error = CodeError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(':').collect();
        if parts.len() < 4 {
            return Err(CodeError::CodeMalformed);
        }

        let pattern = Pattern::try_from(parts[0]).map_err(|err| CodeError::Pattern(err))?;
        let altitude = str::parse::<f64>(parts[1]).map_err(|_| CodeError::AltitudeUnparsable)?;
        let inclination =
            str::parse::<f64>(parts[2]).map_err(|_| CodeError::InclinationUnparsable)?;
        let plane_parameters =
            PlaneParameters::try_from(parts[3]).map_err(|err| CodeError::PlaneParameters(err))?;
        Ok(Code {
            pattern,
            altitude,
            inclination,
            plane_parameters,
        })
    }
}

impl TryFrom<&str> for Code {
    type Error = CodeError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Code::try_from(value.to_string())
    }
}
