// auth.rs
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::io;
use std::io::Write;
use uuid::Uuid;

use crate::model::insert_user;

// Definição da estrutura User para representar um usuário com nome de usuário e senha
/* pub struct User {
    pub email: String,
    pub password: String,
} */

// Função para ler a entrada do usuário e converter para String
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

pub fn register(conn: &Connection) {
    let email = read_input("Digite seu endereço de email: ");
    let password = read_input("Digite sua senha: ");

    //gera um hash SHA-256 da senha
    let password_hash = hash_password(&password);

    // gera um novo UUID para o usuário
    let user_id = Uuid::new_v4().to_string();

    insert_user(conn, &user_id, &email, &password_hash);

    println!("Usuário registrado com sucesso! {}", user_id);
}


// Função para autenticar o usuário
pub fn authenticate(conn: &Connection) -> Option<String> {
    println!("Bem vindo ao internet banking");

    let email = read_input("Digite seu email: ");
    let password = read_input("Digite sua senha: ");

    let password_hash = hash_password(&password);

    let mut stmt = conn
        .prepare("SELECT email FROM users WHERE email = ?1 AND password = ?2")
        .unwrap();
    let user_iter = stmt
        .query_map(params![email, password_hash], |row| row.get(0))
        .unwrap();


    for user_email in user_iter {
        if let Ok(user_email) = user_email {
            return Some(user_email);
        }
    }

    None
}
