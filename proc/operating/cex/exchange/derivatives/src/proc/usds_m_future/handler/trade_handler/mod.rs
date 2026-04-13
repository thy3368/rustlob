use base_types::handler::handler_update2::DomainEventSet;
use cmd_handler::DomainEventSet;

pub use cmd::auto_cancel_all_open_orders_handler::AutoCancelAllOpenOrdersCmdHandler;
pub use cmd::cancel_all_open_orders_handler::CancelAllOpenOrdersCmdHandler;
pub use cmd::cancel_multiple_orders_handler::CancelMultipleOrdersCmdHandler;
pub use cmd::cancel_order_handler::CancelOrderCmdHandler;
pub use cmd::change_initial_leverage_handler::ChangeInitialLeverageCmdHandler;
pub use cmd::change_margin_type_handler::ChangeMarginTypeCmdHandler;
pub use cmd::change_multi_assets_mode_handler::ChangeMultiAssetsModeCmdHandler;
pub use cmd::change_position_mode_handler::ChangePositionModeCmdHandler;
pub use cmd::modify_isolated_position_margin_handler::ModifyIsolatedPositionMarginCmdHandler;
pub use cmd::modify_multiple_orders_handler::ModifyMultipleOrdersCmdHandler;
pub use cmd::modify_order_handler::ModifyOrderCmdHandler;
pub use cmd::new_order_handler::NewOrderCmdHandler;
pub use cmd::new_order_test_handler::NewOrderTestCmdHandler;
pub use cmd::place_multiple_orders_handler::PlaceMultipleOrdersCmdHandler;

pub mod cmd;
pub mod query;
#[derive(Debug, Clone, Default)]
pub struct EmptyStateSet;

impl DomainEventSet for EmptyStateSet {
    fn domain_event_count(&self) -> usize {
        0
    }
}
