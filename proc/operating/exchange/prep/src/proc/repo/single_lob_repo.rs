use account::{Price, Quantity, Side};

pub struct SingleLobRepo {}

impl SingleLobRepo {
    // pub fn new() -> SingleLobRepo {
    //
    // }

    fn match_orders(&self, side: Side, price: Price, quantity: Quantity) -> Option<Vec<&OrderEntry>> {}
}
