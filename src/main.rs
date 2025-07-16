mod models;
mod storage;
mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    data_file: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 添加新交易
    Add {
        amount: f64,
        category: String,
        #[arg(short, long)]
        income: bool,
    },
    /// 列出所有交易
    List,
    /// 显示财务摘要
    Summary,
    /// 进入交互模式
    Interactive,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let data_file = cli.data_file.unwrap_or_else(|| PathBuf::from("finance_data.json"));

    match &cli.command {
        Some(Commands::Add {
            amount,
            category,
            income,
        }) => {
            commands::add_transaction(&data_file, *amount, category, *income)
        }
        Some(Commands::List) => commands::list_transactions(&data_file),
        Some(Commands::Summary) => commands::show_summary(&data_file),
        Some(Commands::Interactive) => commands::interactive_mode(&data_file),
        None => commands::interactive_mode(&data_file),
    }
}