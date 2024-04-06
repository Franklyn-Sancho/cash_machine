use rusqlite::params;
use uuid::Uuid;

use crate::{authentication::register::hash_password, database::database::Database};

pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn new(id: String, email: String, password_hash: String) -> Self {
        Self {
            id,
            email,
            password_hash,
        }
    }

    pub fn insert_user(db: &Database, email: &str, password: &str) {
        let password_hash = hash_password(password);
        let user_id = Uuid::new_v4().to_string();

        db.conn
            .execute(
                "INSERT INTO users (id, email, password) VALUES (?1, ?2, ?3)",
                params![user_id, email, password_hash],
            )
            .unwrap();
    }

    pub fn find_by_email(db: &Database, email: &str) -> Option<Self> {
        let mut stmt = db
            .conn
            .prepare("SELECT id, password FROM users WHERE email = ?1")
            .ok()?; // Tratamento de erro com `ok()`

        let user_iter = stmt
            .query_map([email], |row| Ok((row.get(0)?, row.get(1)?)))
            .ok()?;

        let user_data = user_iter.map(|x| x.unwrap()).next()?;
        let (user_id, password_hash) = user_data; // Usando `match` para desestruturar

        Some(Self::new(user_id, email.to_owned(), password_hash)) // Evitando clonagem desnecessária
    }
}
