use chrono::{DateTime, TimeZone, Utc};
use rusqlite::{
    params,
    types::{FromSql, FromSqlError, FromSqlResult, ValueRef},
};

use chrono_tz::Tz;

use crate::{account::account::Account, database::database::Database};

#[derive(Debug)]
pub struct Transaction {
    pub account_id: i32,
    pub date: DateTime<Utc>,
    pub value: f64,
    pub kind: TransactionKind,
    pub description: String,
}

#[derive(Debug)]
pub enum TransactionKind {
    Deposit,
    Withdraw,
    Transfer,
}

impl FromSql for TransactionKind {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "Deposit" => Ok(TransactionKind::Deposit),
            "Withdrawal" => Ok(TransactionKind::Withdraw),
            "Transfer" => Ok(TransactionKind::Transfer),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

pub fn get_account_by_user(db: &Database, user_id: &str) -> Option<Account> {
    db.conn
        .query_row(
            "SELECT id, balance FROM accounts WHERE user_id = ?1",
            [user_id],
            |row| Ok(Account::new(row.get(0)?, row.get(1)?)),
        )
        .ok()
}

pub fn get_account_by_id(db: &Database, account_id: &i32) -> Option<Account> {
    db.conn
        .query_row(
            "SELECT id, balance FROM accounts WHERE id = ?1",
            [account_id],
            |row| Ok(Account::new(row.get(0)?, row.get(1)?)),
        )
        .ok()
}

pub fn create_account(db: &Database, user_id: &str) -> Account {
    db.conn
        .execute(
            "INSERT INTO accounts (user_id, balance) VALUES (?1, 0)",
            [user_id],
        )
        .unwrap();
    let account_id = db.conn.last_insert_rowid();
    Account::new(account_id as i32, 0.0)
}


pub fn update_account(db: &Database, account: &Account) {
    match db.conn.execute(
        "UPDATE accounts SET balance = ?1 WHERE id = ?2",
        params![account.balance, account.id],
    ) {
        Ok(_) => (),
        Err(e) => println!("Erro ao atualizar a conta: {}", e),
    }
}

pub fn create_transaction(db: &Database, transaction: &Transaction) {
    let formatted_date = transaction.date.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("Formatted date: {}", formatted_date);
    db.conn.execute(
        "INSERT INTO transactions (account_id, date, value, kind, description) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![transaction.account_id, formatted_date, transaction.value, format!("{:?}", transaction.kind), transaction.description],
    ).unwrap();
}


pub fn get_transactions(db: &Database, account_id: i32) -> Vec<String> {
    let tz: Tz = "America/Sao_Paulo".parse().unwrap();
    let mut stmt = db.conn.prepare("SELECT date, value, kind, description FROM transactions WHERE account_id = ?1").unwrap();
    let rows = stmt.query_map(params![account_id], |row| {
        let date: String = row.get(0).unwrap();
        let date = Utc.datetime_from_str(&date, "%Y-%m-%d %H:%M:%S").expect(&format!("Failed to parse date: {}", date));
        let date = date.with_timezone(&tz);
        let value: f64 = row.get(1).unwrap();
        let kind: String = row.get(2).unwrap();
        let description: String = row.get(3).unwrap();
        Ok(format!("{} | {} | {:.2} | {}", date.to_rfc3339(), kind, value, description))
    }).unwrap();

    rows.map(|row| row.unwrap()).collect()
}
 

