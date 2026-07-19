// File: tests/test_lib.rs
// Purpose: Unit tests for the library
// Project: curve25519
// Description: Rust implementation of the Curve25519 elliptic curve

#[cfg(test)]
mod tests {
    use crate::utils::generate_random_scalar;
    use crate::scalar_multiplication;
    use crate::signature_generation;
    use crate::key_generation;
    use elliptic_curves::curve25519::*;

    #[test]
    fn test_key_generation() {
        let (public_key, private_key) = key_generation();
        assert!(public_key.is_some());
        assert!(private_key.is_some());
    }

    #[test]
    fn test_scalar_multiplication() {
        let scalar = generate_random_scalar();
        let point = scalar_multiplication(scalar);
        assert!(point.is_some());
    }

    #[test]
    fn test_signature_generation() {
        let (public_key, private_key) = key_generation();
        let signature = signature_generation(private_key, "test message".as_bytes());
        assert!(signature.is_some());
    }

    #[test]
    fn test_signature_verification() {
        let (public_key, private_key) = key_generation();
        let signature = signature_generation(private_key, "test message".as_bytes());
        assert!(signature_generation_verification(public_key, signature.unwrap(), "test message".as_bytes()).unwrap());
    }

    #[test]
    fn test_scalar_multiplication_inverse() {
        let scalar = generate_random_scalar();
        let point = scalar_multiplication(scalar);
        let inverse_scalar = scalar_multiplication_inverse(point.unwrap());
        assert_eq!(scalar * inverse_scalar, EdwardsPoint::identity());
    }
}