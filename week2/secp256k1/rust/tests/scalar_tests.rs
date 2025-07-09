use csv::ReaderBuilder;
use num_bigint::BigUint;
use rust::affine::modp;
use rust::scalar_mul::scalar_mul::{affine_scalar_mul, jacobian_scalar_mul};

#[test]
fn test_affine_scalar() {
    let path = format!(
        "{}/../test_vectors/scalar_test_vectors.csv",
        env!("CARGO_MANIFEST_DIR")
    );

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(&path)
        .expect("Failed to open scalar_test_vectors.csv");

    for record in reader.records().map(|r| r.expect("Failed to parse record")) {
        let parse = |i: usize| -> BigUint {
            BigUint::parse_bytes(record.get(i).unwrap().as_bytes(), 10).unwrap() % modp()
        };

        let k = parse(0);
        let point = Some((parse(1), parse(2)));
        let expected = (parse(3), parse(4));

        let result = affine_scalar_mul(&k, &point).expect("Affine scalar multiplication failed");
        assert_eq!(
            result, expected,
            "Affine scalar mul failed:\nk = {}\nP = {:?}",
            k, point
        );
    }
}

#[test]
fn test_jacobian_scalar() {
    let path = format!(
        "{}/../test_vectors/scalar_test_vectors.csv",
        env!("CARGO_MANIFEST_DIR")
    );

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(&path)
        .expect("Failed to open scalar_test_vectors.csv");

    for record in reader.records().map(|r| r.expect("Failed to parse record")) {
        let parse = |i: usize| -> BigUint {
            BigUint::parse_bytes(record.get(i).unwrap().as_bytes(), 10).unwrap() % modp()
        };

        let k = parse(0);
        let point = (parse(1), parse(2));
        let expected = (parse(3), parse(4));

        let result =
            jacobian_scalar_mul(&k, &point).expect("Jacobian scalar multiplication failed");
        assert_eq!(
            result, expected,
            "Jacobian scalar mul failed:\nk = {}\nP = {:?}",
            k, point
        );
    }
}
