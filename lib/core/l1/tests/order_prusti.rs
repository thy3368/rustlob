#![allow(warnings)]

extern crate prusti_contracts;

use prusti_contracts::*;

struct Order {
    total_qty: u64,
    filled_qty: u64,
}

impl Order {
    #[pure]
    #[ensures(result == (self.filled_qty <= self.total_qty))]
    fn is_valid(&self) -> bool {
        self.filled_qty <= self.total_qty
    }

    #[requires(self.is_valid())]
    #[requires(filled_qty <= self.total_qty - self.filled_qty)]
    #[ensures(self.filled_qty == old(self.filled_qty) + filled_qty)]
    fn fill(&mut self, filled_qty: u64) {
        self.filled_qty += filled_qty;
    }
}
