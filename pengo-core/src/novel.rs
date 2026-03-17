use crate::error::PengoError;
use std::env;
use std::fs::{File, create_dir, write};
use std::path::Path;
use std::process::Command;

pub fn new_novel(novel_name: &str) -> Result<(), PengoError> {
    if Path::new(novel_name).exists() {
        return Err(PengoError::AlreadyExists(novel_name.to_string()));
    }

    create_dir(novel_name)?;

    let config_content = format!(
        r#"[book]
title = "{}"
author = ""
version = "0.1.0"

[build]
output_format = ["epub", "pdf"]

[editor]
command = "nvim"
auto_open = true

[status]
"#,
        novel_name
    );

    write(format!("{}/pengo.toml", novel_name), config_content)?;
    File::create(format!("{}/outline.md", novel_name))?;
    File::create(format!("{}/ideas.md", novel_name))?;
    create_dir(format!("{}/book", novel_name))?;
    create_dir(format!("{}/lore", novel_name))?;
    create_dir(format!("{}/lore/characters", novel_name))?;
    create_dir(format!("{}/lore/scenes", novel_name))?;
    create_dir(format!("{}/drafts", novel_name))?;

    let git_status = Command::new("git")
        .arg("init")
        .current_dir(novel_name)
        .status()
        .map_err(|e| {
            PengoError::GitError(format!(
                "無法啟動 Git (請確認是否已安裝 Git)。詳細錯誤: {}",
                e
            ))
        })?;

    if !git_status.success() {
        return Err(PengoError::GitError(format!(
            "Git 命令回傳了失敗的狀態碼: {}",
            git_status
        )));
    }

    // TODO: add .gitignore

    log::info!("novel '{}' create success!", novel_name);
    Ok(())
}
