// deposit.rs
use crate::account::Account;
use crate::authentication::read_input;
use crate::database::Database;

// Função para realizar depósitos na conta
pub fn deposit(db: &Database ,account: &mut Account) {
    loop {
        let value =
            read_input("Quando você deseja depositar (digite 0 para retornar ao menu inicial): ");
        if value == "0" {
            break;
        }
        let value: f64 = value.parse().unwrap();
        account.deposit(value);
        account.save(db)
    }
}