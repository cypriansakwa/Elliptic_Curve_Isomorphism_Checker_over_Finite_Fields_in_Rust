# Elliptic Curve Isomorphism Checker over Finite Fields in Rust

This Rust program checks if two elliptic curves defined over a finite field are isomorphic. It performs this check by iterating over possible values of `u` in the field and finding a match based on curve parameters.

## Overview

In mathematics, two elliptic curves are considered **isomorphic** over a finite field if there exists a non-zero element `u` in the field such that:
- $a' = u^4 \cdot a$ (mod `p`)
- $b' = u^6 \cdot b$ (mod `p`)

Given two curves:
- Curve 1: $y^2 = x^3 + a \cdot x + b$
- Curve 2:  $y^2 = x^3 + a' \cdot x + b'$

The program checks if they satisfy the above conditions.

## Features

- **Modular Arithmetic**: Uses modular exponentiation to compute values of $u^4$ and $u^6$ modulo `p`.
- **Elliptic Curve Structure**: Checks isomorphism for elliptic curves of the form $y^2 = x^3 + ax + b$ over a finite field $\mathbb{F}_p$.

## Dependencies

This project requires the following crates:
- `num-bigint`: For handling large integers.
- `num-traits`: For numerical traits like `One`, `Zero`, and `ToPrimitive`.

To install these dependencies, include the following in your `Cargo.toml` file:

<```
[dependencies]
num-bigint = "0.4"
num-traits = "0.2"

## Usage
