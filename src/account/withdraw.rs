// withdraw.rs
use crate::account::account::Account;
use crate::models::account_model::update_account;
use crate::database::database::Database;
use crate::utils::read_input::read_input;

// Função para realizar saques na conta
pub fn withdraw(db: &Database, account: &mut Account) {
    if account.balance == 0.0 {
        println!("Saldo insuficiente");
        return;
    }

    loop {
        let value = read_input(
            "Digite quanto você deseja sacar (digite 0 para retornar ao menu inicial): ",
        );
        if value == "0" {
            break;
        }
        if let Ok(value) = value.parse::<f64>() {
            account.withdraw(db, value);
            update_account(db, account)
        }
        else {
            println!("Valid inválido, tente novamente")
        }
    }
}
