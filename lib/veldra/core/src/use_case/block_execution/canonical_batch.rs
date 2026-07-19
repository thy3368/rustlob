use std::cmp::Ordering;
use std::collections::BTreeSet;

use super::BuildBlockError;
use crate::entity::{CommandEnvelope, ProductCommand, SpotCommand, TreasuryCommand};

/// 对 block builder 输入做最小防串改校验：
/// 1. batch 内不能重复注入 command_id / (account_id, nonce)
/// 2. envelope 身份必须和业务命令发起方一致
/// 3. 输入顺序必须已经是 canonical order，避免上游重排同一批命令
pub(in crate::use_case::block_execution) fn validate_and_clone_canonical_commands(
    commands: &[CommandEnvelope<ProductCommand>],
) -> Result<Vec<CommandEnvelope<ProductCommand>>, BuildBlockError> {
    validate_batch_uniqueness(commands)?;
    let canonical = canonical_sort_commands(commands);
    if canonical.as_slice() != commands {
        return Err(BuildBlockError::NonCanonicalCommandOrder);
    }
    Ok(canonical)
}

/// 生成 block 内唯一、稳定的命令顺序。
/// 当前规则是 ALO 现货单优先，其余命令再按通用 envelope 字段做全序比较。
pub(in crate::use_case::block_execution) fn canonical_sort_commands(
    commands: &[CommandEnvelope<ProductCommand>],
) -> Vec<CommandEnvelope<ProductCommand>> {
    let mut indexed = commands.iter().enumerate().collect::<Vec<_>>();
    indexed.sort_by(|(left_index, left), (right_index, right)| {
        compare_command_envelopes(left, *left_index, right, *right_index)
    });
    indexed.into_iter().map(|(_, command)| command.clone()).collect()
}

fn validate_batch_uniqueness(
    commands: &[CommandEnvelope<ProductCommand>],
) -> Result<(), BuildBlockError> {
    // 这里只做 batch 内部的一致性和去重，不引入跨 batch nonce window。
    let mut command_ids = BTreeSet::new();
    let mut account_nonces = BTreeSet::new();

    for command in commands {
        if command.timestamp_ns == 0 {
            return Err(BuildBlockError::ZeroCommandTimestamp {
                command_id: command.command_id.clone(),
            });
        }

        if !command_ids.insert(command.command_id.clone()) {
            return Err(BuildBlockError::DuplicateCommandId {
                command_id: command.command_id.clone(),
            });
        }

        let nonce_key = (command.account_id.clone(), command.nonce);
        if !account_nonces.insert(nonce_key.clone()) {
            return Err(BuildBlockError::DuplicateAccountNonce {
                account_id: nonce_key.0,
                nonce: nonce_key.1,
            });
        }

        if let Some(command_party_id) = command_party_id(&command.command) {
            if command.account_id != command_party_id {
                return Err(BuildBlockError::EnvelopeAccountMismatch {
                    command_id: command.command_id.clone(),
                    envelope_account_id: command.account_id.clone(),
                    command_party_id: command_party_id.to_string(),
                });
            }
        }
    }

    Ok(())
}

fn compare_command_envelopes(
    left: &CommandEnvelope<ProductCommand>,
    left_index: usize,
    right: &CommandEnvelope<ProductCommand>,
    right_index: usize,
) -> Ordering {
    // 用稳定 tie-break 链把部分有序字段补成严格全序，保证 block root 可复算。
    command_priority(left)
        .cmp(&command_priority(right))
        .then_with(|| left.timestamp_ns.cmp(&right.timestamp_ns))
        .then_with(|| left.account_id.cmp(&right.account_id))
        .then_with(|| left.nonce.cmp(&right.nonce))
        .then_with(|| left.command_id.cmp(&right.command_id))
        .then_with(|| left_index.cmp(&right_index))
}

fn command_priority(command: &CommandEnvelope<ProductCommand>) -> u8 {
    // 参考 Hyperliquid 的 maker-friendly 思路，先给 ALO 单独 priority bucket。
    if is_alo_priority_command(&command.command) {
        return 0;
    }
    1
}

fn is_alo_priority_command(command: &ProductCommand) -> bool {
    matches!(
        command,
        ProductCommand::Spot(SpotCommand::PlaceSpotOrderV2(command))
            if matches!(command.tif.as_str(), "alo" | "Alo")
    )
}

fn command_party_id(command: &ProductCommand) -> Option<&str> {
    // 用 payload 中的业务发起方反查 envelope.account_id，防止包裹层身份被串改。
    match command {
        ProductCommand::Spot(SpotCommand::PlaceSpotOrderV2(command)) => {
            Some(command.party_id.as_str())
        }
        ProductCommand::Treasury(TreasuryCommand::DepositQuote(command)) => {
            Some(command.party_id.as_str())
        }
        ProductCommand::Treasury(TreasuryCommand::WithdrawQuote(command)) => {
            Some(command.party_id.as_str())
        }
        ProductCommand::Perp(_) => None,
    }
}
