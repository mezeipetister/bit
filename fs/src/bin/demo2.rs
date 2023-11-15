fn encrypt(bytes: &[u8], secret: &[u8]) -> Vec<u8> {
    let len = secret.len();
    bytes
        .iter()
        .enumerate()
        .map(|(index, byte)| byte ^ secret[index % len])
        .collect()
}

fn decrypt(bytes: &[u8], secret: &[u8]) -> Vec<u8> {
    let len = secret.len();
    bytes
        .iter()
        .enumerate()
        .map(|(index, byte)| byte ^ secret[index % len])
        .collect()
}

fn main() {
    let secret = b"hellobello";
    let a = "hello bello mi a helyzet?";

    let encrypted = encrypt(a.as_bytes(), secret);
    let decrypted = decrypt(&encrypted, secret);

    println!("Original: {}", &a);
    println!("Encypted: {}", String::from_utf8_lossy(&encrypted));
    println!("Decrypted: {}", String::from_utf8_lossy(&decrypted));
}
