use sha1::{Digest, Sha1};

#[derive(Debug)]
pub struct Blob {
    pub oid: String,
    pub data: String,
}

impl Blob {
    pub fn new(data: String) -> Self {
        Blob {
            oid: format!(
                "{:X}",
                Sha1::digest(
                    [
                        "blob".to_string(),
                        format!("{:x}", data.clone().as_bytes().len()),
                        data.clone(),
                    ]
                    .join("")
                    .as_bytes(),
                )
            ),
            data,
        }
    }

    pub fn object_type(&self) -> String {
        "blob".to_string()
    }
}
