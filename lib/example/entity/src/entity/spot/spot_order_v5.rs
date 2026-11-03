use rust_decimal::Decimal;
use common_entity::EntityError;
use crate::entity::common::common_order::OrderBase;
use crate::entity::spot::spot_order_primitives::{OrderSide, SpotOrderStatus};
use crate::entity::spot::spot_order_v3::{SpotOrderV3, SpotOrderV3Any, Pending, MatchSpotOrderV3Input, MatchResult};

/// V5 的状态动作矩阵（业务状态由包装层保存，宏仅用于声明合法拓扑）。
mod matrix {
    use tabular_center::transition_matrix;
    transition_matrix! {
        machine SpotOrderLifecycle;
        context OrderContext;
        state LifecycleState;
        action LifecycleAction;
        effects LifecycleEffect { Changed }
        initial Pending;
        states { Pending, Open, PartiallyFilled, Filled, Canceled, Rejected }
        actions { Trigger, MatchRest, RecordFill, Cancel, Modify, Reject }
        Pending => [ HANDLE, HANDLE, IGNORE, HANDLE, HANDLE, HANDLE ];
        Open => [ IGNORE, HANDLE, HANDLE, HANDLE, HANDLE, HANDLE ];
        PartiallyFilled => [ IGNORE, HANDLE, HANDLE, HANDLE, HANDLE, HANDLE ];
        Filled => [ IGNORE, IGNORE, IGNORE, IGNORE, IGNORE, IGNORE ];
        Canceled => [ IGNORE, IGNORE, IGNORE, IGNORE, IGNORE, IGNORE ];
        Rejected => [ IGNORE, IGNORE, IGNORE, IGNORE, IGNORE, IGNORE ];
    }
    pub struct OrderContext;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderV5Error { Invalid(String), IllegalAction, NotTriggered, Entity(String) }
impl From<EntityError> for SpotOrderV5Error { fn from(e: EntityError) -> Self { Self::Entity(e.to_string()) } }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotOrderV5 { record: OrderBase }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchSpotOrderV5Input { pub trade_id_start:u64, pub maker_fee_bps:u64, pub taker_fee_bps:u64, pub timestamp:u64 }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchSpotOrderV5Result { Resting, PartiallyFilled, Filled, CanceledAfterPartialFill, Rejected }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriggerSpotOrderV5Result { NotTriggered, Matched(MatchSpotOrderV5Result) }

impl SpotOrderV5 {
    pub fn place(mut order: OrderBase) -> Self { order.status=SpotOrderStatus::Pending; Self{record:order} }
    pub fn place_normal_tpsl(parent: OrderBase, children: Vec<OrderBase>) -> (Self, Vec<Self>) {
        let mut p=Self::place(parent); p.record.group_relation=crate::entity::spot::spot_order_v2::OrderGroupRelation::NormalTpslParent; let id=p.record.order_id;
        let cs=children.into_iter().map(|mut c| { c.status=SpotOrderStatus::Pending; c.group_relation=crate::entity::spot::spot_order_v2::OrderGroupRelation::NormalTpslChild{parent_order_id:id}; Self{record:c} }).collect(); (p,cs)
    }
    pub fn from_record(order: OrderBase) -> Result<Self, SpotOrderV5Error> { Ok(Self{record:order}) }
    pub fn order_base(&self)->&OrderBase { &self.record }
    pub fn status(&self)->SpotOrderStatus { self.record.status }
    pub fn into_record(self)->OrderBase { self.record }
    pub fn cancel(&self, timestamp:u64)->Result<Self,SpotOrderV5Error> { if !self.status().is_cancelable(){return Err(SpotOrderV5Error::IllegalAction)} let mut r=self.record.clone(); r.status=SpotOrderStatus::Canceled; r.updated_at=timestamp; r.version=r.version.checked_add(1).ok_or(SpotOrderV5Error::IllegalAction)?; Ok(Self{record:r}) }
    pub fn modify(&self, price:Decimal,size:Decimal,timestamp:u64)->Result<Self,SpotOrderV5Error>{ if !matches!(self.status(),SpotOrderStatus::Pending|SpotOrderStatus::Open|SpotOrderStatus::PartiallyFilled)||price<=Decimal::ZERO||size<=Decimal::ZERO{return Err(SpotOrderV5Error::IllegalAction)} let mut r=self.record.clone(); r.limit_px=price;r.size=size;r.orig_size=size;r.updated_at=timestamp; Ok(Self{record:r}) }
    pub fn match_order(&self, makers:Vec<SpotOrderV5>, input:MatchSpotOrderV5Input)->Result<MatchSpotOrderV5Result,SpotOrderV5Error>{ if self.record.is_trigger() && self.status()==SpotOrderStatus::Pending{return Err(SpotOrderV5Error::IllegalAction)} let taker=SpotOrderV3::<Pending>::try_from_record(self.record.clone()).map_err(SpotOrderV5Error::from)?; let ms=makers.into_iter().map(|m| SpotOrderV3Any::from_record(m.record).map_err(SpotOrderV5Error::from)).collect::<Result<Vec<_>,_>>()?; let r=taker.match_with_input(ms,MatchSpotOrderV3Input{trade_id_start:input.trade_id_start,maker_fee_bps:input.maker_fee_bps,taker_fee_bps:input.taker_fee_bps,timestamp:input.timestamp})?; Ok(match r{MatchResult::Resting{..}=>MatchSpotOrderV5Result::Resting,MatchResult::PartiallyFilled{..}=>MatchSpotOrderV5Result::PartiallyFilled,MatchResult::Filled{..}=>MatchSpotOrderV5Result::Filled,MatchResult::CanceledAfterPartialFill{..}=>MatchSpotOrderV5Result::CanceledAfterPartialFill,MatchResult::Rejected{..}=>MatchSpotOrderV5Result::Rejected}) }
    pub fn trigger(&self, market_price:Decimal, makers:Vec<SpotOrderV5>, input:MatchSpotOrderV5Input)->Result<TriggerSpotOrderV5Result,SpotOrderV5Error>{ if self.status()!=SpotOrderStatus::Pending||!self.record.is_trigger(){return Err(SpotOrderV5Error::IllegalAction)} let px=self.record.trigger_px.ok_or(SpotOrderV5Error::IllegalAction)?; let hit=match (self.record.side,self.record.trigger_role()){(OrderSide::Buy,Some(crate::OrderTriggerRole::TakeProfit))|(OrderSide::Sell,Some(crate::OrderTriggerRole::StopLoss))=>market_price<=px,_=>market_price>=px}; if !hit{return Ok(TriggerSpotOrderV5Result::NotTriggered)} let mut active=self.clone(); active.record.status=SpotOrderStatus::Open; Ok(TriggerSpotOrderV5Result::Matched(active.match_order(makers,input)?)) }
}
