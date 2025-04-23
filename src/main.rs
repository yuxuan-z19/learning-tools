use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::PathBuf;

mod grader;
use grader::{find_exercise_files, grade_exercise, GradeResult};

#[derive(Parser)]
#[command(author, version, about = "Rustlings本地评测工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 评测所有Rustlings练习
    Grade {
        /// Rustlings练习目录路径
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        
        /// 是否显示详细输出
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// 评测单个Rustlings练习
    GradeSingle {
        /// 练习文件路径
        #[arg(short, long)]
        file: PathBuf,
        
        /// 是否显示详细输出
        #[arg(short, long)]
        verbose: bool,
    },
}

async fn grade_all_exercises(exercises_path: &PathBuf, verbose: bool) -> Result<GradeResult> {
    // 查找所有.rs文件
    let exercise_files = find_exercise_files(exercises_path)?;
    
    println!("{} {} {}", "找到".blue().bold(), exercise_files.len(), "个练习文件".blue().bold());
    
    // 创建进度条
    let total_exercises = exercise_files.len() as u64;
    let bar = ProgressBar::new(total_exercises);
    bar.set_style(
        ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("##-"),
    );
    
    let mut exercise_results = Vec::new();
    let mut total_succeeds = 0;
    let mut total_failures = 0;
    let mut total_time = 0;
    
    for exercise_path in exercise_files.iter() {
        bar.inc(1); // Increment position before processing
        
        let (name, result, time) = grade_exercise(exercise_path, verbose).await?;
        
        if result {
            total_succeeds += 1;
        } else {
            total_failures += 1;
        }
        
        total_time += time;
        
        exercise_results.push(grader::ExerciseResult {
            name,
            result,
        });
    }
    
    bar.finish_with_message("评测完成!");
    // println!(); // finish_with_message usually handles the final state
    
    Ok(GradeResult {
        exercises: exercise_results,
        statistics: grader::Statistics {
            total_exercations: exercise_files.len(),
            total_succeeds,
            total_failures,
            total_time,
        },
    })
}

async fn grade_single_exercise(exercise_path: &PathBuf, verbose: bool) -> Result<GradeResult> {
    let (name, result, time) = grade_exercise(exercise_path, verbose).await?;
    
    let total_succeeds = if result { 1 } else { 0 };
    let total_failures = if result { 0 } else { 1 };
    
    Ok(GradeResult {
        exercises: vec![grader::ExerciseResult { name, result }],
        statistics: grader::Statistics {
            total_exercations: 1,
            total_succeeds,
            total_failures,
            total_time: time,
        },
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let result = match &cli.command {
        Commands::Grade { path, verbose } => {
            println!("{}", "开始评测所有Rustlings练习...".blue().bold());
            grade_all_exercises(path, *verbose).await?
        },
        Commands::GradeSingle { file, verbose } => {
            println!("{}", "开始评测单个Rustlings练习...".blue().bold());
            grade_single_exercise(file, *verbose).await?
        },
    };
    
    // 打印统计信息
    println!("{}", "评测结果统计".green().bold());
    println!("{}: {}", "总练习数".blue(), result.statistics.total_exercations);
    println!("{}: {}", "通过数量".green(), result.statistics.total_succeeds);
    println!("{}: {}", "失败数量".red(), result.statistics.total_failures);
    println!("{}: {}秒", "总耗时".blue(), result.statistics.total_time);
    
    // 计算通过率
    let pass_rate = if result.statistics.total_exercations > 0 {
        (result.statistics.total_succeeds as f32 / result.statistics.total_exercations as f32) * 100.0
    } else {
        0.0
    };
    println!("{}: {:.2}%", "通过率".green(), pass_rate);
    
    // 将结果保存到文件
    let json_result = serde_json::to_string_pretty(&result)?;
    fs::write("rustlings_result.json", json_result)?;
    println!("{}", "评测结果已保存到 rustlings_result.json".blue());
    
    Ok(())
}