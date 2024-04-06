// withdraw.rs
use crate::models::account::Account;
use crate::database::database::Database;
use crate::models::transaction::TransactionKind;
use crate::utils::read_input::read_input_and_check;

pub fn withdraw(db: &Database, account: &mut Account, value: f64) -> Result<(), String> {
    if account.balance < value {
        return Err("Insufficient funds".to_string());
    }

    Account::update_balance(
        db,
        account,
        -value,
        TransactionKind::Withdraw,
        format!("withdraw of {:.2}", value),
    );
    Ok(())
}

pub fn withdraw_input(db: &Database, account: &mut Account) {
    if account.balance == 0.0 {
        println!("Insufficient Funds");
        return;
    }

    while let Some(value) = read_input_and_check("Enter the withdrawal amount (enter 0 to return to the initial menu): ") {
        if let Ok(value) = value.parse::<f64>() {
            match withdraw(db, account, value) {
                Ok(_) => println!("Withdrawal successful!"),
                Err(e) => println!("{}", e),
            }
        } else {
            println!("Invalid value, try again")
        }
    }
}

