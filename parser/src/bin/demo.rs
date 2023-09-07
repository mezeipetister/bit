use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"

# Demo Demo
  Hello Bello
  Lorem ipsum dolorem

// Comment slash

# comment2

NOTE
    ID 12
    PARTNER farmmix kereskedelmi kft
    NET 100
    VAT 27
    GROSS 127
    CDATE 2023-01-01
    DDATE 2023-01-02
    IDATE 2023-01-03

// Comment slash

NOTE
    ID 12
    PARTNER farmmix kereskedelmi kft
    NET 100
    VAT 27
    GROSS 127

// comment3
// comment4

NOTE
    ID 12
    NET 100
    VAT 27
    GROSS 127
    "#;

    parser::expression::from_str(input).unwrap();
    Ok(())
}
