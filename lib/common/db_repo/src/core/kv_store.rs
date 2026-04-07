use bytecheck::CheckBytes;
use rancor::Strategy;
use rkyv::{
    api::high::{HighSerializer, HighValidator, from_bytes, to_bytes},
    de::Pool,
    rancor::Error,
    ser::allocator::ArenaHandle,
    Archive, Deserialize, Serialize,
};
use thiserror::Error as ThisError;

pub type StorageSource = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, ThisError)]
pub enum StorageError {
    #[error("failed to open storage")]
    Open {
        #[source]
        source: StorageSource,
    },
    #[error("failed to read value")]
    Read {
        #[source]
        source: StorageSource,
    },
    #[error("failed to write value")]
    Write {
        #[source]
        source: StorageSource,
    },
    #[error("failed to delete value")]
    Delete {
        #[source]
        source: StorageSource,
    },
    #[error("failed to encode or decode value")]
    Codec {
        #[source]
        source: StorageSource,
    },
}

pub trait KvStore: Send + Sync {
    fn put(&self, key: &[u8], value: &[u8]) -> Result<(), StorageError>;

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, StorageError>;

    fn delete(&self, key: &[u8]) -> Result<(), StorageError>;

    fn contains(&self, key: &[u8]) -> Result<bool, StorageError> {
        self.get(key).map(|value| value.is_some())
    }
}

pub trait RkyvKvStoreExt: KvStore {
    fn put_obj<T>(&self, key: &[u8], value: &T) -> Result<(), StorageError>
    where
        T: for<'a> Serialize<HighSerializer<rkyv::util::AlignedVec, ArenaHandle<'a>, Error>>,
    {
        let bytes = to_bytes::<Error>(value).map_err(|e| StorageError::Codec {
            source: Box::new(e),
        })?;
        self.put(key, bytes.as_slice())
    }

    fn get_obj<T>(&self, key: &[u8]) -> Result<Option<T>, StorageError>
    where
        T: Archive,
        T::Archived: for<'a> CheckBytes<HighValidator<'a, Error>>
            + Deserialize<T, Strategy<Pool, Error>>,
    {
        let Some(bytes) = self.get(key)? else {
            return Ok(None);
        };

        let value = from_bytes::<T, Error>(&bytes).map_err(|e| StorageError::Codec {
            source: Box::new(e),
        })?;
        Ok(Some(value))
    }
}

impl<T: KvStore + ?Sized> RkyvKvStoreExt for T {}
