use chrono::Utc;
use rusqlite::params;
use uuid::Uuid;

use crate::{database::database::Database, models::account_model::{TransactionKind, Transaction}};

// Definição da estrutura Account para representar uma conta bancária
#[derive(Debug)]
pub struct Account {
    pub id: String,
    pub balance: f64, // Saldo da conta
    pub coins: [f64; 6],  // Cédulas aceitas para depósito
}

impl Account {
    pub fn new(id: String, balance: f64) -> Self {
        Self {
            id,
            balance,
            coins: [2.00, 5.00, 10.00, 20.00, 50.00, 100.00], //valid coins
        }

    }

    pub fn create_account(db: &Database, user_id: &str) -> Account {
        let account_id = Uuid::new_v4().to_string();
        db.conn
            .execute(
                "INSERT INTO accounts (id, user_id, balance) VALUES (?1, ?2, 0)",
                params![account_id, user_id],
            )
            .unwrap();
        Account::new(account_id, 0.0)
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
    
    pub fn get_account_by_id(db: &Database, account_id: &str) -> Option<Account> {
        db.conn
            .query_row(
                "SELECT id, balance FROM accounts WHERE id = ?1",
                [account_id],
                |row| Ok(Account::new(row.get(0)?, row.get(1)?)),
            )
            .ok()
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

    pub fn is_valid_coin(&self, value: f64) -> bool {
        self.coins.contains(&value)
    }

    pub fn update_balance(
        db: &Database,
        account: &mut Account,
        value: f64,
        kind: TransactionKind,
        description: String,
    ) {
        account.balance += value;
        Self::update_account(db, account);
    
        let transaction_id = Uuid::new_v4().to_string();
        let transaction = Transaction {
            id: transaction_id,
            account_id: account.id.clone(),
            date: Utc::now(),
            value,
            kind,
            description,
        };
        Transaction::create_transaction(db, &transaction);
    }
}

