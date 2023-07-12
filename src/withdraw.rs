use rusqlite::Connection;

// withdraw.rs
use crate::account::Account;
use crate::auth::read_input;

// Função para realizar saques na conta
pub fn withdraw(conn: &Connection ,account: &mut Account) {
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
        let value: f64 = value.parse().unwrap();
        account.withdraw(value);
        account.save(conn)
    }
}