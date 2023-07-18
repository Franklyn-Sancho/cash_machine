use crate::{database::database::Database, utils::read_input::read_input};

use super::{account::Account, transactions::transfer};


pub fn transfer_input(db: &Database, account: &mut Account) {
    loop {
        let email = read_input("Insira o email do destinário (digite 0 para retornar ao menu principal): ");
        if email == "0" {
            break;
        }
        let value = read_input("Insira o valor a ser transferido (digite 0 para retornar ao menu principal): ");
        if value == "0" {
            break;
        }
        if let Ok(value) = value.parse::<f64>() {
            match transfer(db, &account.id, &email, value) {
                Ok(_) => println!("Você transferiu {:.2} para {}", value, email),
                Err(e) => println!("{}", e)
            }
        }
        else {
            println!("Valo inválido, tente novamente")
        }
    }
}