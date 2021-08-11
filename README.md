Finite Fields In `no_std` Rust
==============================

[![Build Status](https://travis-ci.org/rspencer01/tinyfield.svg?branch=master)](https://travis-ci.org/rspencer01/tinyfield)
[![Latest Version](https://img.shields.io/crates/v/tinyfield.svg)](https://crates.io/crates/tinyfield)
[![Coverage Status](https://coveralls.io/repos/github/rspencer01/tinyfield/badge.svg?branch=master)](https://coveralls.io/github/rspencer01/tinyfield?branch=master)

This crate exposes a number of small finite field types.  It does not depend on
the standard library.

At time of writing, the top few results for "rust finite fields" in a google
search shows:

 * A crate that no longer compiles, and doesn't implement finite fields
   correctly at all 
 * A crate that only implements fields of characteristic two
 * A crate that does general finite fields, but doesn't expose arithmetic past
   addition for higher degree finite fields.

This crate attempts to supply:

 * A small library with low footprint when linked
 * Support for as many fields as I have energy to write irreducible polynomials for
 * Small finite field elements (up to 32 bits only)
 * Small characteristic fields only (fitting in one `u8`)

This crate does not attempt to:

 * Be particularly fast
 * Handle large primes suitable for, say, cryptography

This crate should, in the future:

 * Handle all finite fields that fit within a `u32`
 * Never rely on `std`
 * Be a pleasure to use and have intuitive interfaces
 * Have easy to understand code without unnecessary optimisation

Pull requests to make that happen would be most welcome.

Issues on the github tracker are also welcome.

Example usage
-------------
```rust
# use tinyfield::prime_power_field::*;
type F = tinyfield::fields::GF9;

let delta = F::elts()
              .filter(|x| *x * *x - PrimePowerFieldElt::from(2) == F::zero)
              .next()
              .expect("GF9 should contain a square root of two");
```
