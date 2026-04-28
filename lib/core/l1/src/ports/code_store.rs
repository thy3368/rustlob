use alloy_primitives::B256;

use crate::CodeBlob;

pub trait CodeStore {
    type Error;

    fn code(&self, code_hash: B256) -> Result<Option<CodeBlob>, Self::Error>;
    fn put_code(&mut self, code: CodeBlob) -> Result<(), Self::Error>;
}
