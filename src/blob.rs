use sha1::{Digest, Sha1};

#[derive(Debug)]
pub struct Blob {
    pub oid: String,
    pub data: String,
}

impl Blob {
    pub fn new(data: String) -> Self {
        Blob {
            oid: format!("{:X}", Sha1::digest(data.as_bytes())),
            data,
        }
    }
}
