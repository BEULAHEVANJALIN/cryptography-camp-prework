import csv
import pytest
from affine import P
from scalar import affine_scalar_mul, jacobian_scalar_mul
from typing import Tuple

Point = Tuple[int, int]

def parse_point(row) -> Tuple[int, Point]:
    """Parse a row into (scalar, point) and expected result, reduced mod P."""
    k = int(row['k'])
    point = (int(row['X']) % P, int(row['Y']) % P)
    expected = (int(row['expected_x']) % P, int(row['expected_y']) % P)
    return k, point, expected

# Load test vectors
csv_path = '../test_vectors/scalar_test_vectors.csv'
with open(csv_path, newline='') as f:
    test_data = list(csv.DictReader(f))

@pytest.mark.parametrize("row", test_data)
def test_scalar_variants(row):
    k, point, expected = parse_point(row)

    result_affine = affine_scalar_mul(k, point)
    result_jacobian = jacobian_scalar_mul(k, point)

    assert result_affine == expected, (
        f"Affine scalar mul failed:\n"
        f"k = {k}\nP = {point}\nExpected = {expected}, Got = {result_affine}"
    )
    assert result_jacobian == expected, (
        f"Jacobian scalar mul failed:\n"
        f"k = {k}\nP = {point}\nExpected = {expected}, Got = {result_jacobian}"
    )