use chrono::{DateTime, TimeZone, Utc};
use rusqlite::{
    params,
    types::{FromSql, FromSqlError, FromSqlResult, Value, ValueRef},
};

use chrono_tz::Tz;

use crate::database::database::Database;

#[derive(Debug)]
pub struct Transaction {
    pub id: String,
    pub account_id: String,
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

impl Transaction {
    pub fn new(
        id: String,
        account_id: String,
        date: DateTime<Utc>,
        value: f64,
        kind: TransactionKind,
        description: String,
    ) -> Self {
        Self {
            id, account_id, date, value, kind, description
        }
    }

    pub fn create_transaction(db: &Database, transaction: &Transaction) {
        let formatted_date = transaction.date.format("%Y-%m-%d %H:%M:%S").to_string();
        db.conn.execute(
            "INSERT INTO transactions (id, account_id, date, value, kind, description) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![transaction.id, transaction.account_id, formatted_date, transaction.value, format!("{:?}", transaction.kind), transaction.description],
        ).unwrap();
    }

    pub fn get_transactions(db: &Database, account_id: &str) -> Vec<String> {
        let tz: Tz = "America/Sao_Paulo".parse().unwrap();
        let mut stmt = db
            .conn
            .prepare(
                "SELECT date, value, kind, description FROM transactions WHERE account_id = ?1",
            )
            .unwrap();
        let rows = stmt
            .query_map(params![account_id], |row| {
                let date: String = row.get(0).unwrap();
                let date = Utc
                    .datetime_from_str(&date, "%Y-%m-%d %H:%M:%S")
                    .expect(&format!("Failed to parse date: {}", date));
                let date = date.with_timezone(&tz);
                let value: f64 = row.get(1).unwrap();
                let kind: String = row.get(2).unwrap();
                let description: String = row.get(3).unwrap();
                Ok(format!(
                    "{} | {} | {:.2} | {}",
                    date.to_rfc3339(),
                    kind,
                    value,
                    description
                ))
            })
            .unwrap();

        rows.map(|row| row.unwrap()).collect()
    }
}
