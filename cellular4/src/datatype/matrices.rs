use nalgebra::{
    geometry::{Rotation2, Translation2},
    *,
};
use serde::{Deserialize, Serialize};

use crate::datatype::continuous::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SNFloatMatrix3 {
    value: nalgebra::Matrix3<f32>,
}

impl SNFloatMatrix3 {
    pub fn new_translation(x: SNFloat, y: SNFloat) -> Self {
        Self {
            value: Translation2::new(x.into_inner(), y.into_inner()).to_homogeneous(),
        }
    }

    pub fn new_rotation(theta: Angle) -> Self {
        Self {
            value: Rotation2::new(theta.into_inner()).to_homogeneous(),
        }
    }

    pub fn new_scaling(x: SNFloat, y: SNFloat) -> Self {
        Self {
            value: Matrix3::new_nonuniform_scaling(&Vector2::new(x.into_inner(), y.into_inner())),
        }
    }

    pub fn new_shear(x: SNFloat, y: SNFloat) -> Self {
        Self {
            value: Matrix2::new(1.0, x.into_inner(), y.into_inner(), 1.0).to_homogeneous(),
        }
    }

    pub fn multiply(self, other: Self) -> Self {
        Self {
            value: self.into_inner() * other.into_inner(),
        }
    }

    pub fn identity() -> Self {
        Self {
            value: Matrix3::identity(),
        }
    }

    pub fn into_inner(self) -> Matrix3<f32> {
        self.value
    }
}
