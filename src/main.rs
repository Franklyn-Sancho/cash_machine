use std::io;

static mut TOTAL_COINS: f64 = 0.0;

fn insert_money() {
    unsafe {
        let coins = [2.00, 5.00, 10.00, 20.00, 50.00, 100.00];

        loop {
            println!("Insira quanto você deseja investir: ");

            let mut credit = String::new();

            io::stdin()
                .read_line(&mut credit)
                .expect("Falha ao ler entrada");

            let credit = credit.trim().parse().expect("valor inválidos");

            if coins.contains(&credit) {
                TOTAL_COINS = credit + TOTAL_COINS;
                println!("Você inseriu {} e seu saldo é {}", credit, TOTAL_COINS)
            } else if credit == 0.0 {
                main()
            } else {
                println!("Ocorreu um erro")
            }
        }
    }
}

fn get_money() {
    unsafe {
        loop {
            println!("Digite quanto você deseja sacar: ");

            let mut discredit = String::new();

            io::stdin()
                .read_line(&mut discredit)
                .expect("Falha ao let entrada");

            let discredit: f64 = discredit.trim().parse().expect("Valor inválido");

            if discredit < TOTAL_COINS {
                TOTAL_COINS = TOTAL_COINS - discredit;
                println!("Você sacou {discredit} e seu saldo é {TOTAL_COINS}")
            } else {
                println!("Ocorreu um erro");
            }
        }
    }
}
fn main() {
    println!("Bem vindo ao seu internet banking");
    println!("escolha a sua opção: ");
    println!("1 - Inserir dinheiro");
    println!("2 - Sacar dinheiro");
    println!("Insira a sua opção aqui: ");

    let mut option = String::new();

    io::stdin().read_line(&mut option).expect("Erro de entrada");

    if option.contains("1") {
        insert_money()
    } else if option.contains("2") {
        get_money()
    } else {
        println!("Opção inválida")
    }
}
