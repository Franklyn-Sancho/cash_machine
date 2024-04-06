// deposit.rs

use crate::database::database::Database;
use crate::models::transaction::TransactionKind;
use crate::utils::read_input::read_input_and_check;
use crate::models::account::Account;


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

pub fn deposit_input(db: &Database, account: &mut Account) {
    while let Some(value) = read_input_and_check(
        "How much do you want to deposit (enter 0 to return to the home menu): ",
    ) {
        if let Ok(value) = value.parse::<f64>() {
            if let Err(err) = deposit(db, account, value) {
                println!("{}", err);
            }
        } else {
            println!("Invalid value, please try again")
        }
    }
}
