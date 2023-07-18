// deposit.rs

use crate::models::account_model::TransactionKind;
use crate::database::database::Database;
use crate::utils::read_input::read_input;

use super::account::Account;
use super::transactions::update_balance;

// Função para realizar depósitos na conta
pub fn deposit(db: &Database, account: &mut Account) {
    loop {
        let value =
            read_input("Quando você deseja depositar (digite 0 para retornar ao menu inicial): ");
        if value == "0" {
            break;
        }
        if let Ok(value) = value.parse::<f64>() {
            update_balance(db, account, value, TransactionKind::Deposit, format!("Depósito de {:.2}", value));
        }
        else {
            println!("Valor inválido, tente novamente")
        }
    }
}
