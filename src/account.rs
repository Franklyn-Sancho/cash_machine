use rusqlite::params;
use crate::database::Database;

// Definição da estrutura Account para representar uma conta bancária
pub struct Account {
    pub id: i32,
    pub balance: f64, // Saldo da conta
    coins: [f64; 6], // Cédulas aceitas para depósito
}

// Implementação de métodos para a estrutura Account
impl Account {
    // Método para criar uma nova conta com saldo zero
    pub fn new(id: i32, balance: f64) -> Self {
        Self {
            id,
            balance,
            coins: [2.00, 5.00, 10.00, 20.00, 50.00, 100.00],
        }
    }

    // Método para depositar dinheiro na conta
    pub fn deposit(&mut self, value: f64) {
        // Verifica se a cédula é aceita
        if self.coins.contains(&value) {
            self.balance += value;
            println!(
                "Você depositou {} e seu saldo total é: {}",
                value, self.balance
            );
        } else {
            println!("Só aceitamos cédulas de {:?}", self.coins);
        }
    }

    // Método para sacar dinheiro da conta
    pub fn withdraw(&mut self, value: f64) {
        // Verifica se há saldo suficiente na conta
        if value <= self.balance {
            // Subtrai o valor do saldo da conta
            self.balance -= value;
            println!("Você sacou {} e seu saldo é {}", value, self.balance);
        } else {
            println!("Saldo insuficiente");
        }
    }

    pub fn save(&self, db: &Database) {
        db.conn.execute(
            "UPDATE accounts SET balance = ?1 WHERE id = ?2",
            params![self.balance, self.id],
        )
        .unwrap();
    }

}