use crate::{
    database::database::Database, 
    models::account_model::{Transaction, TransactionKind, create_transaction}};
use chrono::Utc;


// Definição da estrutura Account para representar uma conta bancária
pub struct Account {
    pub id: i32,
    pub balance: f64, // Saldo da conta
    coins: [f64; 6], // Cédulas aceitas para depósito
    transactions: Vec<Transaction>
}

// Implementação de métodos para a estrutura Account
impl Account {
    // Método para criar uma nova conta com saldo zero
    pub fn new(id: i32, balance: f64) -> Self {
        Self {
            id,
            balance,
            coins: [2.00, 5.00, 10.00, 20.00, 50.00, 100.00],
            transactions: Vec::new(),
        }
    }

    // Método para depositar dinheiro na conta
    pub fn deposit(&mut self, db: &Database, value: f64) {
        // Verifica se a cédula é aceita
        if self.coins.contains(&value) {
            self.balance += value;
            println!(
                "Você depositou {:.2} e seu saldo total é: {:.2}",
                value, self.balance
            );

            let transaction = Transaction {
                account_id: self.id,
                date: Utc::now(),
                value,
                kind: TransactionKind::Deposit,
                description: format!("deposito de {:.2}", value)
            };
            create_transaction(db, &transaction);
            self.transactions.push(transaction);
        } else {
            println!("Só aceitamos cédulas de {:?}", self.coins);
        }
    }

    // Método para sacar dinheiro da conta
    pub fn withdraw(&mut self, db: &Database ,value: f64) {
        // Verifica se há saldo suficiente na conta
        if value <= self.balance {
            // Subtrai o valor do saldo da conta
            self.balance -= value;
            println!("Você sacou {} e seu saldo é {:.2}", value, self.balance);
            let transaction = Transaction {
                account_id: self.id,
                date: Utc::now(),
                value,
                kind: TransactionKind::Withdraw,
                description: format!("saque de {:.2}", value)
            };
            
            create_transaction(db, &transaction);
            self.transactions.push(transaction);
        } else {
            println!("Saldo insuficiente");
        }
    }

}