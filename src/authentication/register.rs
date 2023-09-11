use bcrypt::{hash, DEFAULT_COST};
use uuid::Uuid;

use crate::{database::database::Database, utils::read_input::read_input, models::user::User};

fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

pub fn register(db: &Database) {
    println!("Welcome to internet banking! sign it: ");

    let email = read_input("enter your favorite email: ");
    let password = read_input("enter a secure password: ");

    if User::find_by_email(db, &email).is_some() {
        println!("invalid email or password");
        return;
    }

    create_user(db, &email, &password);

    println!("User registered successfully");
}

fn create_user(db: &Database, email: &str, password: &str) {
    let password_hash = hash_password(password);
    let user_id = Uuid::new_v4().to_string();
    User::create_user(db, &user_id, email, &password_hash);
}