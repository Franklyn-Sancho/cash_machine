// deposit.rs
use crate::account::account::Account;
use crate::models::account_model::update_account;
use crate::database::database::Database;
use crate::utils::read_input::read_input;

// Função para realizar depósitos na conta
pub fn deposit(db: &Database, account: &mut Account) {
    loop {
        let value =
            read_input("Quando você deseja depositar (digite 0 para retornar ao menu inicial): ");
        if value == "0" {
            break;
        }
        if let Ok(value) = value.parse::<f64>() {
            account.deposit(db, value);
            update_account(db, account)
        }
        else {
            println!("Valid inválido, tente novamente")
        }
    }
}