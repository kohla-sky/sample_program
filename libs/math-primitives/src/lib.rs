use common::{CommonError, CommonResult};

/// Mathematical constants used throughout the system
pub mod constants {
    /// Maximum value for basis points (100.00%)
    pub const MAX_BASIS_POINTS: u16 = 10000;
    
    /// Default precision for calculations
    pub const DEFAULT_PRECISION: u32 = 8;
    
    /// Maximum safe multiplier to prevent overflow
    pub const MAX_SAFE_MULTIPLIER: u64 = u64::MAX / 1000000;
    
    /// Common decimal precisions
    pub const PRECISION_6: u32 = 6;
    pub const PRECISION_8: u32 = 8;
    pub const PRECISION_9: u32 = 9;
    pub const PRECISION_18: u32 = 18;
    
    /// Mathematical limits for safe calculations
    pub const SQRT_MAX_U64: u64 = 4294967295; // sqrt(u64::MAX)
}

/// Primitive mathematical operations with overflow protection
pub mod primitives {
    use super::*;
    use crate::constants::*;

    /// Calculate power of 10 with overflow protection
    pub fn pow10(exponent: u32) -> CommonResult<u64> {
        if exponent > 19 {
            return Err(CommonError::InvalidCalculation);
        }
        
        let result = match exponent {
            0 => 1,
            1 => 10,
            2 => 100,
            3 => 1_000,
            4 => 10_000,
            5 => 100_000,
            6 => 1_000_000,
            7 => 10_000_000,
            8 => 100_000_000,
            9 => 1_000_000_000,
            10 => 10_000_000_000,
            11 => 100_000_000_000,
            12 => 1_000_000_000_000,
            13 => 10_000_000_000_000,
            14 => 100_000_000_000_000,
            15 => 1_000_000_000_000_000,
            16 => 10_000_000_000_000_000,
            17 => 100_000_000_000_000_000,
            18 => 1_000_000_000_000_000_000,
            19 => 10_000_000_000_000_000_000,
            _ => return Err(CommonError::InvalidCalculation),
        };
        
        Ok(result)
    }

    /// Calculate square root using Newton's method (integer approximation)
    pub fn isqrt(n: u64) -> CommonResult<u64> {
        if n > SQRT_MAX_U64.saturating_pow(2) {
            return Err(CommonError::InvalidCalculation);
        }
        
        if n == 0 {
            return Ok(0);
        }
        
        let mut x = n;
        let mut y = (n + 1) / 2;
        
        while y < x {
            x = y;
            y = (x + n / x) / 2;
        }
        
        Ok(x)
    }

    /// Check if a number is within safe multiplication bounds
    pub fn is_safe_for_multiplication(a: u64, b: u64) -> bool {
        if a == 0 || b == 0 {
            return true;
        }
        
        a <= u64::MAX / b
    }

    /// Validate that a basis points value is within valid range
    pub fn validate_basis_points(bp: u16) -> CommonResult<()> {
        if bp > MAX_BASIS_POINTS {
            return Err(CommonError::Custom(
                format!("Basis points {} exceeds maximum {}", bp, MAX_BASIS_POINTS)
            ));
        }
        Ok(())
    }

    /// Get the precision multiplier for a given decimal places
    pub fn get_precision_multiplier(decimals: u8) -> CommonResult<u64> {
        if decimals > 19 {
            return Err(CommonError::InvalidCalculation);
        }
        pow10(decimals as u32)
    }
}

/// Number theory utilities
pub mod number_theory {
    use super::*;

    /// Calculate greatest common divisor using Euclidean algorithm
    pub fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    /// Calculate least common multiple
    pub fn lcm(a: u64, b: u64) -> CommonResult<u64> {
        if a == 0 || b == 0 {
            return Ok(0);
        }
        
        let gcd_val = gcd(a, b);
        let result = a.checked_mul(b / gcd_val)
            .ok_or(CommonError::InvalidCalculation)?;
        
        Ok(result)
    }

    /// Check if a number is a perfect square
    pub fn is_perfect_square(n: u64) -> CommonResult<bool> {
        let sqrt_n = primitives::isqrt(n)?;
        Ok(sqrt_n * sqrt_n == n)
    }

    /// Calculate modular exponentiation: (base^exp) % modulus
    pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> CommonResult<u64> {
        if modulus == 0 {
            return Err(CommonError::InvalidCalculation);
        }
        
        if modulus == 1 {
            return Ok(0);
        }
        
        let mut result = 1;
        base %= modulus;
        
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result as u128 * base as u128 % modulus as u128) as u64;
            }
            exp >>= 1;
            base = (base as u128 * base as u128 % modulus as u128) as u64;
        }
        
        Ok(result)
    }
}

/// Validation utilities for mathematical operations
pub mod validation {
    use super::*;
    use crate::constants::*;

    /// Validate that a value doesn't exceed safe multiplication bounds
    pub fn validate_multiplication_safety(a: u64, b: u64) -> CommonResult<()> {
        if !primitives::is_safe_for_multiplication(a, b) {
            return Err(CommonError::Custom(
                "Multiplication would cause overflow".to_string()
            ));
        }
        Ok(())
    }

    /// Validate decimal precision is within supported range
    pub fn validate_precision(decimals: u8) -> CommonResult<()> {
        if decimals > 19 {
            return Err(CommonError::Custom(
                format!("Decimal precision {} exceeds maximum 19", decimals)
            ));
        }
        Ok(())
    }

    /// Validate that a value is within percentage bounds (0-100%)
    pub fn validate_percentage(percentage: f64) -> CommonResult<()> {
        if !(0.0..=100.0).contains(&percentage) {
            return Err(CommonError::Custom(
                format!("Percentage {} must be between 0 and 100", percentage)
            ));
        }
        Ok(())
    }
} 