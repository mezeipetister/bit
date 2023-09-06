use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"

# Demo Demo
  Hello Bello
  Lorem ipsum dolorem

# comment2

NOTE
    K V
    K V
    K V

// comment3
// comment4

NOTE
    K V
    K V
    K V
    K V"#;

    let token_stream = parser::frame::parse(input)?;
    println!("{:?}", token_stream);
    Ok(())
}
