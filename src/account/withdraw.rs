// withdraw.rs
use crate::account::account::Account;
use crate::database::database::Database;
use crate::models::account_model::TransactionKind;
use crate::utils::read_input::read_input;

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

// Função para realizar saques na conta
pub fn withdraw_input(db: &Database, account: &mut Account) {
    if account.balance == 0.0 {
        println!("Insufficient funds");
        return;
    }

    loop {
        let value =
            read_input("Enter the withdrawal amount (enter 0 to return to the initial menu): ");
        if value == "0" {
            break;
        }
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
