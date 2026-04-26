# 扩展性设计

## 产品/合约 技术类型分类

- 原生代码， 用rust开发， 如native_spot,native_prep,native_option等。
- evm, 用脚本开发 如evm_spot,evm_prep,evm_option,evm_stock

## 抽象一个产品trait, 支持未来扩展

/plan /Users/hongyaotang/src/rustlob/operating/dex/design/flow/dex_block_execution_flow.xpdl 完善区块处理全流程，补缺失的，从交易请求进入开始， 有些资料可以参考  
/Users/hongyaotang/src/rustlob/operating/dex/竞品分析/hyperliquid下面相关的文件  

