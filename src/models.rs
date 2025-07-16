use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;

/// 交易记录数据结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: u32,                   // 交易唯一ID
    pub amount: f64,               // 交易金额
    pub category: String,          // 交易类别
    pub date: DateTime<Local>,     // 交易日期时间
    pub transaction_type: TransactionType, // 交易类型
}

/// 交易类型枚举
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionType {
    Income,    // 收入
    Expense,   // 支出
}

/// 为交易类型实现Display trait
impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransactionType::Income => write!(f, "Income"),
            TransactionType::Expense => write!(f, "Expense"),
        }
    }
}

/// 预算数据结构（供扩展使用）
#[derive(Debug, Serialize, Deserialize)]
pub struct Budget {
    pub category: String,  // 预算类别
    pub limit: f64,        // 预算限额
    pub current: f64,     // 当前已使用金额
}
