use routee_compass_core::model::unit::{DistanceUnit, WeightUnit};
use serde::{Deserialize, Serialize};

use crate::app::compass::model::frontier_model::vehicle_restrictions::VehicleParameter;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum VehicleParameterConfig {
    Height { value: f64, unit: DistanceUnit },
    Width { value: f64, unit: DistanceUnit },
    TotalLength { value: f64, unit: DistanceUnit },
    TrailerLength { value: f64, unit: DistanceUnit },
    TotalWeight { value: f64, unit: WeightUnit },
    WeightPerAxle { value: f64, unit: WeightUnit },
}

impl Into<VehicleParameter> for VehicleParameterConfig {
    fn into(self) -> VehicleParameter {
        match self {
            VehicleParameterConfig::Height { value, unit } => VehicleParameter::Height {
                value: unit.to_uom(value),
            },
            VehicleParameterConfig::Width { value, unit } => VehicleParameter::Width {
                value: unit.to_uom(value),
            },
            VehicleParameterConfig::TotalLength { value, unit } => VehicleParameter::TotalLength {
                value: unit.to_uom(value),
            },
            VehicleParameterConfig::TrailerLength { value, unit } => {
                VehicleParameter::TrailerLength {
                    value: unit.to_uom(value),
                }
            }
            VehicleParameterConfig::TotalWeight { value, unit } => VehicleParameter::TotalWeight {
                value: unit.to_uom(value),
            },
            VehicleParameterConfig::WeightPerAxle { value, unit } => {
                VehicleParameter::WeightPerAxle {
                    value: unit.to_uom(value),
                }
            }
        }
    }
}
