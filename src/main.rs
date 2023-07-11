use std::io;
use std::io::Write;

// Definição da estrutura Account para representar uma conta bancária
struct Account {
    balance: f64, // Saldo da conta
    coins: [f64; 6], // Cédulas aceitas para depósito
}

// Implementação de métodos para a estrutura Account
impl Account {
    // Método para criar uma nova conta com saldo zero
    fn new() -> Self {
        Self {
            balance: 0.0,
            coins: [2.00, 5.00, 10.00, 20.00, 50.00, 100.00],
        }
    }

    // Método para depositar dinheiro na conta
    fn deposit(&mut self, value: f64) {
        // Verifica se a cédula é aceita
        if self.coins.contains(&value) {
            // Adiciona o valor ao saldo da conta
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
    fn withdraw(&mut self, value: f64) {
        // Verifica se há saldo suficiente na conta
        if value <= self.balance {
            // Subtrai o valor do saldo da conta
            self.balance -= value;
            println!("Você sacou {} e seu saldo é {}", value, self.balance);
        } else {
            println!("Saldo insuficiente");
        }
    }
}

// Função para ler a entrada do usuário e converter para f64
fn read_input(prompt: &str) -> f64 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut value = String::new();
    match io::stdin().read_line(&mut value) {
        Ok(_) => match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Valor inválido, tente novamente");
                read_input(prompt)
            }
        },
        Err(_) => {
            println!("Falha ao ler entrada, tente novamente");
            read_input(prompt)
        }
    }
}

// Função para realizar depósitos na conta
fn deposit(account: &mut Account) {
    loop {
        let value =
            read_input("Quando você deseja depositar (digite 0 para retornar ao menu inicial): ");
        if value == 0.0 {
            break;
        }
        account.deposit(value);
    }
}

// Função para realizar saques na conta
fn withdraw(account: &mut Account) {
    // Verifica se o usuário tem dinheiro suficiente na conta
    if account.balance == 0.0 {
        println!("Saldo insuficiente");
        return;
    }

    loop {
        let value = read_input(
            "Digite quanto você deseja sacar (digite 0 para retornar ao menu inicial): ",
        );
        if value == 0.0 {
            break;
        }
        account.withdraw(value);
    }
}

// Função para exibir o menu principal e ler a opção do usuário
fn menu(account: &mut Account) {
    loop {
        println!("Escolha a sua opção: ");
        println!("1 - Depositar");
        println!("2 - Sacar");
        println!("3 - Sair");
        print!("Insira a sua opção aqui: ");

        let option = read_input("");
        match option as i32 {
            1 => deposit(account),
            2 => withdraw(account),
            3 => break,
            _ => println!("Opção inválida"),
        }
    }
}

// Função principal do programa
fn main() {
    let mut account = Account::new(); // Cria uma nova conta bancária
    println!("Bem vindo ao seu internet banking");
    menu(&mut account); // Exibe o menu principal
}

