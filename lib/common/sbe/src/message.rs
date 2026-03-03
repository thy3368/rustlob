use crate::SbeError;

pub trait SbeMessage: Sized {
    fn encode_into(&self, buffer: &mut [u8]) -> Result<usize, SbeError>;
    fn decode_from(buffer: &[u8]) -> Result<Self, SbeError>;
    fn max_encoded_length() -> usize;

    fn encode_to_bytes(&self) -> Result<Vec<u8>, SbeError> {
        let mut buf = vec![0u8; Self::max_encoded_length()];
        let len = self.encode_into(&mut buf)?;
        buf.truncate(len);
        Ok(buf)
    }
}
