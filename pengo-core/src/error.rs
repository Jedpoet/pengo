// pengo-core/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PengoError {
    #[error("'{0}' 已經存在")]
    AlreadyExists(String),

    #[error("IO 錯誤：{0}")]
    Io(#[from] std::io::Error),

    #[error("Git 初始化失敗: {0}")]
    GitError(String),
}
