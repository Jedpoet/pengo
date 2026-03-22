use thiserror::Error;

#[derive(Debug, Error)]
pub enum PengoError {
    #[error("'{0}' 已經存在")]
    AlreadyExists(String),

    #[error("'{0}' 尚未初始化")]
    NovelNotExists(String),

    #[error("'{0}' 卷不存在")]
    VolumeNotFound(String),

    #[error("'{0}' 已經存在")]
    CharacterExists(String),

    #[error("角色資料夾不存在")]
    CharactersNotFound(),

    #[error("'{0}' 已經存在")]
    SceneExists(String),

    #[error("場景資料夾不存在")]
    ScenesNotFound(),

    #[error("IO 錯誤：{0}")]
    Io(#[from] std::io::Error),

    #[error("Git 初始化失敗: {0}")]
    GitError(String),
}
