use crate::{
    account::Account,
    auth::{authenticate, read_input, register},
    deposit, withdraw, model::get_account,
};
use rusqlite::Connection;

//menu inicial para login e autenticação
pub fn login_register_menu(conn: &Connection) {
    loop {
        println!("Escolha a sua opção: ");
        println!("1 - Login");
        println!("2 - Registrar");
        println!("3 - Sair");
        print!("Insira a sua opção aqui: ");

        let option = read_input("");
        match option.parse().unwrap() {
            1 => {
                if let Some(user_email) = authenticate(conn) {
                    let mut account = get_account(conn, &user_email);
                    transaction_menu(conn, &mut account);
                    break;
                } else {
                    println!("Nome de usuário ou senha inválidos");
                }
            }
            2 => {
                register(conn);
            }
            3 => break,
            _ => println!("Opção inválida"),
        }
    }
}

//menu do sistema bancário (saque e deposito)
fn transaction_menu(conn: &Connection, account: &mut Account) {
    loop {
        println!("Escolha a sua opção: ");
        println!("1 - Depositar");
        println!("2 - Sacar");
        println!("3 - Sair");
        print!("Insira a sua opção aqui: ");

        let option = read_input("");
        match option.parse().unwrap() {
            1 => deposit(conn, account),
            2 => withdraw(conn, account),
            3 => break,
            _ => println!("Opção inválida"),
        }
    }
}
