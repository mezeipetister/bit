use cli::Cli;

fn main() {
    let stdout = std::io::stdout();
    let stdin = std::io::stdin();
    let actions = |i: String| -> Result<String, String> {
        if i.is_empty() {
            return Ok("".into());
        }
        match i.as_str() {
            "hello" => Ok("Bello".into()),
            "yuhuu" => Ok("yuhuu too".into()),
            _ => Ok("unknown command".into()),
        }
    };
    Cli::new(&stdout, &stdin, actions).unwrap().run().unwrap();
}
