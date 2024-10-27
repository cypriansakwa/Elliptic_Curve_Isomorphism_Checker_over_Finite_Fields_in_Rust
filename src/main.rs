use num_bigint::BigUint;
use num_traits::{One, Zero, ToPrimitive}; // Added ToPrimitive here

/// Define an elliptic curve over a finite field
struct EllipticCurve {
    a: BigUint,
    b: BigUint,
    p: BigUint, // the prime modulus for the finite field
}

/// Modular exponentiation function
fn mod_exp(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    if modulus.is_one() {
        return BigUint::zero();
    }
    let mut result = BigUint::one();
    let mut base = base % modulus;
    let mut exp = exponent.clone();

    while !exp.is_zero() {
        if &exp & BigUint::one() == BigUint::one() {
            result = (result * &base) % modulus;
        }
        exp >>= 1;
        base = (&base * &base) % modulus;
    }
    result
}

/// Function to check if two elliptic curves are isomorphic and, if so, return the value of `u`
fn isomorphic(curve1: &EllipticCurve, curve2: &EllipticCurve) -> Option<BigUint> {
    // Iterate over possible values of u in the finite field (1 to p-1)
    for u in 1..curve1.p.to_u32().unwrap_or_default() {
        let u = BigUint::from(u);
        let u4 = mod_exp(&u, &BigUint::from(4u32), &curve1.p);
        let u6 = mod_exp(&u, &BigUint::from(6u32), &curve1.p);

        // Check if a' = u^4 * a and b' = u^6 * b (mod p)
        if (curve2.a.clone() == (&u4 * &curve1.a) % &curve1.p) &&
           (curve2.b.clone() == (&u6 * &curve1.b) % &curve1.p) {
            return Some(u); // Found an isomorphism, return the value of u
        }
    }
    None // No isomorphism found
}

fn main() {
    // Example curves over a finite field with p = 5
    let p = BigUint::from(5u32);

    // Curve 1: y^2 = x^3 + x + 1
    let curve1 = EllipticCurve {
        a: BigUint::from(1u32),
        b: BigUint::from(1u32),
        p: p.clone(),
    };

    // Curve 2: y^2 = x^3 + x + 4
    let curve2 = EllipticCurve {
        a: BigUint::from(1u32),
        b: BigUint::from(4u32),
        p: p.clone(),
    };

    // Check if the curves are isomorphic and print the result
    match isomorphic(&curve1, &curve2) {
        Some(u) => println!("The curves are isomorphic with u = {}", u),
        None => println!("The curves are not isomorphic."),
    }
}
