use crate::Q64;
use fixed::traits::Fixed;

/// Elementary functions.
pub trait QBasic: Fixed {
    /// A very small number.
    const EPS: Self;
    /// Half.
    const HALF: Self;
    /// One plus one...
    const TWO: Self;
    /// Negative half.
    const NEG_HALF: Self;
    /// Negative two.
    const NEG_TWO: Self;
    /// Negative DELTA.
    const NEG_DELTA: Self;
    /// Two times PI.
    const TWO_PI: Self;
    /// Negative PI.
    const NEG_PI: Self;
    /// Negative two times PI.
    const NEG_TWO_PI: Self;

    /// Computes and returns the square root of a number.
    #[deprecated(
        since = "0.1.0",
        note = "Use Q64::sqrt instead."
    )]
    fn sqrt(self) -> Self;
    /// Computes and returns the nth power of a number.
    fn powi(self, n: isize) -> Self;
    /// Computes and returns the exponential function of a number.
    fn exp(self) -> Self;
    /// Half of a number.
    fn half(self) -> Self;
}

impl QBasic for Q64 {
    const EPS: Self = Self::lit("1.0e-8");
    const HALF: Self = Self::lit("0.5");
    const TWO: Self = Self::lit("2");
    const NEG_HALF: Self = Self::lit("-0.5");
    const NEG_TWO: Self = Self::lit("-2");
    const NEG_DELTA: Self = Self::lit("-0.0000000002");
    const NEG_PI: Self = Self::lit("-3.1415926535");
    const TWO_PI: Self = Self::lit("6.283185307");
    const NEG_TWO_PI: Self = Self::lit("-6.283185307");
    
    /// # Examples
    /// 
    /// ```rust
    /// use qmath::prelude::*;
    /// let sqrt_pi = Q64::PI.sqrt();
    /// assert!(sqrt_pi.abs_diff(Q64::SQRT_PI) <= Q64::EPS);
    /// ```
    /// 
    /// # Panics
    ///
    /// ```rust,should_panic
    /// use qmath::prelude::*;
    /// let _ = Q64::NEG_ONE.sqrt();
    /// ```
    fn sqrt(self) -> Self {
        if self.signum() == Q64::NEG_ONE {
            return Q64::ZERO;
        }
        if self == Self::ZERO { return self; }

        let mut x = self;

        loop {
            let next_x = (x + self / x) * Self::HALF;
            if (next_x - x).abs() <= Self::EPS { break; }
            x = next_x;
        }

        x
    }

    /// # Examples
    /// 
    /// ```rust
    /// use qmath::prelude::*;
    /// 
    /// let max_pow_neg_one = Q64::MAX.powi(-1);
    /// let one_div_max = Q64::ONE / Q64::MAX;
    /// assert_eq!(max_pow_neg_one, one_div_max);
    /// 
    /// let pi_pow_neg_two = Q64::PI.powi(-2);
    /// let one_div_pi_mul_pi = Q64::ONE / Q64::PI.powi(2);
    /// assert_eq!(pi_pow_neg_two, one_div_pi_mul_pi);
    /// ```
    fn powi(self, n: isize) -> Self {
        if n == Self::ZERO {
            return Self::ONE;
        }
        
        let mut result = Self::ONE;
        let mut abs_exponent = n.abs();
        let mut current_power = self;
    
        while abs_exponent > 0 {
            if abs_exponent == 1 {
                result = result * current_power;
                break;
            }
            current_power = current_power * current_power;
            abs_exponent -= 1;
        }
    
        if n < 0 {
            result = result.recip();
        }
    
        result
    }
    
    /// # Examples
    /// 
    /// ```rust
    /// use qmath::prelude::*;
    /// let e = Q64::ONE.exp();
    /// assert!(e.abs_diff(Q64::E) <= Q64::EPS);
    /// ```
    fn exp(self) -> Self {
        let mut rst = Self::ONE;
        let mut term = Self::ONE;
        for i in 1..=12 {
            let temp = self / i;
            term = term * temp;
            rst = rst + term;
        }
        rst
    }

    fn half(self) -> Self {
        self.saturating_mul(Self::HALF)
    }
}