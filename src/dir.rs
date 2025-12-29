use crate::prelude::*;
use crate::vec2::QVec2;
use serde::{Deserialize, Serialize};
use core::ops::*;

/// A 2-dimensional direction.
/// 
/// Angle in radians, within [0, 2PI), CCW order.
/// 
/// QDir of `QVec2::X` is zero.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QDir {
    angle: Q64,
}

impl QDir {
    pub fn new(angle: Q64) -> Self {
        assert!(angle >= Q64::ZERO && angle < Q64::TWO_PI, "[QDir::new] Angle should be in range of [0, 2PI)");
        Self { angle }
    }

    pub fn new_from_angle(angle: Q64) -> Self {
        let mod_angle = angle % Q64::TWO_PI;
        if mod_angle < Q64::ZERO {
            Self::new(mod_angle + Q64::TWO_PI)
        } else {
            Self::new(mod_angle)
        }
    }

    pub fn new_from_vec(vec: QVec2) -> Self {
        let angle_atan: Q64 = Q64::atan2(vec.y, vec.x);
        if angle_atan < 0 {
            Self::new(angle_atan + Q64::TWO_PI)
        } else {
            Self::new(angle_atan)
        }
    }

    pub fn angle(&self) -> Q64 {
        self.angle
    }

    pub fn to_vec(&self) -> QVec2 {
        QVec2::from_angle(self.angle)
    }

    pub fn rotate(&mut self, angle: Q64) {
        let new_dir = Self::new_from_angle(self.angle + angle);
        self.angle = new_dir.angle
    }

    pub fn rotate_dir(&self, other: QDir) -> QDir {
        QDir::new_from_angle(self.angle + other.angle)
    }

    pub fn rotate_vec(&self, vec: QVec2) -> QVec2 {
        self.to_vec().rotate(vec)
    }

    pub fn projection_of(&self, vec: QVec2) -> Q64 {
        self.to_vec().dot(vec)
    }
}

impl Neg for QDir {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new_from_angle(self.angle.neg())
    }
}

impl From<QDir> for QVec2 {
    fn from(dir: QDir) -> Self {
        QVec2::from_angle(dir.angle)
    }
}

impl From<QVec2> for QDir {
    fn from(vec: QVec2) -> Self {
        QDir::new_from_vec(vec)
    }
}