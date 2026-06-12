use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{
    CommandEnvelope, CommandMeta, CommandUseCaseExecutionError, CommandUseCaseOutbound,
    UseCaseReplyMapper,
};
use example_core::{WithdrawQuoteCmd, WithdrawQuoteError, WithdrawQuoteState};
use serde::Serialize;

use crate::common::{
    ExampleBusinessErrorMapping, ExampleCliParseErrorMapping, execute_withdraw_quote_with_mapper,
    find_string_field, find_u64_field,
};

pub const WITHDRAW_QUOTE_CLI_BIN: &str = "cli_withdraw_demo";
pub const WITHDRAW_QUOTE_CLI_DEFAULT_TRADER_ID: &str = "trader-1";
pub const WITHDRAW_QUOTE_CLI_DEFAULT_AMOUNT: u64 = 200;
const WITHDRAW_QUOTE_CLI_USAGE: &str = "usage: cargo run -p example_inbound_adapter --example cli_withdraw_demo -- <trader_id> <amount>";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WithdrawQuoteCliCommand {
    pub trader_id: String,
    pub amount: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseWithdrawQuoteCliArgsError {
    TooManyArgs,
    InvalidAmount(String),
}

impl std::fmt::Display for ParseWithdrawQuoteCliArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyArgs => f.write_str("too many CLI arguments"),
            Self::InvalidAmount(raw) => write!(f, "invalid amount: {raw}"),
        }
    }
}

impl std::error::Error for ParseWithdrawQuoteCliArgsError {}

impl ExampleCliParseErrorMapping for ParseWithdrawQuoteCliArgsError {
    fn cli_error_code(&self) -> &'static str {
        match self {
            Self::TooManyArgs => "too_many_args",
            Self::InvalidAmount(_) => "invalid_amount",
        }
    }
}

impl ExampleBusinessErrorMapping for WithdrawQuoteError {
    fn inbound_error_code(&self) -> &'static str {
        match self {
            WithdrawQuoteError::InvalidAmount => "invalid_amount",
            WithdrawQuoteError::InsufficientQuoteBalance => "insufficient_quote_balance",
            WithdrawQuoteError::ArithmeticOverflow => "arithmetic_overflow",
        }
    }

    fn http_status_code(&self) -> u16 {
        match self {
            WithdrawQuoteError::ArithmeticOverflow => 500,
            WithdrawQuoteError::InvalidAmount | WithdrawQuoteError::InsufficientQuoteBalance => 400,
        }
    }
}

impl WithdrawQuoteCliCommand {
    fn into_envelope(self) -> CommandEnvelope<WithdrawQuoteCmd> {
        CommandEnvelope {
            meta: CommandMeta {
                trace_id: Some("cli-withdraw-quote".to_string()),
                command_id: Some(format!("cli:{}:withdraw:{}", self.trader_id, self.amount)),
            },
            command: WithdrawQuoteCmd { party_id: self.trader_id, amount: self.amount },
        }
    }
}

pub fn withdraw_quote_cli_usage() -> &'static str {
    WITHDRAW_QUOTE_CLI_USAGE
}

pub fn parse_withdraw_quote_cli_args<I, S>(
    args: I,
) -> Result<WithdrawQuoteCliCommand, ParseWithdrawQuoteCliArgsError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut args = args.into_iter().map(Into::into);

    let trader_id = args.next().unwrap_or_else(|| WITHDRAW_QUOTE_CLI_DEFAULT_TRADER_ID.to_string());
    let amount = match args.next() {
        Some(raw) => {
            raw.parse::<u64>().map_err(|_| ParseWithdrawQuoteCliArgsError::InvalidAmount(raw))?
        }
        None => WITHDRAW_QUOTE_CLI_DEFAULT_AMOUNT,
    };

    if args.next().is_some() {
        return Err(ParseWithdrawQuoteCliArgsError::TooManyArgs);
    }

    Ok(WithdrawQuoteCliCommand { trader_id, amount })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WithdrawQuoteCliResponse {
    pub summary: String,
    pub account_id: String,
}

#[derive(Debug, Clone, Copy, Default)]
struct WithdrawQuoteCliReplyMapper;

impl UseCaseReplyMapper for WithdrawQuoteCliReplyMapper {
    type Reply = WithdrawQuoteCliResponse;

    fn map(&self, events: Vec<EntityReplayableEvent>) -> Self::Reply {
        let account_id = find_string_field(&events, "account_id")
            .unwrap_or_else(|| "missing-account-id".to_string());
        let available_quote = find_u64_field(&events, "available").unwrap_or(0);
        let frozen_quote = find_u64_field(&events, "frozen").unwrap_or(0);

        WithdrawQuoteCliResponse {
            summary: format!(
                "accepted account_id={account_id} available_quote={available_quote} frozen_quote={frozen_quote}"
            ),
            account_id,
        }
    }
}

pub fn run_withdraw_quote_cli<OB>(
    command: WithdrawQuoteCliCommand,
    outbound: &OB,
) -> Result<WithdrawQuoteCliResponse, CommandUseCaseExecutionError<WithdrawQuoteError, OB::Error>>
where
    OB: ?Sized
        + Send
        + Sync
        + CommandUseCaseOutbound<Command = WithdrawQuoteCmd, State = WithdrawQuoteState>,
    OB::Error: 'static,
{
    execute_withdraw_quote_with_mapper(
        command.into_envelope(),
        outbound,
        &WithdrawQuoteCliReplyMapper,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::WithdrawQuoteTestOutbound;

    #[test]
    fn cli_adapter_translates_withdraw_command_and_maps_text_response()
    -> Result<(), Box<dyn std::error::Error>> {
        let outbound = WithdrawQuoteTestOutbound::default();
        let command = WithdrawQuoteCliCommand { trader_id: "trader-1".to_string(), amount: 250 };

        let response = run_withdraw_quote_cli(command, &outbound)?;
        let counts = outbound.snapshot_event_counts()?;

        assert_eq!(response.account_id, "trader-1");
        assert_eq!(
            response.summary,
            "accepted account_id=trader-1 available_quote=750 frozen_quote=0"
        );
        assert_eq!(counts, (1, 1));

        Ok(())
    }

    #[test]
    fn parse_cli_args_supports_defaults() -> Result<(), ParseWithdrawQuoteCliArgsError> {
        let command = parse_withdraw_quote_cli_args(std::iter::empty::<String>())?;

        assert_eq!(command.trader_id, "trader-1");
        assert_eq!(command.amount, 200);

        Ok(())
    }

    #[test]
    fn parse_cli_args_rejects_too_many_args() {
        let result = parse_withdraw_quote_cli_args(["a", "100", "extra"]);

        assert_eq!(result, Err(ParseWithdrawQuoteCliArgsError::TooManyArgs));
    }
}
