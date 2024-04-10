fn main() {
    let hash = core::protocol::bytes_to_sha256(b"lorem ipsum dolorem");
    println!("{}", hash);
}
