mod models;
mod storage;
mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// 命令行参数解析
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    data_file: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

/// 支持的命令枚举
#[derive(Subcommand)]
enum Commands {
    /// 添加新交易
    Add {
        amount: f64,
        category: String,
        #[arg(short, long)]
        income: bool,
    },
    /// 删除交易
    Delete {
        id: u32,
    },
    /// 列出所有交易
    List,
    /// 显示财务摘要
    Summary,
    /// 进入交互模式
    Interactive,
}

/// 主函数
fn main() -> Result<()> {
    let cli = Cli::parse();
    let data_file = cli.data_file.unwrap_or_else(|| PathBuf::from("finance_data.json"));

    match &cli.command {
        Some(Commands::Add {
            amount,
            category,
            income,
        }) => commands::add_transaction(&data_file, *amount, category, *income),
        Some(Commands::Delete { id }) => commands::delete_transaction(&data_file, *id),
        Some(Commands::List) => commands::list_transactions(&data_file),
        Some(Commands::Summary) => commands::show_summary(&data_file),
        Some(Commands::Interactive) => commands::interactive_mode(&data_file),
        None => commands::interactive_mode(&data_file),
    }
}s
