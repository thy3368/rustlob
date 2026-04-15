/// 系统错误定义 (给运维/开发人员看)
#[derive(Debug, Clone)]
pub enum SystemError {
    Database(String),
    Network(String),
    Timeout(String),
}
