use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: u32,
    pub amount: f64,
    pub category: String,
    pub date: DateTime<Local>,
    pub transaction_type: TransactionType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionType {
    Income,
    Expense,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransactionType::Income => write!(f, "Income"),
            TransactionType::Expense => write!(f, "Expense"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Budget {
    pub category: String,
    pub limit: f64,
    pub current: f64,
}