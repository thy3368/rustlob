写个流程建模skill 生成xpdl 参考 /Users/hongyaotang/src/rustlob/docs/hyperliquid-use-case-define/SKILL.md

优化skill /Users/hongyaotang/src/rustlob/.agents/skills/clean-architecture/SKILL.md 及references 

"
大的分三层

## core

- use_case
  input: command/query
- entity

## adapter

- inbound
- outbound

## infra (外部三方工具）

adapter是core与 infra的胶水
"


     测试金字塔（Clean Architecture 版）                                                                                                                                 ▼ MCP                                   
                                                                                                                                                                         • code-review-graph Connected           
            ▲                                                                                                                                                            • context7 Connected                    
           /│\      E2E 测试 (tests/e2e/)                                                                                                                                • grep_app Connected                    
          / │ \     整链验证：HTTP -> Core -> DB                                                                                                                         • websearch Connected                   
         /  │  \                                                                                                                                                                                                 
        /───┼───\                                                                                                                                                        LSP                                     
       /    │    \   集成测试 (tests/*_integration_test.rs)                                                                                                              LSPs will activate as files are read    
      /     │     \  跨层验证：Adapter + UseCase + Real Infra                                                                                                                                                    
     /      │      \                                                                                                                                                                                             
     ───────┼─────── 单元测试 (src/**/tests/ 或 inline #[test])                                                                                                                                                  
            │        单层验证：UseCase (mocked) / Entity                                                                                                                                                         



use case的单元测试放哪？ 另外集成测试放哪？

