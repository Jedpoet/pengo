use crate::error::PengoError;
use regex::Regex;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn new_novel(novel_name: &str) -> Result<(), PengoError> {
    if Path::new(novel_name).exists() {
        return Err(PengoError::AlreadyExists(novel_name.to_string()));
    }

    fs::create_dir(novel_name)?;

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
"#,
        novel_name
    );

    fs::write(format!("{}/pengo.toml", novel_name), config_content)?;
    fs::File::create(format!("{}/outline.md", novel_name))?;
    fs::File::create(format!("{}/ideas.md", novel_name))?;
    fs::create_dir(format!("{}/book", novel_name))?;
    fs::create_dir(format!("{}/lore", novel_name))?;
    fs::create_dir(format!("{}/lore/characters", novel_name))?;
    fs::create_dir(format!("{}/lore/scenes", novel_name))?;
    fs::create_dir(format!("{}/drafts", novel_name))?;

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

pub fn chapter_new(volume: Option<String>, chapter_name: Option<String>) -> Result<(), PengoError> {
    if !Path::new("pengo.toml").exists() {
        return Err(PengoError::NovelNotExists("找不到小說！".to_string()));
    }
    let volume = volume.unwrap_or("volume1".to_string());
    let volume_dir = Path::new("book").join(volume);
    if !volume_dir.exists() {
        fs::create_dir_all(&volume_dir)?;
    }
    let re = Regex::new(r"^(\d{3,})-.*\.md$").unwrap();
    let dir = fs::read_dir(&volume_dir)?;
    let mut max_num = 0;
    for entry in dir.flatten() {
        let filename = entry.file_name();
        let filename_str = filename.to_string_lossy();

        if let Some(caps) = re.captures(&filename_str) {
            let chapter_num: u32 = caps[1].parse().unwrap_or(0);
            log::info!("匹配成功！章節編號：{}", chapter_num);
            if chapter_num > max_num {
                max_num = chapter_num;
            }
        }
    }
    let next_num = max_num + 1;
    let new_filename = match chapter_name {
        Some(name) => format!("{:03}-{}.md", next_num, name),
        None => format!("{:03}.md", next_num),
    };
    let new_filepath = volume_dir.join(&new_filename);

    fs::File::create(&new_filepath)?;
    log::info!("成功建立新章節：{}", new_filepath.display());

    Ok(())
}

pub fn chapter_ls(volume: Option<String>) -> Result<Vec<String>, PengoError> {
    let volume = volume.unwrap_or("volume1".to_string());
    let volume_dir = Path::new("book").join(&volume);
    if !volume_dir.exists() {
        return Err(PengoError::VolumeNotFound(volume));
    }
    let mut chls_with_num: Vec<(u32, String)> = Vec::new();

    let re = Regex::new(r"^(\d{3,})-.*\.md$").unwrap();

    let dir = fs::read_dir(&volume_dir)?;
    for entry in dir.flatten() {
        let filename = entry.file_name();
        let filename_str = filename.to_string_lossy();
        if let Some(caps) = re.captures(&filename_str) {
            let num: u32 = caps[1].parse().unwrap_or(0);
            chls_with_num.push((num, filename_str.into_owned()));
        }
    }
    chls_with_num.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let chls: Vec<String> = chls_with_num.into_iter().map(|(_, name)| name).collect();
    Ok(chls)
}

pub fn idea_add(idea: &str) -> Result<(), PengoError> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true) // Create the file if it doesn't exist
        .open("idea.md")?;

    writeln!(file, "* {}", idea)?;
    Ok(())
}
