use chrono::Utc;

use crate::{database::database::Database, models::{account_model::{TransactionKind, get_account_by_id, get_account_by_user, create_account, update_account, Transaction, create_transaction}, user}};

use super::account::Account;

pub fn deposit(db: &Database, account: &mut Account, value: f64) -> Result<(), String> {
    if !account.is_valid_coin(value) {
        return Err(format!("Só aceitamos cédulas de {:?}", account.coins));
    }

    update_balance(
        db,
        account,
        value,
        TransactionKind::Deposit,
        format!("Depósito de {:.2}", value),
    );
    Ok(())
}

pub fn withdraw(db: &Database, account: &mut Account, value: f64) -> Result<(), String> {
    if account.balance < value {
        return Err("Saldo insuficiente".to_string());
    }

    update_balance(
        db,
        account,
        -value,
        TransactionKind::Withdraw,
        format!("Saque de {:.2}", value),
    );
    Ok(())
}

pub fn transfer(
    db: &Database,
    from_account_id: &i32,
    to_email: &str,
    value: f64,
) -> Result<(), String> {
    let mut from_account =
        get_account_by_id(db, from_account_id).ok_or("Conta de origem não encontrada")?;
    if from_account.balance < value {
        return Err("Saldo insuficiente".to_string());
    }

    let to_user =
        user::User::find_by_email(db, to_email).ok_or("Usuário destinatário não encontrado")?;
    let mut to_account =
        get_account_by_user(db, &to_user.id).unwrap_or_else(|| create_account(db, &to_user.id));

    update_balance(
        db,
        &mut from_account,
        -value,
        TransactionKind::Transfer,
        format!("Transferência para {}", to_email),
    );
    update_balance(
        db,
        &mut to_account,
        value,
        TransactionKind::Transfer,
        format!("Transferência de {}", value),
    );

    Ok(())
}

pub fn update_balance(
    db: &Database,
    account: &mut Account,
    value: f64,
    kind: TransactionKind,
    description: String,
) {
    account.balance += value;
    update_account(db, account);

    let transaction = Transaction {
        account_id: account.id,
        date: Utc::now(),
        value,
        kind,
        description,
    };
    create_transaction(db, &transaction);
}