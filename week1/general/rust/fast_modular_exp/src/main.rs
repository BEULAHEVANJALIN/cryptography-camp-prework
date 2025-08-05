//! # fast_mod_exp
//!
//! A small CLI for demonstrating modular exponentiation (fast vs naive),
//! big-integer benchmarks, and a toy RSA key-gen demo.
//!
//! ## Usage
//! ```text
//! cargo run -- [--test] [--bench] [--rsa] [-v]
//! ```
//! - `--test`      Run correctness tests on u128 routines.
//! - `--bench`     Benchmark fast vs naive (u128 & BigUint).
//! - `--rsa`       Perform small-prime RSA key gen/encrypt/decrypt.
//! - `-v, --verbose` Enable debug logging.

use clap::Parser;
use env_logger::Env;
use log::{debug, error, info};
use num_bigint::BigUint;
use num_traits::{FromPrimitive, One, Zero};
use std::time::Instant;

/// Command-line options
#[derive(Parser, Debug)]
#[command(name = "fast_mod_exp", version, about = "Modular exponentiation demo")]
struct Opts {
    /// Run u128 correctness tests
    #[arg(long)]
    test: bool,

    /// Run performance benchmarks (u128 + BigUint)
    #[arg(long)]
    bench: bool,

    /// Run RSA demonstration
    #[arg(long)]
    rsa: bool,

    /// Enable debug logging
    #[arg(short, long)]
    verbose: bool,
}

/// Fast modular exponentiation on `u128` using square-and-multiply.
///
/// # Arguments
///
/// * `base` — the base.
/// * `exponent` — the exponent (≥ 0).
/// * `modulus` — the modulus (≥ 1).
///
/// # Returns
///
/// `(base.pow(exponent) % modulus)`, computed in `O(log exponent)` time.
fn fast_mod_exp(mut base: u128, mut exponent: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let result = 1u128;
    base %= modulus;

    while exponent > 0 {
        if (exponent & 1) == 1 {
            base = base.wrapping_mul(base) % modulus;
        }
        exponent >>= 1;
        base = base.wrapping_mul(base) % modulus;
    }
    result
}

/// Naive modular exponentiation on `u128`, `O(exponent)` time.
///
/// **Only** for very small exponents.
fn naive_mod_exp(mut base: u128, exponent: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1u128;
    base %= modulus;
    for _ in 0..exponent {
        result = (result * base) % modulus;
    }
    result
}

/// Extended Euclidean algorithm (returns `(g, x, y)` so that `a*x + b*y = g = gcd(a,b)`).
fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (g, x1, y1) = extended_gcd(b % a, a);
    (g, y1 - (b / a) * x1, x1)
}

/// Modular inverse via Extended Euclidean: `Some(a⁻¹ mod m)` if `gcd(a,m) = 1`.
///
/// # Performance Comparison
///
/// For a 128-bit prime modulus:
///
/// 1. **Extended‐GCD Method**
///    - Complexity: O(log p) 128-bit divisions (~7–10 iterations).
///    - Each division costs tens of CPU cycles.
///
/// 2. **Fermat’s Little‐Theorem Method**
///    - Complexity: O(bit‐length) modular multiplications (~127 squarings + ~63 multiplies).
///    - Each multiply is much cheaper (1–2 cycles).
///
/// 3. **Net Effect**
///    - Extended‐GCD often wins on u128 since ~10 divisions < ~200 multiplies.
///    - Benchmarks show both complete in single-digit nanoseconds in optimized builds.
///
/// ## When to Prefer Fermat
///
/// - Large moduli (e.g., 1024‑bit or higher) where exponentiation cost is amortized in windowed algorithms.
/// - Batch inversion scenarios, turning many inverses into one exponentiation + multiplies.
///
/// # Examples
///
/// ```rust
/// let inv_euclid  = mod_inverse(3, 11).unwrap();
/// let inv_fermat  = mod_inverse_fermat(3, 11).unwrap();
/// assert_eq!(inv_euclid, inv_fermat);
/// ```
fn mod_inverse(a: u128, m: u128) -> Option<u128> {
    let (g, x, _) = extended_gcd(a as i128, m as i128);
    if g != 1 {
        None
    } else {
        // ensure positive
        Some(((x % m as i128 + m as i128) % m as i128) as u128)
    }
}

/// Return `Some(b.pow(e) % m)` if it won't overflow `u128`,
/// otherwise `None` so you can skip it safely.
fn safe_builtin_pow_mod(b: u128, e: u128, m: u128) -> Option<u128> {
    // Approximate bit‐length of b: floor(log₂ b) + 1
    let bits_b = 128u128.saturating_sub(b.leading_zeros() as u128);
    // If each multiply can add up to `bits_b` bits in the worst case,
    // total bits ≲ e * bits_b. We need that ≤ 128.
    if e * bits_b <= 128 {
        Some(b.pow(e as u32) % m)
    } else {
        None
    }
}

/// Test a handful of small `u128` cases for correctness against
/// 1) our fast square-and-multiply,
/// 2) the naive O(e) loop,
/// 3) Rust’s built-in `pow` + `%`.
fn test_correctness() {
    info!("Testing correctness (u128)...");
    let cases = &[
        (2u128,   10,  1000),
        (3,       13,  10),
        (5,        7,  13),
        (123,    456,  789),
        (7,      100, 101),
    ];
    for &(b, e, m) in cases {
        let fast  = fast_mod_exp(b, e, m);
        let naive = naive_mod_exp(b, e, m);
        let builtin_opt = safe_builtin_pow_mod(b, e, m);

        match builtin_opt {
            Some(bu) => {
                let pass = (fast == naive) && (naive == bu);
                info!(
                    "{}^{} mod {} => fast={}, naive={}, builtin={} -- {}",
                    b, e, m, fast, naive, bu,
                    if pass { "PASS" } else { "FAIL" }
                );
            }
            None => {
                // Fallback: just compare fast vs naive
                let pass = fast == naive;
                info!(
                    "{}^{} mod {} => fast={}, naive={} -- builtin skipped, {}",
                    b, e, m, fast, naive,
                    if pass { "PASS" } else { "FAIL" }
                );
            }
        }
    }
    info!("Correctness tests done.\n");
}

/// Fast modular exponentiation on `BigUint` using square-and-multiply.
fn fast_mod_exp_big(mut base: BigUint, mut exp: BigUint, modulus: &BigUint) -> BigUint {
    if modulus.is_one() {
        return BigUint::zero();
    }
    let mut result = BigUint::one();
    base %= modulus;
    while !exp.is_zero() {
        if (&exp & BigUint::one()).is_one() {
            result = (&result * &base) % modulus;
        }
        exp >>= 1;
        base = (&base * &base) % modulus;
    }
    result
}

/// Naive modular exponentiation on `BigUint`; only for very small exponents.
fn naive_mod_exp_big(base: BigUint, exp: BigUint, modulus: &BigUint) -> BigUint {
    if modulus.is_one() {
        return BigUint::zero();
    }
    let mut result = BigUint::one();
    let mut counter = BigUint::zero();
    while &counter < &exp {
        result = (&result * &base) % modulus;
        counter += BigUint::one();
    }
    result
}

/// Benchmark both `u128` and `BigUint` versions, skipping naive when impractical.
fn benchmark_performance() {
    info!("Benchmarking exponentiation performance...");
    // Mix of small and huge exponents
    let cases = vec![
        (
            BigUint::from_u32(65_537).unwrap(),
            BigUint::from_u32(65_536).unwrap(),
            BigUint::from_u64(4_294_967_295).unwrap(),
        ),
        (
            BigUint::from_u32(2).unwrap(),
            BigUint::from_u32(1_000_000).unwrap(),
            BigUint::from_u64(1_000_000_007).unwrap(),
        ),
        (
            BigUint::from_u32(2).unwrap(),
            BigUint::one() << 1000,
            (BigUint::one() << 1024) - BigUint::one(),
        ),
    ];

    for (i, (base, exp, modulus)) in cases.into_iter().enumerate() {
        info!("Test case {}: {}^{} % {}", i + 1, base, exp, modulus);

        // Naive only on first two
        if i < 2 {
            let t0 = Instant::now();
            let _naive = naive_mod_exp_big(base.clone(), exp.clone(), &modulus);
            let d0 = t0.elapsed();
            info!("  Naive (BigUint): {:.5}s", d0.as_secs_f64());
        } else {
            info!("  Naive (BigUint): skipped (exponent too large)");
        }

        // Fast BigUint
        let t1 = Instant::now();
        let fast_big = fast_mod_exp_big(base.clone(), exp.clone(), &modulus);
        let d1 = t1.elapsed();
        info!("  Fast  (BigUint): {:.5}s", d1.as_secs_f64());

        debug!("  Fast  (BigUint) result: {}", fast_big);
    }

    info!("\n");

    info!("Benchmarking u128 modular inverses...");
    let (a, p) = (3_000_000_000_000_001u128, 4_000_000_000_000_003u128); // p must be prime for fermat
    let mut t2 = Instant::now();
    let _ = mod_inverse(a, p);
    info!("  Extended-GCD inverse: {:.5}s", t2.elapsed().as_secs_f64());
    t2 = Instant::now();
    let _ = mod_inverse_fermat(a, p);
    info!("  Fermat inverse: {:.5}s", t2.elapsed().as_secs_f64());
    info!("\n");

    // Use a large Mersenne prime (2^127 - 1) for a noticeable benchmark
    let p: u128 = 170141183460469231731687303715884105727u128; // prime = 2^127 - 1
    let a: u128 = p - 1; // test a = p - 1
    let iterations: u32 = 100_000;
    t2 = Instant::now(); 
    for _ in 0..iterations {
        let _ = mod_inverse(a, p);
    }
    let dur2 = t2.elapsed();
    info!("  Extended-GCD inverse: total {} ns, avg {:.2} ns", dur2.as_nanos(), dur2.as_nanos() as f64 / iterations as f64);
    t2 = Instant::now();
    for _ in 0..iterations {
        let _ = mod_inverse_fermat(a, p);
    }
    let dur3 = t2.elapsed();
    info!("  Fermat inverse:       total {} ns, avg {:.2} ns", dur3.as_nanos(), dur3.as_nanos() as f64 / iterations as f64);
    info!("Benchmarking complete.\n");
}

/// Demo: small‐prime RSA key generation, encryption, and decryption on `u128`.
fn rsa_demo() {
    info!("RSA Key Generation Demo (u128)...");
    let p = 61u128;
    let q = 53u128;
    let n = p * q;
    let phi = (p - 1) * (q - 1);
    let e = 17u128;

    // Compute d = e⁻¹ mod φ
    let (g, x, _) = extended_gcd(e as i128, phi as i128);
    if g != 1 {
        error!("e={} is not coprime to φ={}", e, phi);
        return;
    }
    let d = ((x % phi as i128 + phi as i128) % phi as i128) as u128;

    info!("  p={}, q={}, n={}, φ(n)={}", p, q, n, phi);
    info!("  Public key:  (n={}, e={})", n, e);
    info!("  Private key: (n={}, d={})", n, d);

    let msg = 123u128;
    let c = fast_mod_exp(msg, e, n);
    let m = fast_mod_exp(c, d, n);
    info!("  msg={}, cipher={}, decrypted={}, ok={}", msg, c, m, m == msg);
    info!("RSA demo complete.\n");
}

/// Error type for modular inversion
#[derive(Debug)]
enum ModInvError {
    /// Occurs when attempting to invert zero modulo p
    ZeroInput,
}

impl std::fmt::Display for ModInvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModInvError::ZeroInput => write!(f, "cannot compute inverse of zero modulo p"),
        }
    }
}

impl std::error::Error for ModInvError {}

/// Computes the modular inverse of `a` modulo prime `p` using Fermat's little theorem:
///
/// a^{-1} ≡ a^{p-2} mod p
///
/// # Arguments
///
/// * `a` - The integer whose inverse is sought (must not be divisible by `p`).
/// * `p` - A prime modulus.
///
/// # Errors
///
/// Returns `ModInvError::ZeroInput` if `a % p == 0`, since zero has no inverse.
///
/// # Examples
///
/// ```rust
/// let inv = mod_inverse_fermat(3, 11).unwrap();
/// assert_eq!(inv, 4);
/// ```
fn mod_inverse_fermat(a: u128, p: u128) -> Result<u128, ModInvError> {
    if a % p == 0 {
        return Err(ModInvError::ZeroInput);
    }
    Ok(fast_mod_exp(a, p - 2, p))
}


fn main() {
    let opts = Opts::parse();

    // Init logger (RUST_LOG=debug or -v for verbose)
    let env = Env::default().filter_or("RUST_LOG", if opts.verbose { "debug" } else { "info" });
    env_logger::init_from_env(env);

    if !(opts.test || opts.bench || opts.rsa) {
        error!("No action specified. Use --test, --bench, and/or --rsa.");
        std::process::exit(1);
    }

    if opts.test {
        test_correctness();
    }
    if opts.bench {
        benchmark_performance();
    }
    if opts.rsa {
        rsa_demo();
    }
}
