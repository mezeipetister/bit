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

TRANSACTION
    CREDIT 1
    DEBIT 2
    AMOUNT 150

// comment3

// comment4

NOTE ID 13 NET 100 GROSS 127 VAT 27 CDATE 2023-01-01 DDATE 2023-05-01 PARTNER Farmmix Kft."#;

    parser::expression::from_str(input).unwrap();
    Ok(())
}
