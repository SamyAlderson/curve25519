/// Utility functions for the Curve25519 elliptic curve implementation.
/// 
/// This module provides reusable functions for tasks such as prime number generation,
/// Montgomery ladder scalar multiplication, and Montgomery ladder point addition.
///
/// # Features
///
/// * `key_generation`: Enables key generation functionality.
/// * `signature_generation`: Enables signature generation functionality.
/// * `scalar_multiplication`: Enables scalar multiplication functionality.
///
/// # Dependencies
///
/// * `elliptic-curves`: A library for working with elliptic curves.
#[cfg(feature = "scalar_multiplication")]
pub mod utils {
    use crate::{constants::GENERATOR, elliptic_curve::Modulo};
    use rand::RngCore;

    /// Generates a random prime number `p` such that `p = 2^255 + 19`.
    /// 
    /// This function uses the `rand` crate to generate a random number and then performs
    /// the necessary calculations to ensure the result is prime.
    pub fn generate_prime() -> u64 {
        let mut rng = rand::thread_rng();
        let mut n = [0u8; 32];
        rng.fill_bytes(&mut n);
        let mut p = u64::from_le_bytes(n);
        p |= 1 << 255;
        if is_prime(p) {
            p
        } else {
            generate_prime()
        }
    }

    /// Checks if a number `n` is prime.
    /// 
    /// This function uses a simple trial division method to check if `n` is prime.
    fn is_prime(n: u64) -> bool {
        for i in 2.. {
            if i * i > n {
                break;
            }
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    /// Performs Montgomery ladder scalar multiplication on a point `P` with scalar `s`.
    /// 
    /// This function uses the Montgomery ladder algorithm to perform scalar multiplication
    /// on a point `P` with scalar `s`. This is the most efficient method for performing
    /// scalar multiplication on an elliptic curve.
    pub fn montgomery_ladder_scalar_multiplication<P>(P: &P, s: &Modulo<u64>) -> P {
        let mut Q = *P;
        let mut r = 0;
        for i in 0..255 {
            if s[r] == 1 {
                Q = P.add(Q);
            }
            r = (r << 1) | (s[r] as u64);
            if r == 255 {
                Q = P.add(Q);
                r = 0;
            }
        }
        Q
    }

    /// Performs Montgomery ladder point addition on two points `P` and `Q`.
    /// 
    /// This function uses the Montgomery ladder algorithm to perform point addition
    /// on two points `P` and `Q`. This is the most efficient method for performing
    /// point addition on an elliptic curve.
    pub fn montgomery_ladder_point_addition<P, Q>(P: &P, Q: &Q) -> P {
        let mut R = P.clone();
        let mut t = Q;
        let mut x0 = P.x;
        let mut y0 = P.y;
        let mut x1 = t.x;
        let mut y1 = t.y;
        let mut s = 0;
        for _ in 0..255 {
            if x1 == x0 && y1 == y0 {
                break;
            }
            s |= 1 << (255 - (x1 ^ x0).ilog2());
            let mut x2 = (x1 - x0 + Modulo::get_order()) % Modulo::get_order();
            let mut y2 = (y1 - y0 + Modulo::get_order()) % Modulo::get_order();
            if y2 < 0 {
                y2 += Modulo::get_order();
            }
            t = P.add(t);
            x0 = x1;
            y0 = y1;
            x1 = x2;
            y1 = y2;
        }
        let mut r = 0;
        for i in 0..255 {
            r = (r << 1) | (s[r] as u64);
            if r == 255 {
                break;
            }
        }
        R.add(t).clone()
    }
}