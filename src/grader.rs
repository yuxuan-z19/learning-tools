use anyhow::{Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug)]
pub struct ExerciseResult {
    pub name: String,
    pub result: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Statistics {
    pub total_exercations: usize,
    pub total_succeeds: usize,
    pub total_failures: usize,
    pub total_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GradeResult {
    pub exercises: Vec<ExerciseResult>,
    pub statistics: Statistics,
}

/// 查找指定目录下的所有Rustlings练习文件
pub fn find_exercise_files(exercises_path: &Path) -> Result<Vec<PathBuf>> {
    let mut exercise_files = Vec::new();
    for entry in walkdir::WalkDir::new(exercises_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.components().any(|c| c.as_os_str() == "target") {
            continue;
        }
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            // 排除测试文件和辅助文件
            let file_name = path.file_name().unwrap().to_string_lossy();
            if !file_name.starts_with("test_") && !file_name.starts_with("helper_") {
                exercise_files.push(path.to_path_buf());
            }
        }
    }
    Ok(exercise_files)
}

/// 评测单个练习文件
pub async fn grade_exercise(exercise_path: &Path, verbose: bool) -> Result<(String, bool, u64)> {
    let start = Instant::now();
    let exercise_name = exercise_path
        .file_name()
        .context("无法获取文件名")?.
        to_string_lossy()
        .to_string();
    
    println!("{} {}", "评测练习:".blue().bold(), exercise_name);
    
    // 确保target目录存在
    fs::create_dir_all("target/debug").context("创建target目录失败")?;
    
    // 运行 rustc 编译练习
    let compile_output = Command::new("rustc")
        .arg(exercise_path)
        .arg("--test")
        .arg("-o")
        .arg(format!("target/debug/{}", exercise_name))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context(format!("编译练习 {} 失败", exercise_name))?;
    
    if !compile_output.status.success() {
        if verbose {
            println!("{}", String::from_utf8_lossy(&compile_output.stderr));
        }
        println!("{} {}", "编译失败:".red().bold(), exercise_name);
        return Ok((exercise_name, false, start.elapsed().as_secs()));
    }
    
    // 运行测试
    let test_output = Command::new(format!("target/debug/{}", exercise_name))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context(format!("运行练习 {} 失败", exercise_name))?;
    
    let success = test_output.status.success();
    
    if verbose || !success {
        println!("{}", String::from_utf8_lossy(&test_output.stdout));
        println!("{}", String::from_utf8_lossy(&test_output.stderr));
    }
    
    if success {
        println!("{} {}", "✓".green().bold(), exercise_name);
    } else {
        println!("{} {}", "✗".red().bold(), exercise_name);
    }
    
    Ok((exercise_name, success, start.elapsed().as_secs()))
}