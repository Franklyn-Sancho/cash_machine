// deposit.rs

use crate::models::account_model::TransactionKind;
use crate::database::database::Database;
use crate::utils::read_input::read_input;

use super::account::Account;

pub fn deposit(db: &Database, account: &mut Account, value: f64) -> Result<(), String> {
    if !account.is_valid_coin(value) {
        return Err(format!("accepted values: {:?}", account.coins));
    }

    Account::update_balance(
        db,
        account,
        value,
        TransactionKind::Deposit,
        format!("deposited amount {:.2}", value),
    );
    Ok(())
}

// Função para realizar depósitos na conta
pub fn deposit_input(db: &Database, account: &mut Account) {
    loop {
        let value =
            read_input("How much do you want to deposit (enter 0 to return to the home menu): ");
        if value == "0" {
            break;
        }
        if let Ok(value) = value.parse::<f64>() {
            Account::update_balance(db, account, value, TransactionKind::Deposit, format!("Depósito de {:.2}", value));
        }
        else {
            println!("Invalid value, please try again")
        }
    }
}
