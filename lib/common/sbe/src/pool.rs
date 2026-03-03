use crossbeam::queue::ArrayQueue;
use std::sync::Arc;

pub struct BufferPool {
    pool: Arc<ArrayQueue<Vec<u8>>>,
    buffer_size: usize,
}

impl BufferPool {
    pub fn new(capacity: usize, buffer_size: usize) -> Self {
        let pool = Arc::new(ArrayQueue::new(capacity));
        for _ in 0..capacity {
            let _ = pool.push(vec![0u8; buffer_size]);
        }
        Self { pool, buffer_size }
    }

    pub fn acquire(&self) -> PooledBuffer {
        let buf = self.pool.pop()
            .unwrap_or_else(|| vec![0u8; self.buffer_size]);
        PooledBuffer {
            buf,
            pool: self.pool.clone(),
            buffer_size: self.buffer_size,
        }
    }
}

pub struct PooledBuffer {
    pub buf: Vec<u8>,
    pool: Arc<ArrayQueue<Vec<u8>>>,
    buffer_size: usize,
}

impl Drop for PooledBuffer {
    fn drop(&mut self) {
        let mut buf = std::mem::take(&mut self.buf);
        buf.clear();
        buf.resize(self.buffer_size, 0);
        let _ = self.pool.push(buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_acquire_and_return() {
        let pool = BufferPool::new(2, 1024);
        let buf1 = pool.acquire();
        assert_eq!(buf1.buf.len(), 1024);
        drop(buf1);
        let buf2 = pool.acquire();
        assert_eq!(buf2.buf.len(), 1024);
    }
}
