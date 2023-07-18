use crate::{
    account::account::Account,
    models::account_model::{create_account, get_transactions, get_account_by_user},
    database::database::Database,
    utils::read_input::read_input,
    authentication::authentication::register,
    authentication::authentication::authenticate,
    account::{transfer::transfer_input, withdraw::withdraw_input, deposit::deposit_input},
};

pub fn login_register_menu(db: &Database) {
    loop {
        println!("Escolha a sua opção: ");
        println!("1 - Login");
        println!("2 - Registrar");
        println!("3 - Sair");
        print!("Insira a sua opção aqui: ");

        let option = read_input("");
        if let Ok(option) = option.parse() {
            match option {
                1 => {
                    if let Some(user) = authenticate(db) {
                        let mut account = get_account_by_user(db, &user.id)
                            .unwrap_or_else(|| create_account(db, &user.id));
                        transaction_menu(db, &mut account);
                        break;
                    } else {
                        println!("Nome de usuário ou senha inválidos");
                    }
                }
                2 => {
                    register(db);
                }
                3 => break,
                _ => println!("Opção inválida"),
            }
        } else {
            println!("opção inválida, tente novamente")
        }
    }
}

//menu do sistema bancário (saque e deposito)
fn transaction_menu(db: &Database, account: &mut Account) {
    loop {
        println!("Seu saldo atual é: {:.2}", account.balance);
        println!("Escolha a sua opção: ");
        println!("1 - Depositar");
        println!("2 - Sacar");
        println!("3 - Ver Extrato");
        println!("4 - Transferência");
        println!("5 - Sair");
        print!("Insira a sua opção aqui: ");

        let option = read_input("");
        if let Ok(option) = option.parse() {
            match option {
                1 => deposit_input(db, account),
                2 => withdraw_input(db, account),
                3 => {
                    let transactions = get_transactions(db, account.id);
                    for transaction in transactions {
                        println!("{:?}", transaction)
                    }
                }
                4 => transfer_input(db, account),
                5 => break,
                _ => println!("Opção inválida"),
            }
        } else {
            println!("Opção inválida, tente novamente")
        }
    }
}
