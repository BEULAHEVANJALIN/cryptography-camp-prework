use csv::ReaderBuilder;
use num_bigint::BigUint;
use rust::affine::modp;
use rust::jacobian::jacobian_add;

#[test]
fn csv_jacobian_vectors() {
    let path = format!(
        "{}/../test_vectors/jacobian_test_vectors.csv",
        env!("CARGO_MANIFEST_DIR")
    );

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(&path)
        .expect("Failed to open jacobian_test_vectors.csv");

    for result in reader.records() {
        let record = result.expect("Failed to read CSV record");

        let parse = |i: usize| {
            record
                .get(i)
                .and_then(|s| BigUint::parse_bytes(s.as_bytes(), 10))
                .unwrap_or_else(|| panic!("Failed to parse field {}", i))
                % modp()
        };

        // Input Jacobian points: P = (X1, Y1, Z1), Q = (X2, Y2, Z2)
        let p = (parse(0), parse(1), parse(2));
        let q = (parse(3), parse(4), parse(5));

        // Expected output: affine point or None (for point at infinity)
        let expected = match record.get(6).unwrap().is_empty() {
            true => None,
            false => Some((parse(6), parse(7))),
        };

        let result = jacobian_add(&p, &q);

        assert_eq!(
            result, expected,
            "Mismatch:\nP = {:?}\nQ = {:?}\nExpected = {:?}\nGot = {:?}",
            p, q, expected, result
        );
    }
}
