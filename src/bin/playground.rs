use qmath::prelude::*;
use qmath::vec2::QVec2;

fn main() {
    println!("{name:<16}: {}", Q64::ZERO, name = "ZERO");
    println!("{name:<16}: {}", Q64::HALF, name = "HALF");
    println!("{name:<16}: {}", Q64::ONE, name = "ONE");
    println!("{name:<16}: {}", Q64::TWO, name = "TWO");
    println!("{name:<16}: {}", Q64::NEG_HALF, name = "NEG_HALF");
    println!("{name:<16}: {}", Q64::NEG_ONE, name = "NEG_ONE");
    println!("{name:<16}: {}", Q64::NEG_TWO, name = "NEG_TWO");
    println!("{name:<16}: {}", Q64::DELTA, name = "DELTA");
    println!("{name:<16}: {}", Q64::NEG_DELTA, name = "NEG_DELTA");
    println!("{name:<16}: {}", Q64::MIN, name = "MIN");
    println!("{name:<16}: {}", Q64::MAX, name = "MAX");
    println!("{name:<16}: {}\n", Q64::EPS, name = "EPS");

    println!("{name:<16}: {}", Q64::SQRT_2, name = "SQRT_2");
    println!("{name:<16}: {}", Q64::SQRT_3, name = "SQRT_3");
    println!("{name:<16}: {}", Q64::LN_2, name = "LN_2");
    println!("{name:<16}: {}", Q64::LN_10, name = "LN_10");
    println!("{name:<16}: {}", Q64::LOG10_2, name = "LOG10_2");
    println!("{name:<16}: {}", Q64::LOG2_10, name = "LOG2_10");
    println!("{name:<16}: {}", Q64::FRAC_1_SQRT_2, name = "FRAC_1_SQRT_2");
    println!("{name:<16}: {}\n", Q64::FRAC_1_SQRT_3, name = "FRAC_1_SQRT_3");

    println!("{name:<16}: {}", Q64::PI, name = "PI");
    println!("{name:<16}: {}", Q64::NEG_PI, name = "NEG_PI");
    println!("{name:<16}: {}", Q64::TWO_PI, name = "TWO_PI");
    println!("{name:<16}: {}", Q64::NEG_TWO_PI, name = "NEG_TWO_PI");
    println!("{name:<16}: {}", Q64::SQRT_PI, name = "SQRT_PI");
    println!("{name:<16}: {}", Q64::SQRT_2PI, name = "SQRT_2PI");
    println!("{name:<16}: {}", Q64::FRAC_PI_2, name = "FRAC_PI_2");
    println!("{name:<16}: {}", Q64::FRAC_PI_3, name = "FRAC_PI_3");
    println!("{name:<16}: {}", Q64::FRAC_PI_4, name = "FRAC_PI_4");
    println!("{name:<16}: {}", Q64::FRAC_PI_6, name = "FRAC_PI_6");
    println!("{name:<16}: {}", Q64::FRAC_PI_8, name = "FRAC_PI_8");
    println!("{name:<16}: {}", Q64::FRAC_1_PI, name = "FRAC_1_PI");
    println!("{name:<16}: {}", Q64::FRAC_2_PI, name = "FRAC_2_PI");
    println!("{name:<16}: {}", Q64::FRAC_1_SQRT_PI, name = "FRAC_1_SQRT_PI");
    println!("{name:<16}: {}", Q64::FRAC_2_SQRT_PI, name = "FRAC_2_SQRT_PI");
    println!("{name:<16}: {}\n", Q64::FRAC_1_SQRT_2PI, name = "FRAC_1_SQRT_2PI");

    println!("{name:<16}: {}", Q64::TAU, name = "TAU");
    println!("{name:<16}: {}", Q64::FRAC_1_TAU, name = "FRAC_1_TAU");
    println!("{name:<16}: {}", Q64::FRAC_2_TAU, name = "FRAC_2_TAU");
    println!("{name:<16}: {}", Q64::FRAC_4_TAU, name = "FRAC_4_TAU");
    println!("{name:<16}: {}", Q64::FRAC_TAU_2, name = "FRAC_TAU_2");
    println!("{name:<16}: {}", Q64::FRAC_TAU_3, name = "FRAC_TAU_3");
    println!("{name:<16}: {}", Q64::FRAC_TAU_4, name = "FRAC_TAU_4");
    println!("{name:<16}: {}", Q64::FRAC_TAU_6, name = "FRAC_TAU_6");
    println!("{name:<16}: {}", Q64::FRAC_TAU_8, name = "FRAC_TAU_8");
    println!("{name:<16}: {}\n", Q64::FRAC_TAU_12, name = "FRAC_TAU_12");

    println!("{name:<16}: {}", Q64::E, name = "E");
    println!("{name:<16}: {}", Q64::SQRT_E, name = "SQRT_E");
    println!("{name:<16}: {}", Q64::LOG2_E, name = "LOG2_E");
    println!("{name:<16}: {}\n", Q64::LOG10_E, name = "LOG10_E");

    println!("{name:<16}: {}", Q64::PHI, name = "PHI");
    println!("{name:<16}: {}", Q64::SQRT_PHI, name = "SQRT_PHI");
    println!("{name:<16}: {}\n", Q64::FRAC_1_PHI, name = "FRAC_1_PHI");

    println!("{name:<16}: {}\n", Q64::GAMMA, name = "GAMMA");
    println!("{name:<16}: {}\n", Q64::CATALAN, name = "CATALAN");

    println!("{name:<16}: {}", Q64::IS_SIGNED, name = "IS_SIGNED");
    println!("{name:<16}: {}", Q64::INT_NBITS, name = "INT_NBITS");
    println!("{name:<16}: {}\n", Q64::FRAC_NBITS, name = "FRAC_NBITS");

    println!("COORDIC");
    println!("SIN COS 0: {:?}", Q64::ZERO.sin_cos());
    println!("SIN COS 30: {:?}", Q64::FRAC_PI_6.sin_cos());
    println!("SIN COS -30: {:?}", (-Q64::FRAC_PI_6).sin_cos());
    println!("SIN COS 180: {:?}", Q64::PI.sin_cos());
    println!("SIN COS -180: {:?}", Q64::NEG_PI.sin_cos());
    println!("SIN COS 360: {:?}", Q64::TWO_PI.sin_cos());
    println!("SIN COS -360: {:?}", Q64::NEG_TWO_PI.sin_cos());
    println!("ASIN ACOS 0: {:?} {:?}", Q64::ZERO.asin(), Q64::ZERO.acos());
    println!("ASIN ACOS 1: {:?} {:?}", Q64::ONE.asin(), Q64::ONE.acos());
    println!("ATAN 1: {:?}", Q64::ONE.atan());
    println!("ATAN 0: {:?}", Q64::ZERO.atan());
    println!("ATAN -1: {:?}\n", Q64::NEG_ONE.atan());

    // Error accumulation.
    let mut norm_vec2_x = QVec2::X;
    for _ in 0..10000 {
        norm_vec2_x = QVec2::from_angle(Q64::PI / 6).rotate(norm_vec2_x);
    }
    println!("Unit X after rotate 100: {:?} {}", norm_vec2_x, norm_vec2_x.length());
    for _ in 0..90000 {
        norm_vec2_x = QVec2::from_angle(Q64::PI / 6).rotate(norm_vec2_x);
    }
    println!("Unit X after rotate 10000: {:?} {}", norm_vec2_x, norm_vec2_x.length());

    println!("DELTA divide by MAX: {}", Q64::DELTA / Q64::MAX);
    println!("MAX divide by DELTA: {}", Q64::MAX.saturating_div(Q64::DELTA));
}