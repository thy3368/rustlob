use account::{Price, Quantity, Side};

use crate::proc::prep_types::InternalOrder;

#[allow(dead_code)]
pub struct SingleLobRepo {
    lobs: Vec<InternalOrder>
}

impl SingleLobRepo {
    #[allow(dead_code)]
    fn match_orders(&self, _side: Side, _price: Price, _quantity: Quantity) -> Option<Vec<&InternalOrder>> {
        // TODO: 实现订单匹配逻辑
        None
    }
}
