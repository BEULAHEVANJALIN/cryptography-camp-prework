use num_bigint::BigUint;
use num_traits::{One, Zero};
use once_cell::sync::Lazy;

// Field modulus p = 2^256 - 2^32 - 977
pub static P: Lazy<BigUint> =
    Lazy::new(|| (BigUint::one() << 256) - (BigUint::one() << 32) - BigUint::from(977u32));

pub fn modp() -> &'static BigUint {
    &*P
}
/// Affine point or infinity
pub type Point = Option<(BigUint, BigUint)>;

/// Modular addition: (a + b) mod p
pub fn mod_add(a: &BigUint, b: &BigUint) -> BigUint {
    (a + b) % modp()
}

/// Modular subtraction: (a - b) mod p
pub fn mod_sub(a: &BigUint, b: &BigUint) -> BigUint {
    if a >= b {
        (a - b) % modp()
    } else {
        (modp() + a - b) % modp()
    }
}

/// Modular multiplication: (a * b) mod p
pub fn mod_mul(a: &BigUint, b: &BigUint) -> BigUint {
    (a * b) % modp()
}

/// Modular inverse: a^{-1} mod p
pub fn mod_inv(a: &BigUint) -> BigUint {
    assert!(!a.is_zero(), "modular inverse of zero is undefined");
    a.modpow(&(modp() - BigUint::from(2u32)), modp())
}

/// secp256k1 affine addition
pub fn affine_add(a: &Point, b: &Point) -> Point {
    match (a, b) {
        (None, None) => None,
        (None, Some(q)) => Some(q.clone()),
        (Some(p), None) => Some(p.clone()),
        (Some((x1, y1)), Some((x2, y2))) => {
            if x1 == x2 && mod_add(y1, y2).is_zero() {
                return None;
            }

            let lam = if x1 != x2 {
                let num = mod_sub(y2, y1);
                let den = mod_sub(x2, x1);
                mod_mul(&num, &mod_inv(&den))
            } else {
                let num = mod_mul(&BigUint::from(3u32), &mod_mul(x1, x1));
                let den = mod_mul(&BigUint::from(2u32), y1);
                mod_mul(&num, &mod_inv(&den))
            };

            let xr = mod_sub(&mod_sub(&mod_mul(&lam, &lam), x1), x2);
            let yr = mod_sub(&mod_mul(&lam, &mod_sub(x1, &xr)), y1);

            Some((xr, yr))
        }
    }
}
