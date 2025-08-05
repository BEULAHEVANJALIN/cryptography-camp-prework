#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
fast_modular_exp.py

Provides:
 - fast_mod_exp: O(log e) modular exponentiation
 - naive_mod_exp: O(e) modular exponentiation (for small exponents only)
 - test_correctness: verify against Python's built-in pow()
 - benchmark_performance: compare runtimes
 - rsa_key_generation: small RSA key demo

Usage:
    python fast_modular_exp.py [--test] [--benchmark] [--rsa]
"""

import time
import argparse
import logging
import sys


def fast_mod_exp(base: int, exponent: int, modulus: int) -> int:
    """
    Fast modular exponentiation using square-and-multiply.
    
    Args:
        base: The base integer.
        exponent: The exponent (must be ≥ 0).
        modulus: The modulus (must be ≥ 1).
    
    Returns:
        (base ** exponent) % modulus.
    """
    if modulus == 1:
        return 0

    result = 1
    base %= modulus

    while exponent > 0:
        if (exponent & 1) == 1:
            result = (result * base) % modulus
        exponent >>= 1
        base = (base * base) % modulus

    return result


def naive_mod_exp(base: int, exponent: int, modulus: int) -> int:
    """
    Naive modular exponentiation in O(exponent) time.
    Only suitable for very small exponents.
    
    Args:
        base: The base integer.
        exponent: The exponent (must be ≥ 0 and small).
        modulus: The modulus (must be ≥ 1).
    
    Returns:
        (base ** exponent) % modulus.
    """
    if modulus == 1:
        return 0

    result = 1
    base %= modulus

    for _ in range(exponent):
        result = (result * base) % modulus

    return result


import logging

logger = logging.getLogger(__name__)

def test_correctness() -> None:
    """
    Verify that fast_mod_exp and naive_mod_exp both match Python’s built-in
    pow(base, exponent, modulus) on a set of representative test cases.
    
    Logs one line per case in the form:
      2^10 mod 1000 => fast=24, naive=24, builtin=24 -- PASS
    """
    logger.info("Running correctness tests...")
    
    test_cases = [
        (2,   10,  1000),   # 2^10 mod 1000 == 24
        (3,   13,  10),     # 3^13 mod 10   == 3
        (5,   7,   13),     # 5^7 mod 13    == 8
        (123, 456, 789),    # 123^456 mod 789 == 699
        (7,   100, 101),    # 7^100 mod 101 == 1
    ]
    
    for base, exp, mod in test_cases:
        fast    = fast_mod_exp(base, exp, mod)
        naive   = naive_mod_exp(base, exp, mod)
        builtin = pow(base, exp, mod)
        
        passed = (fast == naive == builtin)
        status = "PASS" if passed else "FAIL"
        
        logger.info(
            "%d^%d mod %d => fast=%d, naive=%d, builtin=%d -- %s",
            base, exp, mod,
            fast, naive, builtin,
            status
        )
    
    logger.info("Correctness tests complete.\n")


def benchmark_performance(threshold: int = 1_000_000) -> None:
    """
    Time naive, fast, and built-in methods on increasingly large exponents
    
    Args:
        threshold: maximum exponent for which to run the naive method.
    """
    logging.info("Running benchmark performance...")
    cases = [
        (65537, 2**16,    2**32 - 1),     # small RSA-like
        (65537, 2**32,    2**64 - 1),     # medium
        (2,     2**1000,  2**1024 - 1),   # huge
    ]

    for idx, (base, exp, mod) in enumerate(cases, start=1):
        logging.info("Test case %d: %d^%d mod %d", idx, base, exp, mod)

        # Naive
        if exp <= threshold:
            t0 = time.time()
            naive = naive_mod_exp(base, exp, mod)
            dt0 = time.time() - t0
            logging.info("  Naive algorithm: %.6f s", dt0)
        else:
            naive = None
            logging.info("  Naive algorithm: skipped (exponent > %d)", threshold)

        # Fast
        t1 = time.time()
        fast = fast_mod_exp(base, exp, mod)
        dt1 = time.time() - t1
        logging.info("  Fast algorithm:  %.6f s", dt1)

        # Built-in
        t2 = time.time()
        builtin = pow(base, exp, mod)
        dt2 = time.time() - t2
        logging.info("  Python pow():    %.6f s", dt2)

        # Compare results
        if naive is not None:
            ok = (naive == fast == builtin)
            logging.info("  Results match:   %s", "YES" if ok else "NO")
        else:
            ok = (fast == builtin)
            logging.info("  fast == pow():   %s", "YES" if ok else "NO")

        logging.info("")  # blank line

    logging.info("Benchmarking complete.\n")


def rsa_key_generation() -> None:
    """
    Demonstrate RSA key generation, encryption, and decryption
    using small primes (for illustration only).
    """
    logging.info("RSA Key Generation Demo")
    # small primes for demonstration
    p, q = 61, 53
    n = p * q
    phi = (p - 1) * (q - 1)
    e = 17

    def extended_gcd(a: int, b: int):
        """Return (g, x, y) such that a*x + b*y = g = gcd(a, b)."""
        if a == 0:
            return b, 0, 1
        g, x1, y1 = extended_gcd(b % a, a)
        return g, y1 - (b // a) * x1, x1

    g, x, _ = extended_gcd(e, phi)
    if g != 1:
        logging.error("Chosen e=%d is not coprime to φ=%d", e, phi)
        sys.exit(1)

    d = x % phi

    logging.info("  p = %d, q = %d", p, q)
    logging.info("  n = p*q = %d", n)
    logging.info("  φ(n) = %d", phi)
    logging.info("  Public key:  (n=%d, e=%d)", n, e)
    logging.info("  Private key: (n=%d, d=%d)", n, d)

    # Encrypt & decrypt a sample message
    message = 123
    ciphertext = fast_mod_exp(message, e, n)
    decrypted = fast_mod_exp(ciphertext, d, n)
    success = (decrypted == message)

    logging.info("  Message       = %d", message)
    logging.info("  Ciphertext    = %d", ciphertext)
    logging.info("  Decrypted     = %d", decrypted)
    logging.info("  Round-trip OK = %s\n", "YES" if success else "NO")


def parse_args():
    parser = argparse.ArgumentParser(
        description="Modular exponentiation utilities and RSA demo."
    )
    parser.add_argument(
        "--test", action="store_true",
        help="Run correctness tests (fast vs. built-in pow)."
    )
    parser.add_argument(
        "--benchmark", action="store_true",
        help="Run performance benchmarks."
    )
    parser.add_argument(
        "--rsa", action="store_true",
        help="Run RSA key generation demo."
    )
    parser.add_argument(
        "-v", "--verbose", action="store_true",
        help="Enable debug-level logging."
    )
    return parser.parse_args()


def main():
    args = parse_args()
    level = logging.DEBUG if args.verbose else logging.INFO
    logging.basicConfig(
        format="%(asctime)s %(levelname)s: %(message)s",
        datefmt="%H:%M:%S",
        level=level
    )

    if not (args.test or args.benchmark or args.rsa):
        logging.error("No action specified. Use --test, --benchmark, and/or --rsa.")
        sys.exit(1)

    if args.test:
        test_correctness()
    if args.benchmark:
        benchmark_performance()
    if args.rsa:
        rsa_key_generation()


if __name__ == "__main__":
    main()