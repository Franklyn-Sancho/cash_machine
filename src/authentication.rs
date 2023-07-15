use std::io::{self, Write};

use sha2::{Sha256, Digest};
use uuid::Uuid;

use crate::database::Database;
use crate::user::User;

pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Falha ao ler entrada");
    value.trim().to_string()
}

//função responsável por gerar uma hash
fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:x}", hasher.finalize())
}

pub fn register(db: &Database) {
    let email = read_input("Digite seu endereço de email: ");
    let password = read_input("Digite sua senha: ");

    //gera um hash SHA-256 da senha
    let password_hash = hash_password(&password);

    // gera um novo UUID para o usuário
    let user_id = Uuid::new_v4().to_string();

    User::create(db, &user_id, &email, &password_hash);

    println!("Usuário registrado com sucesso! {}", user_id);
}

// Função para autenticar o usuário
pub fn authenticate(db: &Database) -> Option<String> {
    println!("Bem vindo ao internet banking");

    let email = read_input("Digite seu email: ");
    let password = read_input("Digite sua senha: ");

    let password_hash = hash_password(&password);

    if let Some(user) = User::find_by_email(db, &email) {
        if user.password_hash == password_hash {
            return Some(user.email);
        }
    }

    None
}