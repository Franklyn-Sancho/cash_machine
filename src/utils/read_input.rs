use std::io::{self, Write};

pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect(
        "
        failed to read input",
    );
    value.trim().to_string()
}

pub fn read_input_and_check(input_prompt: &str) -> Option<String> {
    let input = read_input(input_prompt);
    if input == "0" {
        None
    } else {
        Some(input)
    }
}
