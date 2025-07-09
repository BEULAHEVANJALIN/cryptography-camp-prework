from typing import Optional, Tuple
from affine import affine_add, inv, P

JacPoint = Tuple[int, int, int]
AffPoint = Optional[Tuple[int, int]]


def to_affine(p: JacPoint) -> AffPoint:
    """Convert Jacobian point (X:Y:Z) to affine coordinates (x, y)."""
    X, Y, Z = p
    if Z == 0:
        return None
    z_inv = inv(Z)
    z2 = (z_inv * z_inv) % P
    z3 = (z2 * z_inv) % P
    return (X * z2 % P, Y * z3 % P)


def jacobian_double(p: JacPoint) -> JacPoint:
    """Double a point in Jacobian coordinates."""
    X1, Y1, Z1 = p
    if Y1 == 0:
        return (0, 1, 0)  # Point at infinity

    Y1_sq = Y1 * Y1 % P
    Y1_4 = Y1_sq * Y1_sq % P
    S = 4 * X1 * Y1_sq % P
    M = 3 * X1 * X1 % P
    X3 = (M * M - 2 * S) % P
    Y3 = (M * (S - X3) - 8 * Y1_4) % P
    Z3 = (2 * Y1 * Z1) % P
    return (X3, Y3, Z3)


def jacobian_add_jac(p: JacPoint, q: JacPoint) -> JacPoint:
    """Add two Jacobian points, returning a Jacobian point."""
    X1, Y1, Z1 = p
    X2, Y2, Z2 = q

    if Z1 == 0:
        return q
    if Z2 == 0:
        return p

    Z1_sq = Z1 * Z1 % P
    Z2_sq = Z2 * Z2 % P
    Z1_cu = Z1_sq * Z1 % P
    Z2_cu = Z2_sq * Z2 % P

    U1 = X1 * Z2_sq % P
    U2 = X2 * Z1_sq % P
    S1 = Y1 * Z2_cu % P
    S2 = Y2 * Z1_cu % P

    if U1 == U2:
        if S1 != S2:
            return (0, 1, 0)  # Point at infinity
        return jacobian_double(p)

    H = (U2 - U1) % P
    R = (S2 - S1) % P
    H_sq = H * H % P
    H_cu = H_sq * H % P
    U1_H_sq = U1 * H_sq % P

    X3 = (R * R - H_cu - 2 * U1_H_sq) % P
    Y3 = (R * (U1_H_sq - X3) - S1 * H_cu) % P
    Z3 = Z1 * Z2 * H % P

    return (X3, Y3, Z3)


def jacobian_add(p: JacPoint, q: JacPoint) -> AffPoint:
    """Add two Jacobian points, returning an affine point."""
    return to_affine(jacobian_add_jac(p, q))