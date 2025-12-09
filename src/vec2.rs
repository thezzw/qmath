use crate::prelude::*;

use core::fmt;
use core::ops::*;

/// A 2-dimensional vector.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct QVec2 {
    pub x: Q64,
    pub y: Q64,
}

impl QVec2 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(Q64::ZERO);

    /// All ones.
    pub const ONE: Self = Self::splat(Q64::ONE);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(Q64::NEG_ONE);

    /// All `Q64::MIN`.
    pub const MIN: Self = Self::splat(Q64::MIN);

    /// All `Q64::MAX`.
    pub const MAX: Self = Self::splat(Q64::MAX);

    /// A unit vector pointing along the positive X axis.
    pub const X: Self = Self::new(Q64::ONE, Q64::ZERO);

    /// A unit vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(Q64::ZERO, Q64::ONE);

    /// A unit vector pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(Q64::NEG_ONE, Q64::ZERO);

    /// A unit vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(Q64::ZERO, Q64::NEG_ONE);

    /// The unit axes.
    pub const AXES: [Self; 2] = [Self::X, Self::Y];

    /// All `Q64::EPS`.
    pub const EPS: Self = Self::splat(Q64::EPS);

    /// `Q64::EPS` in the X axis, 'Q64::ZERO' in the Y axis.
    pub const EPS_X: Self = Self::new(Q64::EPS, Q64::ZERO);

    /// `Q64::ZERO` in the X axis, 'Q64::EPS' in the Y axis.
    pub const EPS_Y: Self = Self::new(Q64::ZERO, Q64::EPS);

    /// All `Q64::DELTA`.
    pub const DELTA: Self = Self::splat(Q64::DELTA);

    /// `Q64::DELTA` in the X axis, 'Q64::ZERO' in the Y axis.
    pub const DELTA_X: Self = Self::new(Q64::DELTA, Q64::ZERO);

    /// `Q64::ZERO` in the X axis, 'Q64::DELTA' in the Y axis.
    pub const DELTA_Y: Self = Self::new(Q64::ZERO, Q64::DELTA);

    /// Creates a new vector.
    #[inline(always)]
    #[must_use]
    pub const fn new(x: Q64, y: Q64) -> Self {
        Self { x, y }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: Q64) -> Self {
        Self { x: v, y: v }
    }

    /// Saturating addition.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn saturating_add(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_add(rhs.x),
            y: self.y.saturating_add(rhs.y),
        }
    }

    /// Saturating subtraction.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn saturating_sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }

    /// Saturating multiplication.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn saturating_mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_mul(rhs.x),
            y: self.y.saturating_mul(rhs.y),
        }
    }

    /// Saturating division.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn saturating_div(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_div(rhs.x),
            y: self.y.saturating_div(rhs.y),
        }
    }

    /// Saturating addition.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn saturating_add_num(self, rhs: Q64) -> Self {
        Self {
            x: self.x.saturating_add(rhs),
            y: self.y.saturating_add(rhs),
        }
    }

    /// Saturating subtraction.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn saturating_sub_num(self, rhs: Q64) -> Self {
        Self {
            x: self.x.saturating_sub(rhs),
            y: self.y.saturating_sub(rhs),
        }
    }

    /// Saturating multiplication.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn saturating_mul_num(self, rhs: Q64) -> Self {
        Self {
            x: self.x.saturating_mul(rhs),
            y: self.y.saturating_mul(rhs),
        }
    }

    /// Saturating division.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn saturating_div_num(self, rhs: Q64) -> Self {
        Self {
            x: self.x.saturating_div(rhs),
            y: self.y.saturating_div(rhs),
        }
    }

    /// Computes the dot product of `self` and `rhs`.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> Q64 {
        self.x.saturating_mul_add(rhs.x, self.y.saturating_mul(rhs.y))
    }

    /// Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }

    /// Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[inline]
    #[must_use]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    /// Component-wise clamping of values, similar to [`Q64::clamp`].
    ///
    /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min.x.le(&max.x) && min.y.le(&max.y), "[QVec2::clamp] Expected min <= max.");
        self.max(min).min(max)
    }

    /// Returns the horizontal minimum of `self`.
    ///
    /// In other words this computes `min(x, y, ..)`.
    #[inline]
    #[must_use]
    pub fn min_element(self) -> Q64 {
        self.x.min(self.y)
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline]
    #[must_use]
    pub fn max_element(self) -> Q64 {
        self.x.max(self.y)
    }

    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Computes the length of `self`.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn length(self) -> Q64 {
        self.dot(self).saturating_sqrt()
    }

    /// Computes the squared length of `self`.
    /// 
    /// This is faster than `length()` as it avoids a square root operation.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn length_squared(self) -> Q64 {
        self.dot(self)
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must not be of length zero.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn length_recip(self) -> Q64 {
        self.length().saturating_recip()
    }

    /// Computes the Euclidean distance between two points in space.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn distance(self, rhs: Self) -> Q64 {
        self.saturating_sub(rhs).length()
    }

    /// Compute the squared euclidean distance between two points in space.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn distance_squared(self, rhs: Self) -> Q64 {
        self.saturating_sub(rhs).length_squared()
    }

    /// Returns `self` normalized to length 1.0.
    /// 
    /// For valid results, `self` must not be of length zero, nor very close to zero.
    /// 
    /// # Panics
    ///
    /// Will panic if `self` is zero length when `glam_assert` is enabled.
    /// 
    /// SAT
    pub fn normalize(self) -> Self {
        self.saturating_mul_num(self.length_recip())
    }

    /// Returns whether `self` is length `1.0` or not.
    ///
    /// SAT
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool {
        self.length_squared().saturating_sub(Q64::ONE).abs() <= Q64::EPS
    }

    /// Returns a vector containing the nearest integer to a number for each element of `self`.
    /// Round half-way cases away from 0.0.
    #[inline]
    #[must_use]
    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    /// Returns a vector containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline]
    #[must_use]
    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    /// Returns a vector containing the smallest integer greater than or equal to a number for
    /// each element of `self`.
    #[inline]
    #[must_use]
    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    /// Returns a vector containing the integer part each element of `self`. This means numbers are
    /// always truncated towards zero.
    #[inline]
    #[must_use]
    pub fn trunc(self) -> Self {
        Self {
            x: self.x.round_to_zero(),
            y: self.y.round_to_zero(),
        }
    }

    /// Returns a vector containing the fractional part of the vector, e.g. `self -
    /// self.floor()`.
    ///
    /// Note that this is fast but not precise for large numbers.
    #[inline]
    #[must_use]
    pub fn fract(self) -> Self {
        self - self.floor()
    }

    /// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    /// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    /// extrapolated.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn lerp(self, rhs: Self, s: Q64) -> Self {
        self.saturating_add(rhs.saturating_sub(self).saturating_mul_num(s))
    }

    /// Calculates the midpoint between `self` and `rhs`.
    ///
    /// The midpoint is the average of, or halfway point between, two vectors.
    /// `a.midpoint(b)` should yield the same result as `a.lerp(b, 0.5)`
    /// while being slightly cheaper to compute.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn midpoint(self, rhs: Self) -> Self {
        self.saturating_add(rhs).saturating_mul_num(Q64::HALF)
    }

    /// Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
    /// conjunction with the [`rotate()`][Self::rotate()] method, e.g.
    /// `QVec2::from_angle(PI).rotate(QVec2::Y)` will create the vector `[-1, 0]`
    /// and rotate [`QVec2::Y`] around it returning `-QVec2::Y`.
    #[inline]
    #[must_use]
    pub fn from_angle(angle: Q64) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self { x: cos, y: sin }
    }

    /// Returns the angle (in radians) of this vector in the range `[-π, +π]`.
    ///
    /// The input does not need to be a unit vector however it must be non-zero.
    /// 
    /// # Panics
    /// 
    /// Will panic if `self` is `QVec2::ZERO`.
    #[inline]
    #[must_use]
    pub fn to_angle(self) -> Q64 {
        assert!(self.x.ne(&Q64::ZERO) || self.y.ne(&Q64::ZERO), "[QVec2::to_angle] Computeing angle of zero vector, zero is returned.");
        Q64::atan2(self.y, self.x)
    }

    /// Returns the angle (in radians) between `self` and `rhs` in the range `[-π, +π]`.
    ///
    /// The inputs do not need to be unit vectors however they must be non-zero.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn angle_between(self, rhs: Self) -> Q64 {
        let hypotenuse_length = (self.length_squared().saturating_mul(rhs.length_squared())).saturating_sqrt();
        assert!(hypotenuse_length.ne(&Q64::ZERO), "[QVec2::angle_between] Computeing angle between zero vectors: {:?} {:?}.", self, rhs);
        let angle = (
            self.dot(rhs).saturating_div(hypotenuse_length)
        ).clamp(Q64::NEG_ONE, Q64::ONE).acos().1;

        angle * (self.cross(rhs)).signum()
    }

    /// Returns a vector that is equal to `self` rotated by 90 degrees.
    #[inline]
    #[must_use]
    pub fn perp(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// The perpendicular dot product of `self` and `rhs`.
    /// Also known as the wedge product, 2D cross product, and determinant.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn cross(self, rhs: Self) -> Q64 {
        self.x.saturating_mul(rhs.y).saturating_sub(self.y.saturating_mul(rhs.x))
    }

    /// Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
    /// then this just rotation. This is what you usually want. Otherwise,
    /// it will be like a rotation with a multiplication by `self`'s length.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn rotate(self, rhs: Self) -> Self {
        Self {
            x: self.x.saturating_mul(rhs.x).saturating_sub(self.y.saturating_mul(rhs.y)),
            y: self.y.saturating_mul(rhs.x).saturating_add(self.x.saturating_mul(rhs.y)),
        }
    }

    /// Returns true if the absolute difference of all elements between `self` and `rhs` is
    /// less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two vectors contain similar elements. It works best when
    /// comparing with a known value. The `max_abs_diff` that should be used used depends on
    /// the values being compared against.
    /// 
    /// SAT
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: Q64) -> bool {
        let dif = self.saturating_sub(rhs).abs();
        let gap = Self::splat(max_abs_diff);
        dif.x.le(&gap.x) && dif.y.le(&gap.y)
    }
}

impl Default for QVec2 {
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Add<QVec2> for QVec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<QVec2> for QVec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<QVec2> for QVec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div<QVec2> for QVec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl AddAssign<QVec2> for QVec2 {
    #[inline]
    fn add_assign(&mut self, rhs: QVec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<QVec2> for QVec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: QVec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl MulAssign<QVec2> for QVec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: QVec2) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl DivAssign<QVec2> for QVec2 {
    #[inline]
    fn div_assign(&mut self, rhs: QVec2) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Add<Q64> for QVec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Q64) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Sub<Q64> for QVec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Q64) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl Mul<Q64> for QVec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Q64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<Q64> for QVec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Q64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl AddAssign<Q64> for QVec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Q64) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl SubAssign<Q64> for QVec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Q64) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl MulAssign<Q64> for QVec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Q64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl DivAssign<Q64> for QVec2 {
    #[inline]
    fn div_assign(&mut self, rhs: Q64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Rem<QVec2> for QVec2 { 
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}

impl Neg for QVec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

impl fmt::Display for QVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl fmt::Debug for QVec2 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(QVec2))
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl From<[Q64; 2]> for QVec2 {
    #[inline]
    fn from(a: [Q64; 2]) -> Self {
        Self::new(a[0], a[1])
    }
}

impl From<QVec2> for [Q64; 2] {
    #[inline]
    fn from(v: QVec2) -> Self {
        [v.x, v.y]
    }
}

impl From<(Q64, Q64)> for QVec2 {
    #[inline]
    fn from(t: (Q64, Q64)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<QVec2> for (Q64, Q64) {
    #[inline]
    fn from(v: QVec2) -> Self {
        (v.x, v.y)
    }
}