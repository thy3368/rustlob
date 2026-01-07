use bytes::{Bytes, BytesMut, Buf};
use crate::errors::{WsError, WsResult};

/// Zero-copy buffer for handling WebSocket frames and SBE encoded data
///
/// This buffer uses Bytes (reference-counted, zero-copy) internally to avoid
/// unnecessary data copies when passing messages between different layers.
#[derive(Debug, Clone)]
pub struct ZeroCopyBuffer {
    /// The underlying data buffer (reference-counted for zero-copy semantics)
    data: Bytes,
    /// Current read position
    position: usize,
}

impl ZeroCopyBuffer {
    /// Create a new zero-copy buffer from owned data
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data: Bytes::from(data),
            position: 0,
        }
    }

    /// Create a zero-copy buffer from Bytes (no copy)
    pub fn from_bytes(data: Bytes) -> Self {
        Self { data, position: 0 }
    }

    /// Get the current position
    #[inline]
    pub fn position(&self) -> usize {
        self.position
    }

    /// Set the current position
    #[inline]
    pub fn set_position(&mut self, pos: usize) -> WsResult<()> {
        if pos > self.data.len() {
            return Err(WsError::BufferOverflow);
        }
        self.position = pos;
        Ok(())
    }

    /// Get the remaining bytes without copying
    #[inline]
    pub fn remaining(&self) -> &[u8] {
        &self.data[self.position..]
    }

    /// Get all bytes without copying
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Advance the read position
    #[inline]
    pub fn advance(&mut self, cnt: usize) -> WsResult<()> {
        if self.position + cnt > self.data.len() {
            return Err(WsError::BufferOverflow);
        }
        self.position += cnt;
        Ok(())
    }

    /// Peek bytes at current position without advancing
    #[inline]
    pub fn peek(&self, len: usize) -> WsResult<&[u8]> {
        if self.position + len > self.data.len() {
            return Err(WsError::BufferOverflow);
        }
        Ok(&self.data[self.position..self.position + len])
    }

    /// Get a slice from current position with specified length and advance
    #[inline]
    pub fn read_slice(&mut self, len: usize) -> WsResult<&[u8]> {
        if self.position + len > self.data.len() {
            return Err(WsError::BufferOverflow);
        }
        let slice = &self.data[self.position..self.position + len];
        self.position += len;
        Ok(slice)
    }

    /// Get the total length
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if buffer is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get remaining length
    #[inline]
    pub fn remaining_len(&self) -> usize {
        self.data.len() - self.position
    }

    /// Consume the buffer and return the underlying Bytes (zero-copy)
    pub fn into_bytes(self) -> Bytes {
        self.data
    }

    /// Get a reference to the underlying Bytes
    #[inline]
    pub fn as_bytes(&self) -> &Bytes {
        &self.data
    }
}

/// Writable buffer for encoding SBE messages with zero-copy awareness
#[derive(Debug)]
pub struct WriteBuffer {
    data: BytesMut,
    position: usize,
}

impl WriteBuffer {
    /// Create a new write buffer with capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            data: BytesMut::with_capacity(capacity),
            position: 0,
        }
    }

    /// Write bytes to the buffer
    pub fn write(&mut self, bytes: &[u8]) -> WsResult<()> {
        if self.position + bytes.len() > self.data.capacity() {
            return Err(WsError::BufferOverflow);
        }
        if self.position > self.data.len() {
            self.data.resize(self.position, 0);
        }
        self.data[self.position..self.position + bytes.len()].copy_from_slice(bytes);
        self.position += bytes.len();
        Ok(())
    }

    /// Get current position
    #[inline]
    pub fn position(&self) -> usize {
        self.position
    }

    /// Set position
    #[inline]
    pub fn set_position(&mut self, pos: usize) -> WsResult<()> {
        if pos > self.data.capacity() {
            return Err(WsError::BufferOverflow);
        }
        self.position = pos;
        Ok(())
    }

    /// Freeze into immutable Bytes (zero-copy)
    pub fn freeze(mut self) -> Bytes {
        self.data.truncate(self.position);
        self.data.freeze()
    }

    /// Get as mutable slice for SBE encoder
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        if self.position > self.data.len() {
            self.data.resize(self.position, 0);
        }
        &mut self.data
    }

    /// Get the current length written
    #[inline]
    pub fn len(&self) -> usize {
        self.position
    }

    /// Check if empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.position == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_copy_buffer() {
        let data = vec![1, 2, 3, 4, 5];
        let mut buf = ZeroCopyBuffer::new(data);

        assert_eq!(buf.position(), 0);
        assert_eq!(buf.remaining_len(), 5);
        assert_eq!(buf.as_slice(), &[1, 2, 3, 4, 5]);

        assert_eq!(buf.read_slice(2).unwrap(), &[1, 2]);
        assert_eq!(buf.position(), 2);
        assert_eq!(buf.remaining_len(), 3);
    }

    #[test]
    fn test_write_buffer() {
        let mut buf = WriteBuffer::new(10);

        buf.write(&[1, 2, 3]).unwrap();
        assert_eq!(buf.position(), 3);

        buf.write(&[4, 5]).unwrap();
        assert_eq!(buf.position(), 5);

        let bytes = buf.freeze();
        assert_eq!(&bytes[..], &[1, 2, 3, 4, 5]);
    }
}
