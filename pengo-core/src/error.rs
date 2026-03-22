use thiserror::Error;

#[derive(Debug, Error)]
pub enum PengoError {
    #[error("'{0}' already exists")]
    AlreadyExists(String),

    #[error("Novel is not initialized")]
    NovelNotExists(),

    #[error("Volume '{0}' not found")]
    VolumeNotFound(String),

    #[error("Character '{0}' already exists")]
    CharacterExists(String),

    #[error("Characters directory not found")]
    CharactersNotFound(),

    #[error("Scene '{0}' already exists")]
    SceneExists(String),

    #[error("Scenes directory not found")]
    ScenesNotFound(),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Git initialization failed: {0}")]
    GitError(String),
}
