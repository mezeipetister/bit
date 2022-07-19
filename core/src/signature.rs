use chrono::{Local, Utc};
use hex_literal::hex;
use md4::{Digest, Md4};
use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::parser::{Line, NoteRaw};

fn create_signature(bytes: &[u8]) -> String {
    // create a Md4 hasher instance
    let mut hasher = Md4::new();
    // process input message
    hasher.update(bytes);
    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 16]
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn sign_note(note: NoteRaw) {
    if !note.is_signed() {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(note.file_path())
            .unwrap();

        let signature = create_signature(&note.raw_bytes());

        if let Err(e) = writeln!(
            file,
            "\nSIGNATURE {} {}",
            Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
            signature
        ) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

pub fn check_signature(lines_slice: Vec<&Line>, signature: &str) -> bool {
    if let text_raw = lines_slice
        .iter()
        .map(|i| i.raw())
        .collect::<Vec<&str>>()
        .join("\n")
        .as_bytes()
    {
        return create_signature(text_raw) == signature;
    }
    false
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_sign() {
        let res = create_signature(b"hello world");
        assert_eq!(&res, "aa010fbc1d14c795d86ef98c95479d17");
    }

    #[test]
    fn test_sign_vec() {
        let res = create_signature(vec!["a", "b"].join("\n").as_bytes());
        assert_eq!(&res, "90937ae2b1c2fcb71e841fd0471226da");
    }

    #[test]
    fn test_sign_file() {
        let note = NoteRaw::from_file(&PathBuf::from("./example/notes/c.bit")).unwrap();
        sign_note(note);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_note_file() {
        let note_raw = NoteRaw::from_file(&PathBuf::from("./example/notes/c.bit")).unwrap();
        let note: crate::note::Note = crate::note::Note::from_raw_note(note_raw, false).unwrap();
        println!("{:?}", note);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_accounts_note_file() {
        let note_raw = NoteRaw::from_file(&PathBuf::from("./example/notes/accounts.bit")).unwrap();
        let note: crate::note::Note = crate::note::Note::from_raw_note(note_raw, false).unwrap();
        println!("{:?}", note);
        assert_eq!(1, 1);
    }
}
