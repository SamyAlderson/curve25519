// This module contains utility functions for the Curve25519 implementation.
pub mod utils {
    use std::convert::TryFrom;
    use std::fmt;

    // A wrapper around an i32 to represent a Montgomery ladder coefficient.
    pub struct MontgomeryCoefficient {
        value: i32,
    }

    impl MontgomeryCoefficient {
        // Create a new Montgomery coefficient from the given value.
        pub fn new(value: i32) -> Self {
            Self { value }
        }

        // Convert this coefficient to a u64.
        pub fn to_u64(self) -> u64 {
            self.value as u64
        }
    }

    impl fmt::Display for MontgomeryCoefficient {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    // A wrapper around an i32 to represent a Curve25519 field element.
    pub struct FieldElement {
        value: i32,
    }

    impl FieldElement {
        // Create a new field element from the given value.
        pub fn new(value: i32) -> Self {
            Self { value }
        }

        // Convert this field element to a Montgomery coefficient.
        pub fn to_coefficient(self) -> MontgomeryCoefficient {
            MontgomeryCoefficient::new(self.value)
        }

        // Check if this field element is zero.
        pub fn is_zero(&self) -> bool {
            self.value == 0
        }
    }

    impl fmt::Display for FieldElement {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }

    // A wrapper around an i32 to represent a Curve25519 scalar.
    pub struct Scalar {
        value: u32,
    }

    impl Scalar {
        // Create a new scalar from the given value.
        pub fn new(value: u32) -> Self {
            Self { value }
        }

        // Convert this scalar to a Montgomery coefficient.
        pub fn to_coefficient(self) -> MontgomeryCoefficient {
            MontgomeryCoefficient::new((self.value as i32).try_into().unwrap())
        }
    }

    impl fmt::Display for Scalar {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.value)
        }
    }
}