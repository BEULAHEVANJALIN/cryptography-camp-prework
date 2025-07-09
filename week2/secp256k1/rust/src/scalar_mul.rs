use num_bigint::BigUint;
use num_traits::{One, Zero};

use crate::affine::affine_add;
use crate::jacobian::{jacobian_add, jacobian_double, to_affine};

/// Scalar multiplication variants
pub mod scalar_mul {
    use super::*;

    /// Affine double-and-add scalar multiplication
    pub fn affine_scalar_mul(
        k: &BigUint,
        point: &Option<(BigUint, BigUint)>,
    ) -> Option<(BigUint, BigUint)> {
        if k.is_zero() || point.is_none() {
            return None;
        }

        let mut result: Option<(BigUint, BigUint)> = None;
        let addend = point.clone();

        for i in (0..k.bits()).rev() {
            if result.is_some() {
                result = affine_add(&result, &result); // double
            }
            if k.bit(i) {
                result = affine_add(&result, &addend); // add
            }
        }

        result
    }

    /// Jacobian double-and-add scalar multiplication
    pub fn jacobian_scalar_mul(
        k: &BigUint,
        point: &(BigUint, BigUint),
    ) -> Option<(BigUint, BigUint)> {
        if k.is_zero() {
            return None;
        }

        // lift point to Jacobian coordinates (X, Y, Z = 1)
        let mut result: Option<(BigUint, BigUint, BigUint)> = None;
        let addend = (point.0.clone(), point.1.clone(), BigUint::one());

        for i in (0..k.bits()).rev() {
            if let Some(r) = &result {
                result = jacobian_double(r).map(|(x, y)| (x, y, BigUint::one()));
            }

            if k.bit(i) {
                result = Some(match &result {
                    Some(r) => {
                        let (x, y) = jacobian_add(r, &addend)?;
                        (x, y, BigUint::one())
                    }
                    None => addend.clone(),
                });
            }
        }

        result.and_then(|r| to_affine(&r))
    }
}
