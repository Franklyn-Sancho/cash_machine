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
                id TEXT PRIMARY KEY, user_id TEXT, balance REAL,
                FOREIGN KEY(user_id) REFERENCES users(id)
            )",
                [],
            )
            .unwrap();

        //cria a tabela transactions
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS transactions (
                    id STRING PRIMARY KEY,
                    account_id STRING,
                    date TEXT,
                    value REAL,
                    kind TEXT,
                    description TEXT
                )",
                [],
            )
            .unwrap();
    }

    /* pub fn insert_user(&self, user_id: &str, email: &str, password_hash: &str) {
        self.conn
            .execute(
                "INSERT INTO users (id, email, password) VALUES (?1, ?2, ?3)",
                params![user_id, email, password_hash],
            )
            .unwrap();
    } */
}
