use crate::{
    models::{Transaction, TransactionType},
    storage::{load_transactions, save_transactions},
};
use anyhow::Result;
use chrono::Local;
use rustyline::Editor;
use std::collections::HashMap;
use std::path::Path;

/// 添加新交易记录
pub fn add_transaction(file_path: &Path, amount: f64, category: &str, is_income: bool) -> Result<()> {
    let mut transactions = load_transactions(file_path)?;
    
    let new_id = transactions.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    
    let transaction = Transaction {
        id: new_id,
        amount,
        category: category.to_string(),
        date: Local::now(),
        transaction_type: if is_income {
            TransactionType::Income
        } else {
            TransactionType::Expense
        },
    };
    
    transactions.push(transaction);
    save_transactions(file_path, &transactions)?;
    println!("交易添加成功!");
    Ok(())
}

/// 删除指定ID的交易记录
pub fn delete_transaction(file_path: &Path, id: u32) -> Result<()> {
    let mut transactions = load_transactions(file_path)?;
    let original_len = transactions.len();
    
    transactions.retain(|t| t.id != id);
    
    if transactions.len() == original_len {
        println!("未找到ID为 {} 的交易记录", id);
    } else {
        save_transactions(file_path, &transactions)?;
        println!("成功删除ID为 {} 的交易记录", id);
    }
    
    Ok(())
}

/// 列出所有交易记录
pub fn list_transactions(file_path: &Path) -> Result<()> {
    let transactions = load_transactions(file_path)?;
    
    println!("ID\t类型\t金额\t类别\t日期");
    println!("--------------------------------------------");
    for t in transactions {
        println!(
            "{}\t{}\t{:.2}\t{}\t{}",
            t.id,
            t.transaction_type,
            t.amount,
            t.category,
            t.date.format("%Y-%m-%d %H:%M:%S")
        );
    }
    Ok(())
}

/// 显示财务摘要统计
pub fn show_summary(file_path: &Path) -> Result<()> {
    let transactions = load_transactions(file_path)?;
    
    let mut income_by_category = HashMap::new();
    let mut expense_by_category = HashMap::new();
    let mut total_income = 0.0;
    let mut total_expense = 0.0;
    
    for t in transactions {
        match t.transaction_type {
            TransactionType::Income => {
                *income_by_category.entry(t.category.clone()).or_insert(0.0) += t.amount;
                total_income += t.amount;
            }
            TransactionType::Expense => {
                *expense_by_category.entry(t.category.clone()).or_insert(0.0) += t.amount;
                total_expense += t.amount;
            }
        }
    }
    
    println!("=== 财务摘要 ===");
    println!("总收入: {:.2}", total_income);
    println!("总支出: {:.2}", total_expense);
    println!("当前余额: {:.2}", total_income - total_expense);
    
    println!("\n收入分类统计:");
    for (category, amount) in income_by_category {
        println!("- {}: {:.2}", category, amount);
    }
    
    println!("\n支出分类统计:");
    for (category, amount) in expense_by_category {
        println!("- {}: {:.2}", category, amount);
    }
    
    Ok(())
}

/// 交互式命令行模式
pub fn interactive_mode(file_path: &Path) -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    
    println!("个人财务追踪器 - 交互模式");
    println!("可用命令: add, delete, list, summary, exit");
    println!("添加支出示例: add 15.99 餐饮");
    println!("添加收入示例: add income 5000 工资");
    println!("删除记录示例: delete 3");
    
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                match parts.as_slice() {
                    ["add", amount, category] => {
                        if let Ok(amount) = amount.parse() {
                            add_transaction(file_path, amount, category, false)?;
                        } else {
                            println!("金额必须为数字");
                        }
                    }
                    ["add", "income", amount, category] => {
                        if let Ok(amount) = amount.parse() {
                            add_transaction(file_path, amount, category, true)?;
                        } else {
                            println!("金额必须为数字");
                        }
                    }
                    ["delete", id] => {
                        if let Ok(id) = id.parse() {
                            delete_transaction(file_path, id)?;
                        } else {
                            println!("ID必须为数字");
                        }
                    }
                    ["list"] => list_transactions(file_path)?,
                    ["summary"] => show_summary(file_path)?,
                    ["exit"] => break,
                    _ => println!("未知命令，可用命令: add, delete, list, summary, exit"),
                }
            }
            Err(_) => break,
        }
    }
    
    Ok(())
}
