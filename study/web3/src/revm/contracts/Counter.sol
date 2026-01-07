// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * Counter 合约 - 一个简单的计数器示例
 *
 * 功能:
 * - increment(): 增加计数器
 * - get(): 获取当前计数值
 * - reset(): 重置计数器为0
 */
contract Counter {
    uint256 public count;

    event Incremented(uint256 newCount);
    event Reset();

    constructor() {
        count = 0;
    }

    function increment() public {
        count += 1;
        emit Incremented(count);
    }

    function get() public view returns (uint256) {
        return count;
    }

    function reset() public {
        count = 0;
        emit Reset();
    }
}
