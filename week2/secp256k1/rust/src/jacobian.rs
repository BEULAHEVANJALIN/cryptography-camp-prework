use crate::affine::{mod_add, mod_inv, mod_mul, mod_sub};
use num_bigint::BigUint;
use num_traits::Zero;

/// Convert Jacobian (X,Y,Z) to affine (x,y)
pub fn to_affine(j: &(BigUint, BigUint, BigUint)) -> Option<(BigUint, BigUint)> {
    let (x, y, z) = j;
    if z.is_zero() {
        return None;
    }
    let z_inv = mod_inv(z);
    let z2 = mod_mul(&z_inv, &z_inv);
    let z3 = mod_mul(&z2, &z_inv);
    Some((mod_mul(x, &z2), mod_mul(y, &z3)))
}

/// Jacobian point addition: P + Q = R (converted to affine)
pub fn jacobian_add(
    p: &(BigUint, BigUint, BigUint),
    q: &(BigUint, BigUint, BigUint),
) -> Option<(BigUint, BigUint)> {
    let (x1, y1, z1) = p;
    let (x2, y2, z2) = q;

    if z1.is_zero() {
        return to_affine(q);
    }
    if z2.is_zero() {
        return to_affine(p);
    }

    let z1z1 = mod_mul(z1, z1);
    let z2z2 = mod_mul(z2, z2);

    let u1 = mod_mul(x1, &z2z2);
    let u2 = mod_mul(x2, &z1z1);

    let z1z1z1 = mod_mul(&z1z1, z1);
    let z2z2z2 = mod_mul(&z2z2, z2);

    let s1 = mod_mul(y1, &z2z2z2);
    let s2 = mod_mul(y2, &z1z1z1);

    if u1 == u2 {
        if s1 != s2 {
            return None; // P + (-P) = ∞
        }
        return jacobian_double(p);
    }

    let h = mod_sub(&u2, &u1);
    let r = mod_sub(&s2, &s1);

    let h2 = mod_mul(&h, &h);
    let h3 = mod_mul(&h2, &h);
    let u1h2 = mod_mul(&u1, &h2);

    let x3 = mod_sub(&mod_sub(&mod_mul(&r, &r), &h3), &mod_add(&u1h2, &u1h2));
    let y3 = mod_sub(&mod_mul(&r, &mod_sub(&u1h2, &x3)), &mod_mul(&s1, &h3));
    let z3 = mod_mul(&mod_mul(z1, z2), &h);

    to_affine(&(x3, y3, z3))
}

/// Jacobian point doubling: R = 2P (converted to affine)
pub fn jacobian_double(p: &(BigUint, BigUint, BigUint)) -> Option<(BigUint, BigUint)> {
    let (x, y, z) = p;

    if y.is_zero() {
        return None; // tangent is vertical ⇒ point at infinity
    }

    let yy = mod_mul(y, y);
    let yyyy = mod_mul(&yy, &yy);
    let zz = mod_mul(z, z);

    let s = mod_mul(&mod_mul(&BigUint::from(4u32), x), &yy);
    let m = mod_add(
        &mod_mul(&BigUint::from(3u32), &mod_mul(x, x)),
        &mod_mul(&BigUint::from(0u32), &mod_mul(&zz, &zz)), // a = 0 in secp256k1
    );

    let x3 = mod_sub(&mod_mul(&m, &m), &mod_add(&s, &s));
    let y3 = mod_sub(
        &mod_mul(&m, &mod_sub(&s, &x3)),
        &mod_mul(&BigUint::from(8u32), &yyyy),
    );
    let z3 = mod_mul(y, z).clone();
    let z3 = mod_mul(&BigUint::from(2u32), &z3);

    to_affine(&(x3, y3, z3))
}
