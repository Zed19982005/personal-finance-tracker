use crate::models::Transaction;
use anyhow::Result;
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub fn load_transactions(file_path: &Path) -> Result<Vec<Transaction>> {
    if !file_path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let transactions = serde_json::from_reader(reader)?;
    Ok(transactions)
}

pub fn save_transactions(file_path: &Path, transactions: &[Transaction]) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, transactions)?;
    Ok(())
}