use crate::{models::{Transaction, TransactionType}, storage::{load_transactions, save_transactions}};
use anyhow::Result;
use chrono::Local;
use rustyline::Editor;
use std::collections::HashMap;
use std::path::Path;

pub fn add_transaction(
    file_path: &Path,
    amount: f64,
    category: &str,
    is_income: bool,
) -> Result<()> {
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

pub fn show_summary(file_path: &Path) -> Result<()> {
    let transactions = load_transactions(file_path)?;
    
    let mut income_by_category: HashMap<String, f64> = HashMap::new();
    let mut expense_by_category: HashMap<String, f64> = HashMap::new();
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

pub fn interactive_mode(file_path: &Path) -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    
    println!("个人财务追踪器 - 交互模式");
    println!("可用命令: add, list, summary, exit");
    println!("添加支出示例: add 15.99 餐饮");
    println!("添加收入示例: add income 5000 工资");
    
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
                    ["list"] => list_transactions(file_path)?,
                    ["summary"] => show_summary(file_path)?,
                    ["exit"] => break,
                    _ => println!("未知命令，可用命令: add, list, summary, exit"),
                }
            }
            Err(_) => break,
        }
    }
    
    Ok(())
}