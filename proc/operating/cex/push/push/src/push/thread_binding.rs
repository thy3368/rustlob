use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

use actix::prelude::*;
use core_affinity::{CoreId, get_core_ids, set_for_current};
use num_cpus;

// 线程绑定错误类型
#[derive(Debug, thiserror::Error)]
pub enum ThreadBindingError {
    #[error("无效的线程索引: {0}")]
    InvalidThreadIndex(usize),

    #[error("线程绑定失败: {0}")]
    BindingFailed(String),

    #[error("CPU亲和性设置失败: {0}")]
    CpuAffinityFailed(usize),

    #[error("线程迁移失败: {0}")]
    MigrationFailed(String),

    #[error("无法查询线程信息")]
    CannotQueryThread,

    #[error("线程不匹配: 期望 {expected}, 实际 {actual}")]
    ThreadMismatch { expected: usize, actual: usize },

    #[error("多个验证错误: {0:?}")]
    Multiple(Vec<Self>),
}

// 绑定规则
#[derive(Debug, Clone)]
pub enum BindingRule {
    // 固定线程
    Fixed(usize),

    // 负载均衡
    LoadBalanced,

    // 亲和性绑定
    Affinity(ThreadAffinity),

    // 排除特定线程
    Exclude(Vec<usize>),

    // 在指定范围内选择
    Range(std::ops::Range<usize>),
}

// 线程亲和性配置
#[derive(Debug, Clone)]
pub struct ThreadAffinity {
    // 首选线程
    pub preferred_threads: Vec<usize>,

    // 备选线程
    pub fallback_threads: Vec<usize>,

    // 是否严格绑定
    pub strict: bool,
}

impl ThreadAffinity {
    pub fn new(preferred_threads: Vec<usize>) -> Self {
        Self { preferred_threads, fallback_threads: Vec::new(), strict: true }
    }

    pub fn with_fallback(mut self, fallback_threads: Vec<usize>) -> Self {
        self.fallback_threads = fallback_threads;
        self
    }

    pub fn with_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    pub fn select_thread(&self) -> usize {
        // 尝试首选线程
        for &thread in &self.preferred_threads {
            if self.is_thread_available(thread) {
                return thread;
            }
        }

        // 尝试备选线程
        for &thread in &self.fallback_threads {
            if self.is_thread_available(thread) {
                return thread;
            }
        }

        // 都没有，选择第一个首选线程
        self.preferred_threads[0]
    }

    fn is_thread_available(&self, thread_id: usize) -> bool {
        // 简单的可用性检查，实际项目中应该检查线程状态
        true
    }
}

// 默认策略
#[derive(Debug, Clone)]
pub enum DefaultPolicy {
    RoundRobin,
    Random,
    LoadBalanced,
}

impl Default for DefaultPolicy {
    fn default() -> Self {
        Self::RoundRobin
    }
}

// 线程绑定策略
#[derive(Debug, Clone)]
pub struct ThreadBindingPolicy {
    // 基于 Actor 类型的绑定
    type_bindings: HashMap<std::any::TypeId, BindingRule>,

    // 基于消息类型的绑定
    message_bindings: HashMap<std::any::TypeId, BindingRule>,

    // 基于负载的绑定
    // 注意：LoadBasedBinding 现在没有 Clone trait，所以我们使用 Arc 来共享它
    load_based_bindings: Arc<LoadBasedBinding>,

    // 默认策略
    default_policy: DefaultPolicy,
}

impl ThreadBindingPolicy {
    pub fn new() -> Self {
        Self {
            type_bindings: HashMap::new(),
            message_bindings: HashMap::new(),
            load_based_bindings: Arc::new(LoadBasedBinding::new()),
            default_policy: DefaultPolicy::default(),
        }
    }

    // 为 Actor 类型设置绑定规则
    pub fn bind_actor_type<A: 'static>(&mut self, rule: BindingRule) {
        let type_id = std::any::TypeId::of::<A>();
        self.type_bindings.insert(type_id, rule);
    }

    // 为消息类型设置绑定规则
    pub fn bind_message_type<M: actix::Message + 'static>(&mut self, rule: BindingRule) {
        let type_id = std::any::TypeId::of::<M>();
        self.message_bindings.insert(type_id, rule);
    }

    // 获取 Actor 应该运行的线程
    pub fn get_thread_for_actor<A: 'static>(&self) -> Option<usize> {
        let type_id = std::any::TypeId::of::<A>();

        if let Some(rule) = self.type_bindings.get(&type_id) {
            match rule {
                BindingRule::Fixed(thread_id) => Some(*thread_id),
                BindingRule::LoadBalanced => {
                    // 选择负载最轻的线程
                    Some(self.load_based_bindings.select_thread())
                }
                BindingRule::Affinity(affinity) => {
                    // 根据亲和性选择线程
                    Some(affinity.select_thread())
                }
                BindingRule::Exclude(excluded) => {
                    // 选择不在排除列表中的线程
                    self.get_available_thread(excluded)
                }
                BindingRule::Range(range) => {
                    // 在范围内选择线程
                    Some(self.select_in_range(range.clone()))
                }
            }
        } else {
            // 使用默认策略
            match self.default_policy {
                DefaultPolicy::RoundRobin => {
                    static COUNTER: AtomicUsize = AtomicUsize::new(0);
                    Some(COUNTER.fetch_add(1, Ordering::Relaxed) % num_cpus::get())
                }
                DefaultPolicy::Random => {
                    use rand::Rng;
                    Some(rand::thread_rng().gen_range(0..num_cpus::get()))
                }
                DefaultPolicy::LoadBalanced => Some(self.load_based_bindings.select_thread()),
            }
        }
    }

    fn get_available_thread(&self, excluded: &[usize]) -> Option<usize> {
        for i in 0..num_cpus::get() {
            if !excluded.contains(&i) {
                return Some(i);
            }
        }
        None
    }

    fn select_in_range(&self, range: std::ops::Range<usize>) -> usize {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
        range.start + (counter % (range.end - range.start))
    }
}

// 负载均衡绑定
#[derive(Debug)]
pub struct LoadBasedBinding {
    thread_loads: Vec<AtomicUsize>,
}

impl LoadBasedBinding {
    pub fn new() -> Self {
        let cpu_count = num_cpus::get();
        let mut thread_loads = Vec::with_capacity(cpu_count);
        for _ in 0..cpu_count {
            thread_loads.push(AtomicUsize::new(0));
        }
        Self { thread_loads }
    }

    pub fn record_load(&self, thread_id: usize, load: usize) {
        if thread_id < self.thread_loads.len() {
            self.thread_loads[thread_id].fetch_add(load, Ordering::Relaxed);
        }
    }

    pub fn select_thread(&self) -> usize {
        let mut min_load = usize::MAX;
        let mut selected_thread = 0;

        for (i, load) in self.thread_loads.iter().enumerate() {
            let current_load = load.load(Ordering::Relaxed);
            if current_load < min_load {
                min_load = current_load;
                selected_thread = i;
            }
        }

        selected_thread
    }
}

// 线程绑定管理器
pub struct ThreadBindingManager {
    // 线程池
    thread_pool: ThreadPool,

    // 绑定策略
    binding_policy: Arc<RwLock<ThreadBindingPolicy>>,

    // 绑定验证器
    binding_validator: BindingValidator,
}

impl ThreadBindingManager {
    pub fn new(thread_count: usize) -> Self {
        let thread_pool = ThreadPool::new(thread_count);
        let binding_policy = Arc::new(RwLock::new(ThreadBindingPolicy::new()));
        let binding_validator = BindingValidator::new();

        Self { thread_pool, binding_policy, binding_validator }
    }

    // 启动 Actor 并绑定到线程
    pub fn start_actor_with_binding<A>(&self, actor: A) -> Result<Addr<A>, ThreadBindingError>
    where
        A: Actor<Context = Context<A>> + 'static + Send,
    {
        // 获取绑定的线程
        let policy = self.binding_policy.read().unwrap();
        let thread_id = policy.get_thread_for_actor::<A>();

        drop(policy);

        if let Some(thread_id) = thread_id {
            // 在指定线程启动 Actor
            let addr = self.thread_pool.spawn_on_thread(thread_id, actor)?;

            // 验证绑定
            self.binding_validator.validate_binding(&addr, thread_id)?;

            Ok(addr)
        } else {
            // 没有特定绑定，在任意线程启动
            let addr = actor.start();
            Ok(addr)
        }
    }
}

// 线程池
struct ThreadPool {
    arbiters: Vec<Arbiter>,
    cpu_affinity: Vec<Option<CoreId>>,
}

impl ThreadPool {
    pub fn new(thread_count: usize) -> Self {
        let core_ids = get_core_ids().unwrap_or_default();
        let mut arbiters = Vec::with_capacity(thread_count);
        let mut cpu_affinity = Vec::with_capacity(thread_count);

        for i in 0..thread_count {
            let arbiter = Arbiter::new();

            // 尝试为每个线程设置 CPU 亲和性
            let core_id = if core_ids.len() > i { Some(core_ids[i]) } else { None };

            // 在新创建的线程上设置 CPU 亲和性
            let core_id_clone = core_id.clone();
            arbiter.spawn(async move {
                if let Some(core_id) = core_id_clone {
                    let _ = set_for_current(core_id);
                }
            });

            arbiters.push(arbiter);
            cpu_affinity.push(core_id);
        }

        Self { arbiters, cpu_affinity }
    }

    pub fn spawn_on_thread<A>(
        &self,
        thread_id: usize,
        actor: A,
    ) -> Result<Addr<A>, ThreadBindingError>
    where
        A: Actor<Context = Context<A>> + 'static + Send,
    {
        if thread_id < self.arbiters.len() {
            let arbiter = &self.arbiters[thread_id];

            let (tx, rx) = std::sync::mpsc::sync_channel(1);

            arbiter.spawn(async move {
                let addr = actor.start();
                tx.send(addr).unwrap();
            });

            let addr = rx.recv().unwrap();
            Ok(addr)
        } else {
            Err(ThreadBindingError::InvalidThreadIndex(thread_id))
        }
    }

    pub fn cpu_affinity(&self) -> &[Option<CoreId>] {
        &self.cpu_affinity
    }
}

// 线程监控
struct ThreadMonitor {
    thread_count: usize,
    thread_loads: Arc<Mutex<Vec<f32>>>,
    monitor_handle: Option<thread::JoinHandle<()>>,
    stop_flag: Arc<std::sync::atomic::AtomicBool>,
}

impl ThreadMonitor {
    pub fn new(thread_count: usize) -> Self {
        let thread_loads = Arc::new(Mutex::new(vec![0.0; thread_count]));
        let stop_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));

        Self { thread_count, thread_loads, monitor_handle: None, stop_flag }
    }
}

// 绑定验证器
struct BindingValidator {
    validation_rules: Vec<ValidationRule>,
    validation_results: Arc<Mutex<Vec<ValidationResult>>>,
    validator_state: Arc<std::sync::atomic::AtomicBool>,
}

impl BindingValidator {
    pub fn new() -> Self {
        Self {
            validation_rules: vec![ValidationRule::ThreadConsistency, ValidationRule::CpuAffinity],
            validation_results: Arc::new(Mutex::new(Vec::new())),
            validator_state: Arc::new(std::sync::atomic::AtomicBool::new(true)),
        }
    }

    pub fn validate_binding<A: Actor>(
        &self,
        addr: &Addr<A>,
        expected_thread: usize,
    ) -> Result<(), ThreadBindingError> {
        let mut errors = Vec::new();

        for rule in &self.validation_rules {
            match self.check_rule(rule, addr, expected_thread) {
                Ok(_) => {}
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        if errors.is_empty() { Ok(()) } else { Err(ThreadBindingError::Multiple(errors)) }
    }

    fn check_rule(
        &self,
        rule: &ValidationRule,
        addr: &Addr<impl Actor>,
        expected_thread: usize,
    ) -> Result<(), ThreadBindingError> {
        match rule {
            ValidationRule::ThreadConsistency => {
                self.check_thread_consistency(addr, expected_thread)
            }
            ValidationRule::CpuAffinity => self.check_cpu_affinity(addr),
        }
    }

    fn check_thread_consistency(
        &self,
        addr: &Addr<impl Actor>,
        expected_thread: usize,
    ) -> Result<(), ThreadBindingError> {
        // 简单实现，实际项目中应该向 Actor 发送消息查询其所在线程
        Ok(())
    }

    fn check_cpu_affinity(&self, addr: &Addr<impl Actor>) -> Result<(), ThreadBindingError> {
        // 简单实现，实际项目中应该验证 CPU 亲和性
        Ok(())
    }
}

// 验证规则
#[derive(Debug, Clone)]
enum ValidationRule {
    ThreadConsistency,
    CpuAffinity,
}

// 验证结果
#[derive(Debug, Clone)]
struct ValidationResult {
    actor_type: String,
    thread_id: usize,
    cpu_affinity: Option<usize>,
    timestamp: std::time::Instant,
}

// 线程绑定的 Actor 系统
pub struct ThreadBoundActorSystem {
    // 线程绑定管理器
    binding_manager: Arc<ThreadBindingManager>,

    // 系统配置
    config: SystemConfig,
}

impl ThreadBoundActorSystem {
    pub fn new(config: SystemConfig) -> Self {
        let thread_count = config.thread_count;

        let binding_manager = Arc::new(ThreadBindingManager::new(thread_count));

        Self { binding_manager, config }
    }

    // 启动系统
    pub async fn start(&self) -> Result<(), ThreadBindingError> {
        tracing::info!("🚀 启动线程绑定的 Actor 系统");

        Ok(())
    }

    // 创建 Actor
    pub async fn create_actor<A>(&self, actor: A) -> Result<Addr<A>, ThreadBindingError>
    where
        A: Actor<Context = Context<A>> + 'static + Send,
    {
        let addr = self.binding_manager.start_actor_with_binding(actor)?;

        Ok(addr)
    }
}

// 系统配置
#[derive(Debug, Clone)]
pub struct SystemConfig {
    pub thread_count: usize,
    pub binding_rules: Option<BindingRules>,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self { thread_count: num_cpus::get(), binding_rules: None }
    }
}

// 绑定规则配置
#[derive(Debug, Clone)]
pub struct BindingRules {
    rules: Vec<(std::any::TypeId, BindingRule)>,
}

impl BindingRules {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn bind_actor<A: 'static>(mut self, rule: BindingRule) -> Self {
        let type_id = std::any::TypeId::of::<A>();
        self.rules.push((type_id, rule));
        self
    }

    pub fn bind_message<M: actix::Message + 'static>(mut self, rule: BindingRule) -> Self {
        let type_id = std::any::TypeId::of::<M>();
        self.rules.push((type_id, rule));
        self
    }
}

// CPU 亲和性 Actor 基类
pub struct CpuAffinityActor {
    core_id: Option<CoreId>,
    pinned: std::sync::atomic::AtomicBool,
}

impl CpuAffinityActor {
    pub fn with_affinity(core_id: CoreId) -> Self {
        Self { core_id: Some(core_id), pinned: std::sync::atomic::AtomicBool::new(false) }
    }

    // 设置 CPU 亲和性
    fn set_cpu_affinity(&self) -> Result<(), ThreadBindingError> {
        if let Some(core_id) = self.core_id {
            #[cfg(target_os = "linux")]
            {
                if set_for_current(core_id) {
                    self.pinned.store(true, Ordering::SeqCst);
                    tracing::info!("Actor 绑定到 CPU 核心: {}", core_id.id);
                    Ok(())
                } else {
                    Err(ThreadBindingError::CpuAffinityFailed(core_id.id))
                }
            }

            #[cfg(not(target_os = "linux"))]
            {
                tracing::warn!("CPU 亲和性仅在 Linux 上支持");
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

impl Actor for CpuAffinityActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // 设置 CPU 亲和性
        if let Err(e) = self.set_cpu_affinity() {
            tracing::error!("设置 CPU 亲和性失败: {:?}", e);
        }

        // 验证绑定
        if self.pinned.load(Ordering::SeqCst) {
            tracing::info!("✅ Actor 成功绑定到指定 CPU 核心");
        }
    }
}
