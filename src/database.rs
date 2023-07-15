use crate::account::Account;
use rusqlite::{params, Connection};

//estrutura de conexão
pub struct Database {
    pub conn: Connection,
}

//configura um novo banco de dados
impl Database {
    pub fn new(path: &str) -> Self {
        let conn = Connection::open(path).unwrap();
        Self { conn }
    }

    //função responsável por criar as tabelas
    pub fn create_tables(&self) {
        //cria a tabela de usuários
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY, email TEXT UNIQUE, password TEXT
            )",
                [],
            )
            .unwrap();

        //cria a tabela de contas;
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS accounts (
                id INTEGER PRIMARY KEY, user_id TEXT, balance REAL,
                FOREIGN KEY(user_id) REFERENCES users(id)
            )",
                [],
            )
            .unwrap();
    }

    pub fn insert_user(&self, user_id: &str, email: &str, password_hash: &str) {
        self.conn
            .execute(
                "INSERT INTO users (id, email, password) VALUES (?1, ?2, ?3)",
                params![user_id, email, password_hash],
            )
            .unwrap();
    }

    pub fn get_account(&self, user_id: &str) -> Account {
        self.conn
            .query_row(
                "SELECT id, balance FROM accounts WHERE user_id = ?1",
                [user_id],
                |row| Ok(Account::new(row.get(0)?, row.get(1)?)),
            )
            .unwrap_or_else(|_| {
                self.conn
                    .execute(
                        "INSERT INTO accounts (user_id, balance) VALUES (?1, 0)",
                        [user_id],
                    )
                    .unwrap();
                let account_id = self.conn.last_insert_rowid();
                Account::new(account_id as i32, 0.0)
            })
    }
}
