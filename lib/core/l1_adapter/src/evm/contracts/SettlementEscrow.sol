pragma solidity ^0.8.19;

contract SettlementEscrow {
    struct Settlement {
        address beneficiary;
        uint256 amount;
        bool released;
    }

    mapping(bytes32 => Settlement) private settlements;

    event SettlementCreated(bytes32 indexed settlementId, address indexed beneficiary, uint256 amount);
    event SettlementReleased(bytes32 indexed settlementId, address indexed beneficiary);

    function createSettlement(bytes32 settlementId, address beneficiary, uint256 amount) external {
        require(beneficiary != address(0), "invalid beneficiary");
        require(amount > 0, "amount must be positive");
        require(settlements[settlementId].beneficiary == address(0), "already exists");

        settlements[settlementId] = Settlement({
            beneficiary: beneficiary,
            amount: amount,
            released: false
        });

        emit SettlementCreated(settlementId, beneficiary, amount);
    }

    function releaseSettlement(bytes32 settlementId) external {
        Settlement storage settlement = settlements[settlementId];
        require(settlement.beneficiary != address(0), "not found");
        require(!settlement.released, "already released");

        settlement.released = true;

        emit SettlementReleased(settlementId, settlement.beneficiary);
    }

    function getAmount(bytes32 settlementId) external view returns (uint256) {
        return settlements[settlementId].amount;
    }

    function isReleased(bytes32 settlementId) external view returns (bool) {
        return settlements[settlementId].released;
    }
}
