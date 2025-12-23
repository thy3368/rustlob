//! BDD测试 - 用户多账户多余额操作
//!
//! 测试真实业务场景：
//! 1. 一个用户拥有多个账户（现货、合约等）
//! 2. 每个账户有多个资产余额（USDT、BTC、ETH）
//! 3. 对不同账户的余额进行冻结、扣减、解冻等操作
//!
//! 业务模型：
//! User (用户)
//!   ├─ Account 1 (现货账户)
//!   │   ├─ Balance (USDT)
//!   │   ├─ Balance (BTC)
//!   │   └─ Balance (ETH)
//!   ├─ Account 2 (合约逐仓账户)
//!   │   └─ Balance (USDT)
//!   └─ Account 3 (资金账户)
//!       └─ Balance (USDT)

use account::{
    adaptor::outbound::{account_repo::MemoryAccountRepo, balance_repo::MemoryBalanceRepo},
    domain::{
        entity::{Account, AccountId, AccountType, AssetId, Balance, Timestamp, UserId},
        repo::{AccountRepo, BalanceRepo}
    }
};
use base_types::Price;

// ============================================================================
// 辅助函数
// ============================================================================

/// 固定时间戳函数（用于测试）
fn fixed_timestamp() -> Timestamp { 1000000000u64 }

/// 打印用户账户结构
fn print_account_info(account: &Account, account_name: &str) {
    println!(
        "   {} - ID: {}, 类型: {:?}, 状态: {:?}",
        account_name, account.id.0, account.account_type, account.status
    );
}

/// 打印余额信息
fn print_balance(_account_name: &str, asset_name: &str, balance: &Balance) {
    println!(
        "      └─ {} - 可用: {}, 冻结: {}, 总计: {}",
        asset_name,
        balance.available,
        balance.frozen,
        balance.total()
    );
}

// ============================================================================
// Scenario 1: 一个用户拥有多个账户，每个账户有多个余额
// ============================================================================

#[cfg(test)]
mod user_multi_account {
    use super::*;

    #[test]
    fn scenario_user_has_multiple_accounts_with_balances() {
        // Feature: 用户多账户管理
        // Scenario: 一个用户拥有现货账户、合约账户、资金账户

        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📊 Scenario: 用户拥有多个账户和余额");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        // ====================================================================
        // Given: 创建仓储和用户
        // ====================================================================
        let mut account_repo = MemoryAccountRepo::new();
        let mut balance_repo = MemoryBalanceRepo::new(fixed_timestamp);
        let user_id = UserId(1001);
        let now = fixed_timestamp();

        println!("✅ Given: 创建用户 {}", user_id.0);

        // ====================================================================
        // When: 为用户创建多个账户
        // ====================================================================
        println!("\n✅ When: 为用户创建多个账户");

        // 账户1: 现货账户
        let spot_account_id = AccountId(10001);
        let spot_account = Account::new(spot_account_id, user_id, AccountType::Spot, now);
        account_repo.save(spot_account);
        println!("   创建现货账户: ID {}", spot_account_id.0);

        // 账户2: 合约逐仓账户
        let perp_account_id = AccountId(10002);
        let perp_account = Account::new(perp_account_id, user_id, AccountType::PerpIsolated, now);
        account_repo.save(perp_account);
        println!("   创建合约逐仓账户: ID {}", perp_account_id.0);

        // 账户3: 资金账户
        let funding_account_id = AccountId(10003);
        let funding_account = Account::new(funding_account_id, user_id, AccountType::Funding, now);
        account_repo.save(funding_account);
        println!("   创建资金账户: ID {}", funding_account_id.0);

        // ====================================================================
        // When: 为每个账户创建余额
        // ====================================================================
        println!("\n✅ When: 为每个账户创建余额");

        // 现货账户余额
        println!("   现货账户余额:");
        let spot_usdt =
            Balance::with_available(spot_account_id, AssetId::USDT, Price::from_raw(50_000_000_000 as i64), now);
        balance_repo.save(spot_usdt);
        println!("      USDT: 50000");

        let spot_btc = Balance::with_available(spot_account_id, AssetId::BTC, Price::from_raw(200_000_000 as i64), now);
        balance_repo.save(spot_btc);
        println!("      BTC: 2");

        let spot_eth =
            Balance::with_available(spot_account_id, AssetId::ETH, Price::from_raw(10_000_000_000 as i64), now);
        balance_repo.save(spot_eth);
        println!("      ETH: 100");

        // 合约账户余额
        println!("   合约逐仓账户余额:");
        let perp_usdt =
            Balance::with_available(perp_account_id, AssetId::USDT, Price::from_raw(100_000_000_000 as i64), now);
        balance_repo.save(perp_usdt);
        println!("      USDT: 100000");

        // 资金账户余额
        println!("   资金账户余额:");
        let funding_usdt =
            Balance::with_available(funding_account_id, AssetId::USDT, Price::from_raw(500_000_000_000 as i64), now);
        balance_repo.save(funding_usdt);
        println!("      USDT: 500000");

        // ====================================================================
        // Then: 验证账户和余额结构
        // ====================================================================
        println!("\n✅ Then: 验证完整账户结构");

        // 验证现货账户
        let spot = account_repo.get(spot_account_id).unwrap();
        assert_eq!(spot.user_id, user_id);
        assert_eq!(spot.account_type, AccountType::Spot);
        print_account_info(spot, "现货账户");

        let spot_balances = balance_repo.get_all_by_account(spot_account_id);
        assert_eq!(spot_balances.len(), 3, "现货账户应有3种资产");
        for balance in spot_balances {
            let asset_name = match balance.asset_id {
                AssetId::USDT => "USDT",
                AssetId::BTC => "BTC",
                AssetId::ETH => "ETH",
                _ => "Unknown"
            };
            print_balance("现货账户", asset_name, balance);
        }

        // 验证合约账户
        let perp = account_repo.get(perp_account_id).unwrap();
        assert_eq!(perp.user_id, user_id);
        assert_eq!(perp.account_type, AccountType::PerpIsolated);
        print_account_info(perp, "合约逐仓账户");

        let perp_balances = balance_repo.get_all_by_account(perp_account_id);
        assert_eq!(perp_balances.len(), 1, "合约账户应有1种资产");
        print_balance("合约账户", "USDT", perp_balances[0]);

        // 验证资金账户
        let funding = account_repo.get(funding_account_id).unwrap();
        assert_eq!(funding.user_id, user_id);
        assert_eq!(funding.account_type, AccountType::Funding);
        print_account_info(funding, "资金账户");

        let funding_balances = balance_repo.get_all_by_account(funding_account_id);
        assert_eq!(funding_balances.len(), 1, "资金账户应有1种资产");
        print_balance("资金账户", "USDT", funding_balances[0]);

        println!("\n✅ 测试通过：用户多账户多余额结构正确");
    }
}
