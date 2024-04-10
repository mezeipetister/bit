use sha2::{Digest, Sha256};

pub fn bytes_to_sha256(bytes: &[u8]) -> String {
    // create a Sha1 object
    let mut hasher = Sha256::new();

    // process input message
    hasher.update(bytes);

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 20]
    format!("{:x}", hasher.finalize())
}
