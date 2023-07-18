// Definição da estrutura Account para representar uma conta bancária
pub struct Account {
    pub id: i32,
    pub balance: f64, // Saldo da conta
    pub coins: [f64; 6],  // Cédulas aceitas para depósito
}

impl Account {
    // Método para criar uma nova conta com saldo zero
    pub fn new(id: i32, balance: f64) -> Self {
        Self {
            id,
            balance,
            coins: [2.00, 5.00, 10.00, 20.00, 50.00, 100.00],
        }

    }

    pub fn is_valid_coin(&self, value: f64) -> bool {
        self.coins.contains(&value)
    }
}
