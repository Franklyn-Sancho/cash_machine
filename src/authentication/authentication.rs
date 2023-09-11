use bcrypt::verify;

use crate::account::account::Account;
use crate::database::database::Database;
use crate::models::account_model::{create_account, get_account_by_user};
use crate::models::user::User;
use crate::utils::read_input::read_input;

fn verify_password(password: &str, password_hash: &str) -> bool {
    verify(password, password_hash).unwrap()
}

pub fn authenticate(db: &Database) -> Option<Account> {
    println!("Welcome! Enter your login information");

    let email = read_input("Enter your email: ");
    let password = read_input("Enter your password: ");

    if let Some(user) = User::find_by_email(db, &email) {
        if verify_password(&password, &user.password_hash) {
            return Some(
                get_account_by_user(db, &user.id).unwrap_or_else(|| create_account(db, &user.id)),
            );
        }
    }

    None
}
