use csv::ReaderBuilder;
use num_bigint::BigUint;
use rust::affine::{modp, affine_add};

#[test]
fn csv_test_vectors() {
    let file_path = format!(
        "{}/../test_vectors/affine_test_vectors.csv",
        env!("CARGO_MANIFEST_DIR")
    );

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(&file_path)
        .expect("Failed to open affine_test_vectors.csv");

    for result in reader.records() {
        let record = result.expect("Failed to parse CSV record");

        let parse = |index: usize| {
            record
                .get(index)
                .filter(|s| !s.is_empty())
                .map(|s| BigUint::parse_bytes(s.as_bytes(), 10).unwrap() % modp())
        };

        let p = match (parse(0), parse(1)) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        };

        let q = match (parse(2), parse(3)) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        };

        let expected = match (parse(4), parse(5)) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        };

        assert_eq!(
            affine_add(&p, &q),
            expected,
            "Failed test for input: P = {:?}, Q = {:?}",
            p,
            q
        );
    }
}
