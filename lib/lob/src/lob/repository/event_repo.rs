use crate::lob::types::lob_types::EntityEvent;

pub struct EventRepo {}

impl EventRepo {
    pub(crate) fn save_batch(&self, p0: Option<Vec<EntityEvent>>) {
        todo!()
    }
}

impl EventRepo {
    pub(crate) fn save(&self, entity_event: EntityEvent) {
        //todo 使用mysql 存储entity_event
        todo!()
    }
}
