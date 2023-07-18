use crate::{
    database::database::Database,
    models::{
        account_model::{create_account, get_account_by_id, get_account_by_user, TransactionKind},
        user,
    },
    utils::read_input::read_input,
};

use super::account::Account;

pub fn transfer(
    db: &Database,
    from_account_id: &i32,
    to_email: &str,
    value: f64,
) -> Result<(), String> {
    let mut from_account = get_from_account(db, from_account_id)?;
    check_sufficient_balance(&from_account, value)?;

    let mut to_account = get_to_account(db, to_email)?;

    make_transfer(db, &mut from_account, &mut to_account, value)
}

fn get_from_account(db: &Database, from_account_id: &i32) -> Result<Account, String> {
    get_account_by_id(db, from_account_id).ok_or("Conta de origem não encontrada".to_string())
}

fn check_sufficient_balance(from_account: &Account, value: f64) -> Result<(), String> {
    if from_account.balance < value {
        Err("Saldo insuficiente".to_string())
    } else {
        Ok(())
    }
}

fn get_to_account(db: &Database, to_email: &str) -> Result<Account, String> {
    let to_user =
        user::User::find_by_email(db, to_email).ok_or("Usuário destinatário não encontrado")?;
    Ok(get_account_by_user(db, &to_user.id).unwrap_or_else(|| create_account(db, &to_user.id)))
}

fn make_transfer(
    db: &Database,
    from_account: &mut Account,
    to_account: &mut Account,
    value: f64,
) -> Result<(), String> {
    Account::update_balance(
        db,
        from_account,
        -value,
        TransactionKind::Transfer,
        format!("Transferência para {:?}", to_account),
    );
    Account::update_balance(
        db,
        to_account,
        value,
        TransactionKind::Transfer,
        format!("Transferência de {}", value),
    );

    Ok(())
}

pub fn transfer_input(db: &Database, account: &mut Account) {
    loop {
        let email =
            read_input("Insira o email do destinário (digite 0 para retornar ao menu principal): ");
        if email == "0" {
            break;
        }
        let value = read_input(
            "Insira o valor a ser transferido (digite 0 para retornar ao menu principal): ",
        );
        if value == "0" {
            break;
        }
        if let Ok(value) = value.parse::<f64>() {
            match transfer(db, &account.id, &email, value) {
                Ok(_) => println!("Você transferiu {:.2} para {}", value, email),
                Err(e) => println!("{}", e),
            }
        } else {
            println!("Valor inválido, tente novamente")
        }
    }
}
