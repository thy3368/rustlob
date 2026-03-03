#[derive(Debug, Clone, PartialEq)]
pub enum SbeError {
    BufferTooSmall { required: usize, available: usize },
    InsufficientData { required: usize, available: usize },
}

impl std::fmt::Display for SbeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BufferTooSmall { required, available } => {
                write!(f, "Buffer too small: need {} bytes, have {}", required, available)
            }
            Self::InsufficientData { required, available } => {
                write!(f, "Insufficient data: need {} bytes, have {}", required, available)
            }
        }
    }
}

impl std::error::Error for SbeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_too_small_display() {
        let err = SbeError::BufferTooSmall { required: 100, available: 50 };
        assert_eq!(err.to_string(), "Buffer too small: need 100 bytes, have 50");
    }
}
