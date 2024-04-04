use chrono::Utc;
use uuid::Uuid;

use crate::{database::database::Database, models::account_model::{TransactionKind, update_account, Transaction, create_transaction}};

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
        update_account(db, account);
    
        let transaction_id = Uuid::new_v4().to_string();
        let transaction = Transaction {
            id: transaction_id,
            account_id: account.id.clone(),
            date: Utc::now(),
            value,
            kind,
            description,
        };
        create_transaction(db, &transaction);
    }
}

