use common_entity::{DomainReadModel, QueryUseCase, stable_string_entity_id};
use example_core_entity::spot::SpotOrderV7;
use thiserror::Error;

/// 查询用户当前开放现货订单的输入。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenSpotOrdersQuery {
    pub user: String,
    pub dex: Option<String>,
}

/// 查询开放现货订单所需的领域读模型。
#[derive(Debug, Clone)]
pub struct SpotOrderV7ReadModel(pub Vec<SpotOrderV7>);

impl DomainReadModel for SpotOrderV7ReadModel {}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum OpenSpotOrdersError {
    #[error("user must be a 20-byte hexadecimal address")]
    InvalidUser,
    #[error("dex must not be empty")]
    InvalidDex,
}

/// 校验并计算用户当前开放现货订单。
#[derive(Debug, Clone, Copy, Default)]
pub struct OpenSpotOrdersQueryUseCase;

impl QueryUseCase<OpenSpotOrdersQuery, SpotOrderV7ReadModel> for OpenSpotOrdersQueryUseCase {
    type Output = Vec<SpotOrderV7>;
    type Error = OpenSpotOrdersError;

    fn validate_query(&self, query: &OpenSpotOrdersQuery) -> Result<(), Self::Error> {
        let address = query.user.strip_prefix("0x").ok_or(OpenSpotOrdersError::InvalidUser)?;
        if address.len() != 40 || !address.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(OpenSpotOrdersError::InvalidUser);
        }
        if query.dex.as_deref().is_some_and(|dex| dex.trim().is_empty()) {
            return Err(OpenSpotOrdersError::InvalidDex);
        }
        Ok(())
    }

    fn compute_query(
        &self,
        query: &OpenSpotOrdersQuery,
        read_model: SpotOrderV7ReadModel,
    ) -> Result<Self::Output, Self::Error> {
        let user_id = stable_string_entity_id(&query.user);
        let mut orders = read_model
            .0
            .into_iter()
            .filter(|order| {
                let base = order.order_base();
                base.owner.0 == user_id && order.status().is_cancelable()
            })
            .collect::<Vec<_>>();
        orders.sort_unstable_by_key(|order| {
            let base = order.order_base();
            (base.created_at, base.order_id)
        });
        Ok(orders)
    }
}
