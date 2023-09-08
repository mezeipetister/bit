use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = r#"

# Demo Demo
  Hello Bello
  KEY # //
  Lorem ipsum dolorem

// Comment slash

# comment2

ACCOUNT
    ID 38
    NAME Bank

ACCOUNT
    ID 3842
    NAME Kriszti bank

ACCOUNT
    ID 3841
    NAME Peti bank


NOTE
    ID 12 // Demo comment
    PARTNER farmmix kereskedelmi kft
    NET 100 # Example inline comment
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

NOTE ID 13 NET 100 GROSS 127 VAT 27 CDATE 2023-01-01 DDATE 2023-05-01 PARTNER Farmmix Kft.

NOTE
    ID      2023/NYH-0000129/AB
    DESCRIPTION
            Lorem ipsum dolorem
            set ami mi an more
    NET     100
    GROSS   127
    VAT     27
    CDATE   2023-01-01
    DDATE   2023-05-01
    PARTNER Farmmix Kft.

TRANSACTION
    CREDIT 1
    DEBIT  2
    AMOUNT 150

# Transaction demo
  as a last test
  by Peter Mezei

TRANSACTION
    REF    2023/NYH-0000129/AB
    CREDIT 1
    DEBIT  2
    CDATE  2023-01-01
    DDATE  2023-01-01
    IDATE  2023-01-01
    AMOUNT 150"#;

    parser::expression::from_str(input).unwrap();
    Ok(())
}
