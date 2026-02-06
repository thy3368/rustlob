use std::collections::HashMap;

/// 向量时钟结构体
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorClock {
    node_id: String,
    counters: HashMap<String, i32>,
}

impl VectorClock {
    /// 创建一个新的向量时钟
    pub fn new(node_id: &str, all_nodes: &[&str]) -> Self {
        let mut counters = HashMap::new();
        for &node in all_nodes {
            counters.insert(node.to_string(), 0);
        }
        VectorClock { node_id: node_id.to_string(), counters }
    }

    /// 本地事件发生，递增当前节点的计数器
    pub fn local_event(&mut self) {
        let counter = self.counters.entry(self.node_id.clone()).or_insert(0);
        *counter += 1;
    }

    /// 发送消息前调用，返回当前的向量时钟副本用于附加到消息中
    pub fn prepare_message(&mut self) -> HashMap<String, i32> {
        self.local_event();
        self.counters.clone()
    }

    /// 接收消息后，合并发送方的向量时钟
    pub fn receive_message(&mut self, other_clocks: &HashMap<String, i32>) {
        for (node, &other_counter) in other_clocks {
            let self_counter = self.counters.entry(node.clone()).or_insert(0);
            *self_counter = (*self_counter).max(other_counter);
        }
        self.local_event();
    }

    /// 比较两个向量时钟的关系
    pub fn compare(&self, other: &VectorClock) -> ClockRelation {
        let self_gt_other = self
            .counters
            .iter()
            .all(|(node, &self_ctr)| self_ctr >= other.counters.get(node).copied().unwrap_or(0));
        let other_gt_self = other
            .counters
            .iter()
            .all(|(node, &other_ctr)| other_ctr >= self.counters.get(node).copied().unwrap_or(0));

        if self_gt_other && other_gt_self {
            ClockRelation::Equal
        } else if self_gt_other {
            ClockRelation::HappenedBefore
        } else if other_gt_self {
            ClockRelation::HappenedAfter
        } else {
            ClockRelation::Concurrent
        }
    }

    /// 获取当前的计数器副本（用于调试打印）
    pub fn counters(&self) -> &HashMap<String, i32> {
        &self.counters
    }

    /// 检测两个并发操作是否会产生冲突
    /// 返回 true 如果两个事件是并发的（可能冲突）
    pub fn may_conflict(&self, other: &VectorClock) -> bool {
        matches!(self.compare(other), ClockRelation::Concurrent)
    }

    /// 检测当前版本是否比给定版本更新
    /// 用于版本比较和数据同步
    pub fn is_newer_than(&self, other: &VectorClock) -> bool {
        matches!(self.compare(other), ClockRelation::HappenedBefore | ClockRelation::Equal)
    }

    /// 检测当前版本是否是给定版本的直接后继
    /// 用于检测中间操作或消息丢失
    pub fn is_direct_successor(&self, other: &VectorClock) -> bool {
        match self.compare(other) {
            ClockRelation::HappenedBefore => {
                // 检查是否只有一个节点的计数器增加了 1
                let mut diff_count = 0;
                for (node, &self_ctr) in &self.counters {
                    let other_ctr = other.counters.get(node).copied().unwrap_or(0);
                    let diff = self_ctr - other_ctr;
                    if diff > 1 {
                        return false; // 有节点跳跃超过 1
                    }
                    if diff == 1 {
                        diff_count += 1;
                    }
                }
                diff_count == 1 // 只有一个节点增加了 1
            }
            _ => false,
        }
    }

    /// 找出两个并发版本之间的差异
    /// 返回不同的节点列表
    pub fn concurrent_diff(&self, other: &VectorClock) -> Vec<String> {
        if !self.may_conflict(other) {
            return vec![];
        }

        let mut diffs = vec![];
        for (node, &self_ctr) in &self.counters {
            let other_ctr = other.counters.get(node).copied().unwrap_or(0);
            if self_ctr != other_ctr {
                diffs.push(node.clone());
            }
        }
        diffs
    }
}

/// 向量时钟关系枚举
#[derive(Debug, PartialEq)]
pub enum ClockRelation {
    HappenedBefore,
    HappenedAfter,
    Concurrent,
    Equal,
}
