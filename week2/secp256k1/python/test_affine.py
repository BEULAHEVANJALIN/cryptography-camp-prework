import csv
import pytest
from affine import affine_add, P
from typing import Optional, Tuple

Point = Optional[Tuple[int, int]]

def parse_point(x: str, y: str) -> Point:
    """Convert two CSV fields to a point or None."""
    return None if not x else (int(x) % P, int(y) % P)

# Load test vectors
csv_path = '../test_vectors/affine_test_vectors.csv'
with open(csv_path, newline='') as f:
    test_data = list(csv.DictReader(f))

@pytest.mark.parametrize("row", test_data)
def test_affine(row):
    p1 = parse_point(row["x1"], row["y1"])
    p2 = parse_point(row["x2"], row["y2"])
    expected = parse_point(row["expected_x"], row["expected_y"])
    result = affine_add(p1, p2)
    assert result == expected, f"Expected {expected}, got {result} for P1={p1}, P2={p2}"
