use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

use crate::database::database::Database;
use crate::models::user::User;
use crate::utils::read_input::read_input;

//hash com bcrypt
fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

fn verify_password(password: &str, password_hash: &str) -> bool {
    verify(password, password_hash).unwrap()
}

pub fn register(db: &Database) {
    println!("
    Welcome to internet banking! sign it: ");

    let email = read_input("enter your favorite email: ");
    let password = read_input("enter a secure password: ");

    create_user(db, &email, &password);

    println!("User registered successfully");
}

fn create_user(db: &Database, email: &str, password: &str) {
    let password_hash = hash_password(password);
    let user_id = Uuid::new_v4().to_string();
    User::create_user(db, &user_id, email, &password_hash);
}

// Função para autenticar o usuário
pub fn authenticate(db: &Database) -> Option<User> {
    println!("Welcome! Enter your login information");

    let email = read_input("Enter your email: ");
    let password = read_input("Enter your password: ");

    if let Some(user) = User::find_by_email(db, &email) {
        if verify_password(&password, &user.password_hash) {
            return Some(user);
        }
    }

    None
}
