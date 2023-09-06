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

    // let token_stream = parser::token::parse(input)?;
    // println!("{:?}", token_stream);

    parser::expression::from_str(input).unwrap();
    Ok(())
}
