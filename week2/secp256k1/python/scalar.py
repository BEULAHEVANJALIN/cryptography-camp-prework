from typing import Optional, Tuple
from affine import affine_add
from jacobian import jacobian_add_jac, jacobian_double, to_affine

# Types
Point = Optional[Tuple[int, int]]
JacPoint = Tuple[int, int, int]


def affine_scalar_mul(k: int, point: Point) -> Point:
    """Scalar multiplication using double-and-add in affine coordinates."""
    if k == 0 or point is None:
        return None

    result: Point = None
    addend = point

    for bit in bin(k)[2:]:  # Skip the '0b' prefix
        if result is not None:
            result = affine_add(result, result)  # Always double
        if bit == '1':
            result = affine_add(result, addend)  # Conditionally add

    return result


def jacobian_scalar_mul(k: int, point: Tuple[int, int]) -> Point:
    """Scalar multiplication using double-and-add in Jacobian coordinates."""
    if k == 0:
        return None

    result: Optional[JacPoint] = None
    addend: JacPoint = (point[0], point[1], 1)  # Lift to Jacobian

    for bit in bin(k)[2:]:  # Iterate from MSB to LSB
        if result is not None:
            result = jacobian_double(result)  # Always double
        if bit == '1':
            result = addend if result is None else jacobian_add_jac(result, addend)

    return to_affine(result)
