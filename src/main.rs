use std::io;

fn operations_system(mut deposit: f64) {
    loop {
        println!("Digite um valor: ");

        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Falha ao let entrada");

        let value: f64 = value.trim().parse().expect("Valor inválido");

        if value > 0.0 {
            deposit = value + deposit;
            println!("Você depositou {} e seu saldo total é: {}", value, deposit)
        } else if value == 0.0 {
            get_money(deposit);
        } else {
            println!("Valor inválido")
        }
    }
}

fn get_money(mut discredit: f64) {
    loop {
        println!("Digite quanto você deseja sacar: ");

        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Falha ao let entrada");

        let value: f64 = value.trim().parse().expect("Valor inválido");

        if value < discredit {
            discredit = discredit - value;
            println!("Você sacou {} e seu saldo é {}", value, discredit)
        } else {
            println!("Ocorreu um erro");
        }
    }
}
fn main() {
    let total_coins: f64 = 0.0;

    println!("Bem vindo ao seu internet banking");
    println!("escolha a sua opção: ");
    println!("1 - Inserir dinheiro");
    println!("2 - Sacar dinheiro");
    println!("Insira a sua opção aqui: ");

    let mut option = String::new();

    io::stdin().read_line(&mut option).expect("Erro de entrada");

    if option.contains("1") {
        operations_system(total_coins)
    } else if option.contains("2") {
        get_money(total_coins)
    } else {
        println!("Opção inválida")
    }
}
