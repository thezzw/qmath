use crate::Q64;
use crate::basic::QBasic;
use fixed::{ types::I4F60, traits::Fixed };

const CIRCLE_I4F60: [(i8, i64, i64); 32] = [
    (0, 905502432259640320, 815238614083298944),
    (1, 534549298976576448, 1031204342808898688),
    (2, 282441168888798112, 1118498150950604288),
    (3, 143371547418228448, 1144018502608809088),
    (4, 71963988336308048, 1150676280461235072),
    (5, 36017075762092180, 1152358966635028224),
    (6, 18012932708689206, 1152780792883053696),
    (7, 9007016009513623, 1152886321845288960),
    (8, 4503576721087964, 1152912708614486784),
    (9, 2251796950380271, 1152919305589882880),
    (10, 1125899548928888, 1152920954851426304),
    (11, 562949908682076, 1152921367167918080),
    (12, 281474971118251, 1152921470247110144),
    (13, 140737487656277, 1152921496016912512),
    (14, 70368744090283, 1152921502459363328),
    (15, 35184372077909, 1152921504069976064),
    (16, 17592186043051, 1152921504472629248),
    (17, 8796093022037, 1152921504573292544),
    (18, 4398046511083, 1152921504598458368),
    (19, 2199023255549, 1152921504604749824),
    (20, 1099511627776, 1152921504606322688),
    (21, 549755813888, 1152921504606715904),
    (22, 274877906944, 1152921504606814208),
    (23, 137438953472, 1152921504606838784),
    (24, 68719476736, 1152921504606844928),
    (25, 34359738368, 1152921504606846464),
    (26, 17179869184, 1152921504606846848),
    (27, 8589934592, 1152921504606846976),
    (28, 4294967296, 1152921504606846976),
    (29, 2147483648, 1152921504606846976),
    (30, 1073741824, 1152921504606846976),
    (31, 536870912, 1152921504606846976)
];

const CIRCLE_KN_I4F60: i64 = 700114967507363456;

/// Trigonometric functions.
pub trait QCoordic: Fixed {
    /// Returns the Sin and Cos values of an angle in radians as a tuple.
    fn sin_cos(self) -> (Self, Self);
    /// Returns the Sin value of an angle in radians.
    fn sin(self) -> Self;
    /// Returns the Cos value of an angle in radians.
    fn cos(self) -> Self;
    /// Returns the Tan value of an angle in radians.
    fn tan(self) -> Self;
    /// Returns two angles in radians corresponding to the Sin value within [-PI, PI].
    fn asin(self) -> (Self, Self);
    /// Returns two angles in radians corresponding to the Cos value within [-PI, PI].
    fn acos(self) -> (Self, Self);
    /// Returns two angles in radians corresponding to the Tan value within [-PI, PI].
    fn atan(self) -> (Self, Self);
    /// Returns one angle in radians corresponding to the Tan value (y / x) within [-PI, PI].
    fn atan2(y: Self, x: Self) -> Self;
}

impl QCoordic for Q64 {
    fn sin_cos(self) -> (Self, Self) {
        // Converts result angle's value into range [-PI/2, PI/2].
        let mut result_factor = Self::from_num(I4F60::from_bits(CIRCLE_KN_I4F60));
        let mut preliminary_angle = self % ( 2 * Self::PI );
        
        if preliminary_angle < 0 { preliminary_angle += 2 * Self::PI }
        if preliminary_angle > Self::PI / 2 && preliminary_angle < Self::PI * 3 / 2 {
            result_factor = -result_factor;
            if preliminary_angle < Self::PI { preliminary_angle += Self::PI }
            else { preliminary_angle -= Self::PI }
        }
        if preliminary_angle >= Self::PI * 3 / 2 { preliminary_angle -= 2 * Self::PI; }
        
        // Coordinates rotate.
        let mut x = Self::ONE;
        let mut y = Self::ZERO;
        let mut remain = preliminary_angle;
        
        let (mut xv, mut yv);
        for (k, rad, _) in CIRCLE_I4F60 {
            if remain > 0 { // CCW.
                xv = x - (y >> k);
                yv = (x >> k) + y;
                remain -= Self::from_num(I4F60::from_bits(rad));
            } else { // CW.
                xv = x + (y >> k);
                yv = y - (x >> k);
                remain += Self::from_num(I4F60::from_bits(rad));
            }
            (x, y) = (xv, yv);
        }
        
        let result = (y * result_factor, x * result_factor);
        
        result
    }
        
    fn sin(self) -> Self { self.sin_cos().0 }
        
    fn cos(self) -> Self { self.sin_cos().1 }
        
    fn tan(self) -> Self { 
        let sin_cos = self.sin_cos();
        if sin_cos.1.abs() <= Self::EPS { 
            if sin_cos.0 > 0 { return Self::MAX; }
            else { return Self::MIN; }
        }
        sin_cos.0 / sin_cos.1
    }

    /// # Panics
    ///
    /// ```rust,should_panic
    /// use qmath::prelude::*;
    /// let _ = Q64::TWO.asin();
    /// let _ = Q64::NEG_TWO.asin();
    /// ```
    fn asin(self) -> (Self, Self) {
        assert!(self <= 1 && self >= -1, "[QCoordic::asin] Sine value should be in range of [-1, 1].");
        
        let mut x = Self::ONE;
        let mut y = Self::ZERO;
        let mut z = Self::ZERO;
        
        let (mut xv, mut yv);
        for (k, rad, cos) in CIRCLE_I4F60 {
            if (x > 0 && self > y) || (x < 0 && self < y) {
                xv = x - (y >> k);
                yv = (x >> k) + y;
                z += Self::from_num(I4F60::from_bits(rad));
            } else {
                xv = x + (y >> k);
                yv = y - (x >> k);
                z -= Self::from_num(I4F60::from_bits(rad));
            }
            let cos = Self::from_num(I4F60::from_bits(cos));
            (x, y) = (xv * cos, yv * cos);
        }
        
        let result ;
        if z > 0 {
            result = (z, -z + Self::PI);
        } else {
            result = (-z - Self::PI, z);
        }
        result
    }
    
    /// # Panics
    ///
    /// ```rust,should_panic
    /// use qmath::prelude::*;
    /// let _ = Q64::TWO.acos();
    /// let _ = Q64::NEG_TWO.acos();
    /// ```
    fn acos(self) -> (Self, Self) {
        assert!(self <= 1 && self >= -1, "[QCoordic::acos] Can't acos value {}, cosine value should be in range of [-1, 1].", self);
        
        let mut x = Self::ZERO;
        let mut y = Self::ONE;
        let mut z = Self::PI / 2;
        
        let (mut xv, mut yv);
        for (k, rad, cos) in CIRCLE_I4F60 {
            if (y > 0 && self < x) || (y < 0 && self > x) {
                xv = x - (y >> k);
                yv = (x >> k) + y;
                z += Self::from_num(I4F60::from_bits(rad));
            } else {
                xv = x + (y >> k);
                yv = y - (x >> k);
                z -= Self::from_num(I4F60::from_bits(rad));
            }
            let cos = Self::from_num(I4F60::from_bits(cos));
            (x, y) = (xv * cos, yv * cos);
        }
        
        let result = (-z, z);
        result
    }
        
    fn atan(self) -> (Self, Self) {
        let mut x = Self::ONE;
        let mut y = Self::ZERO;
        let mut z = Self::ZERO;
    
        let (mut xv, mut yv);
        for (k, rad, cos) in CIRCLE_I4F60 {
            if x > 0 && self > (y / x) {
                xv = x - (y >> k);
                yv = (x >> k) + y;
                z += Self::from_num(I4F60::from_bits(rad));
            } else {
                xv = x + (y >> k);
                yv = y - (x >> k);
                z -= Self::from_num(I4F60::from_bits(rad));
            }
    
            let cos = Self::from_num(I4F60::from_bits(cos));
            (x, y) = (xv * cos, yv * cos);
        }
    
        let result;
        if z > 0 {
            result = (z - Self::PI, z);
        } else {
            result = (z, z + Self::PI);
        }
        result
    }

    /// # Examples
    /// 
    /// ```rust
    /// use qmath::prelude::*;
    /// let atan_of_one_div_zero = Q64::atan2(Q64::ONE, Q64::ZERO);
    /// let atan_of_max = Q64::MAX.atan();
    /// assert_eq!(atan_of_one_div_zero, atan_of_max.1);
    /// ```
    /// 
    /// # Panics
    ///
    /// ```rust,should_panic
    /// use qmath::prelude::*;
    /// let _ = Q64::atan2(Q64::ZERO, Q64::ZERO);
    /// ```
    fn atan2(y: Self, x: Self) -> Self {
        assert!(y != 0 || x != 0, "[QCoordic::atan2] Both X and Y can't be zero when calculating the tan value.");
        let tan_value = if x == 0 { Self::MAX } else {
            y / x
        };
        let opt_rst = tan_value.atan();
        if y < Self::ZERO { return opt_rst.0; }
        opt_rst.1
    }
}
