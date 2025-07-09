import csv
import pytest
from typing import Optional, Tuple
from jacobian import jacobian_add
from affine import P

JacPoint = Tuple[int, int, int]
AffPoint = Optional[Tuple[int, int]]

def parse_affine(x: str, y: str) -> AffPoint:
    """Parse (x, y) strings to an AffPoint, or None if x is empty."""
    return None if not x else (int(x) % P, int(y) % P)

# Load CSV test data once
csv_path = '../test_vectors/jacobian_test_vectors.csv'
with open(csv_path, newline='') as f:
    test_data = list(csv.DictReader(f))

@pytest.mark.parametrize('row', test_data)
def test_jacobian(row):
    p: JacPoint = (int(row['X1']), int(row['Y1']), int(row['Z1']))
    q: JacPoint = (int(row['X2']), int(row['Y2']), int(row['Z2']))
    expected = parse_affine(row['expected_x'], row['expected_y'])

    result = jacobian_add(p, q)
    assert result == expected, f"Failed on:\nP = {p}\nQ = {q}\nExpected = {expected}, Got = {result}"
