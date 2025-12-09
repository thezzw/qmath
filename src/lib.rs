
mod basic;
mod coordic;

pub mod vec2;
pub mod rng;
pub mod dir;
pub mod prelude {
    pub use crate::{
        Q64,
        basic::*,
        coordic::*,
        q64, qvec2,
    };
}

pub type Q64 = fixed::types::I32F32;

/// Constructs a fixed-point number of type Q64.
#[macro_export]
macro_rules! q64 {
    ($num:expr) => { Q64::from_num($num) };
}

/// Constructs a fixed-point vector2.
#[macro_export]
macro_rules! qvec2 {
    ($x:expr, $y:expr) => {
        QVec2::new(q64!($x), q64!($y))
    };
}