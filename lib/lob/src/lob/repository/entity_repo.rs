use crate::lob::types::lob_types::EntityEvent;

pub struct EntityRepo {}

impl EntityRepo {
    //根据replay回放到entity数据库
    pub(crate) fn replay(&self, p0: Option<Vec<EntityEvent>>) {
        todo!()
    }
}
