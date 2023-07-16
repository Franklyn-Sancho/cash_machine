use crate::{
    account::Account,
    authentication::{authenticate, register},
    database::Database,
    deposit, withdraw, utils::read_input,
};

//menu inicial para login e autenticação
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
                        let mut account = db.get_account(&user.id);
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
        println!("3 - Sair");
        print!("Insira a sua opção aqui: ");

        let option = read_input("");
        if let Ok(option) = option.parse() {
            match option {
                1 => deposit(db, account),
                2 => withdraw(db, account),
                3 => break,
                _ => println!("Opção inválida"),
            }
        } else {
            println!("Opção inválida, tente novamente")
        }
    }
}
