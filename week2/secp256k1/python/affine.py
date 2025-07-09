from typing import Optional, Tuple

# secp256k1 field modulus
P = 2**256 - 2**32 - 977

def inv(a: int) -> int:
    """Modular inverse using Fermat's Little Theorem: a⁻¹ ≡ a^(p−2) mod p"""
    return pow(a, P - 2, P)

def affine_add(
    p1: Optional[Tuple[int, int]],
    p2: Optional[Tuple[int, int]]
) -> Optional[Tuple[int, int]]:
    """
    Add two points in affine coordinates on the secp256k1 elliptic curve.
    Returns None for the point at infinity.
    """
    if p1 is None:
        return p2
    if p2 is None:
        return p1

    x1, y1 = p1
    x2, y2 = p2

    # P + (-P) = ∞
    if x1 == x2 and (y1 + y2) % P == 0:
        return None

    if x1 != x2:
        # λ = (y₂ - y₁) / (x₂ - x₁)
        lam = (y2 - y1) * inv(x2 - x1) % P
    else:
        # Point doubling: λ = (3x₁²) / (2y₁)
        lam = (3 * x1 * x1) * inv(2 * y1) % P

    # x₃ = λ² − x₁ − x₂
    xr = (lam * lam - x1 - x2) % P
    # y₃ = λ(x₁ − x₃) − y₁
    yr = (lam * (x1 - xr) - y1) % P

    return (xr, yr)
