use bcrypt::{hash, DEFAULT_COST, verify};
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
    let email = read_input("Digite seu endereço de email: ");
    let password = read_input("Digite sua senha: ");

    //gera um hash SHA-256 da senha
    let password_hash = hash_password(&password);

    // gera um novo UUID para o usuário
    let user_id = Uuid::new_v4().to_string();

    User::create_user(db, &user_id, &email, &password_hash);

    println!("Usuário registrado com sucesso! {}", user_id);
}

// Função para autenticar o usuário
pub fn authenticate(db: &Database) -> Option<User> {
    println!("Bem vindo ao internet banking");

    let email = read_input("Digite seu email: ");
    let password = read_input("Digite sua senha: ");

    if let Some(user) = User::find_by_email(db, &email) {
        if verify_password(&password, &user.password_hash) {
            return Some(user)
        }
    }

    None
}

