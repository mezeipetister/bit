use std::io::Write;

pub fn read_input(question: &str) -> String {
    let mut buffer = String::new();
    print!("{question} ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

pub fn read_confirm() -> bool {
    let mut buffer = String::new();
    print!("Are you sure? (y/n): ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_lowercase() == "y"
}
