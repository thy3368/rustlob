新写个文档

# 基于EntityChangeLog的微批向量计算流式系统探索：Command、EntityChangeLog、Entity的Pod定义

目标： Command、EntityChangeLog 需要 支持网络二进制编解码开销最小，同时及simd友好


# 基于/Users/hongyaotang/src/rustlob/lib/common/diff/src/diff/entity_change_log.rs EntityChangeLog

## 概述

本文档定义了低时延中心化交易所（CEX）系统的编码规范，重点关注SOA（Structure of Arrays）/SIMD友好的POD（Plain Old
Data）类型设计

## 设计理念

### CEX系统的本质：微批向量计算流式数据系统

中心化交易所（CEX）系统的核心目标是**高吞吐**和**低时延**，其本质是一个**计算EntityChangeLog的微批向量计算流式数据系统**通过消息队列。


newordercmd-order-trade-balance-kline等的 EntityChangeLog




