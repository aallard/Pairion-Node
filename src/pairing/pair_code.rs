//! Pair code generation and LED encoding.
//!
//! Pair codes are 4-slot color sequences with 8 color choices per slot,
//! yielding 4096 combinations. Codes expire after 10 minutes.
//!
//! In M0, this is scaffolding only. Real pair code generation arrives at M6.

/// A pairing code represented as 4 color slots.
#[derive(Debug, Clone, PartialEq)]
pub struct PairCode {
    /// Four color indices (0–7 each).
    pub slots: [u8; 4],
}

impl PairCode {
    /// Generate a new random pair code.
    ///
    /// In M0 this is a stub — returns a fixed code. Real generation at M6.
    pub fn generate() -> Self {
        Self {
            slots: [0, 1, 2, 3],
        }
    }

    /// Encode the pair code as a human-readable string.
    pub fn to_string_code(&self) -> String {
        self.slots
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_returns_valid_code() {
        let code = PairCode::generate();
        assert_eq!(code.slots.len(), 4);
        for slot in &code.slots {
            assert!(*slot < 8);
        }
    }

    #[test]
    fn to_string_code_format() {
        let code = PairCode {
            slots: [0, 1, 2, 3],
        };
        assert_eq!(code.to_string_code(), "0-1-2-3");
    }
}
