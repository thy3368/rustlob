use cmd_handler::EntityReplayableEvent;
use cmd_handler::use_case_def2::{
    CommandEnvelope, CommandMeta, CommandUseCaseOutbound, UseCaseReplyMapper,
};
use example_core::{DepositQuoteCmd, DepositQuoteError, DepositQuoteState};

use crate::common::{execute_deposit_quote_with_mapper, find_string_field, find_u64_field};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepositQuoteCliCommand {
    pub trader_id: String,
    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseDepositQuoteCliArgsError {
    TooManyArgs,
    InvalidAmount(String),
}

impl std::fmt::Display for ParseDepositQuoteCliArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyArgs => f.write_str("too many CLI arguments"),
            Self::InvalidAmount(raw) => write!(f, "invalid amount: {raw}"),
        }
    }
}

impl std::error::Error for ParseDepositQuoteCliArgsError {}

impl DepositQuoteCliCommand {
    fn into_envelope(self) -> CommandEnvelope<DepositQuoteCmd> {
        CommandEnvelope {
            meta: CommandMeta {
                trace_id: Some("cli-deposit-quote".to_string()),
                command_id: Some(format!("cli:{}:deposit:{}", self.trader_id, self.amount)),
            },
            command: DepositQuoteCmd { party_id: self.trader_id, amount: self.amount },
        }
    }
}

pub fn deposit_quote_cli_usage() -> &'static str {
    "usage: cargo run -p example_composition_root --bin cli_deposit_demo -- <trader_id> <amount>"
}

pub fn parse_deposit_quote_cli_args<I, S>(
    args: I,
) -> Result<DepositQuoteCliCommand, ParseDepositQuoteCliArgsError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut args = args.into_iter().map(Into::into);

    let trader_id = args.next().unwrap_or_else(|| "trader-1".to_string());
    let amount = match args.next() {
        Some(raw) => {
            raw.parse::<u64>().map_err(|_| ParseDepositQuoteCliArgsError::InvalidAmount(raw))?
        }
        None => 200,
    };

    if args.next().is_some() {
        return Err(ParseDepositQuoteCliArgsError::TooManyArgs);
    }

    Ok(DepositQuoteCliCommand { trader_id, amount })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepositQuoteCliResponse {
    pub summary: String,
    pub account_id: String,
}

#[derive(Debug, Clone, Copy, Default)]
struct DepositQuoteCliReplyMapper;

impl UseCaseReplyMapper for DepositQuoteCliReplyMapper {
    type Reply = DepositQuoteCliResponse;

    fn map(&self, events: Vec<EntityReplayableEvent>) -> Self::Reply {
        let account_id = find_string_field(&events, "account_id")
            .unwrap_or_else(|| "missing-account-id".to_string());
        let available_quote = find_u64_field(&events, "available_quote").unwrap_or(0);
        let frozen_quote = find_u64_field(&events, "frozen_quote").unwrap_or(0);

        DepositQuoteCliResponse {
            summary: format!(
                "accepted account_id={account_id} available_quote={available_quote} frozen_quote={frozen_quote}"
            ),
            account_id,
        }
    }
}

pub fn run_deposit_quote_cli<OB>(
    command: DepositQuoteCliCommand,
    outbound: &OB,
) -> Result<DepositQuoteCliResponse, DepositQuoteError>
where
    OB: ?Sized
        + Send
        + Sync
        + CommandUseCaseOutbound<DepositQuoteCmd, DepositQuoteState, DepositQuoteError>,
{
    execute_deposit_quote_with_mapper(
        command.into_envelope(),
        outbound,
        &DepositQuoteCliReplyMapper,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::TestOutbound;

    #[test]
    fn cli_adapter_translates_deposit_command_and_maps_text_response()
    -> Result<(), DepositQuoteError> {
        let outbound = TestOutbound::default();
        let command = DepositQuoteCliCommand { trader_id: "trader-1".to_string(), amount: 250 };

        let response = run_deposit_quote_cli(command, &outbound)?;
        let counts =
            outbound.snapshot_event_counts().map_err(|_| DepositQuoteError::StoreUnavailable)?;

        assert_eq!(response.account_id, "trader-1");
        assert_eq!(
            response.summary,
            "accepted account_id=trader-1 available_quote=1250 frozen_quote=0"
        );
        assert_eq!(counts, (1, 1));

        Ok(())
    }

    #[test]
    fn parse_cli_args_supports_defaults() -> Result<(), ParseDepositQuoteCliArgsError> {
        let command = parse_deposit_quote_cli_args(std::iter::empty::<String>())?;

        assert_eq!(command.trader_id, "trader-1");
        assert_eq!(command.amount, 200);

        Ok(())
    }

    #[test]
    fn parse_cli_args_rejects_too_many_args() {
        let result = parse_deposit_quote_cli_args(["a", "100", "extra"]);

        assert_eq!(result, Err(ParseDepositQuoteCliArgsError::TooManyArgs));
    }
}
