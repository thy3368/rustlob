use crate::info::common::validate::{ensure_type, validate_hex_address_field};
use crate::info::common::wire::UserRoleResponseWire;
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = crate::info::common::wire::UserRoleResponseWire;
}

#[derive(Debug, serde::Deserialize)]
pub struct RequestWire {
    #[serde(rename = "type")]
    type_: String,
    user: String,
}

pub async fn handle(
    body: &[u8],
    _deps: &InfoQueryDeps,
) -> Result<reply::ResponseWire, InfoHttpError> {
    let request: RequestWire =
        serde_json::from_slice(body).map_err(InfoHttpError::from_json_error)?;
    ensure_type(&request.type_, "userRole")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    UserRoleResponseWire { role: "user".to_string(), data: None }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::info::common::wire::UserRoleDataWire;

    #[test]
    fn user_role_user_shape_serializes_without_data() {
        let value = serde_json::to_value(stub_response()).unwrap();
        assert_eq!(value, json!({ "role": "user" }));
    }

    #[test]
    fn user_role_agent_shape_serializes_with_user_data() {
        let value = serde_json::to_value(UserRoleResponseWire {
            role: "agent".to_string(),
            data: Some(UserRoleDataWire::Agent {
                user: "0x0000000000000000000000000000000000000001".to_string(),
                extra: Default::default(),
            }),
        })
        .unwrap();
        assert_eq!(
            value,
            json!({
                "role": "agent",
                "data": { "user": "0x0000000000000000000000000000000000000001" }
            })
        );
    }

    #[test]
    fn user_role_sub_account_shape_serializes_with_master_data() {
        let value = serde_json::to_value(UserRoleResponseWire {
            role: "subAccount".to_string(),
            data: Some(UserRoleDataWire::SubAccount {
                master: "0x0000000000000000000000000000000000000002".to_string(),
                extra: Default::default(),
            }),
        })
        .unwrap();
        assert_eq!(
            value,
            json!({
                "role": "subAccount",
                "data": { "master": "0x0000000000000000000000000000000000000002" }
            })
        );
    }
}
