use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ErrorResponseWire {
    pub status: &'static str,
    pub error: String,
}
