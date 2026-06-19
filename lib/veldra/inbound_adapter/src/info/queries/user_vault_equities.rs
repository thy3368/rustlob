use crate::info::common::validate::{ensure_type, validate_hex_address_field};
use crate::info::error::InfoHttpError;
use crate::info::queries::InfoQueryDeps;

pub mod reply {
    use serde::Serialize;
    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub struct UserVaultEquityWire {
        #[serde(rename = "vaultAddress")]
        pub vault_address: String,
        pub equity: String,
    }
    pub type ResponseWire = Vec<UserVaultEquityWire>;
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
    ensure_type(&request.type_, "userVaultEquities")?;
    validate_hex_address_field("user", &request.user)?;
    Ok(stub_response())
}

pub(crate) fn stub_response() -> reply::ResponseWire {
    vec![reply::UserVaultEquityWire {
        vault_address: "0xdfc24b077bc1425ad1dea75bcb6f8158e10df303".to_string(),
        equity: "742500.082809".to_string(),
    }]
}
