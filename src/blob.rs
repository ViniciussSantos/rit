use sha1::{Digest, Sha1};

#[derive(Debug)]
pub struct Blob {
    pub oid: String,
    pub data: String,
    pub content: String,
}

impl Blob {
    pub fn new(data: String) -> Self {
        let content = Blob::create_content(&data);

        let oid = {
            let mut hasher = Sha1::new();
            hasher.update(content.as_bytes());
            let result = hasher.finalize();
            hex::encode(result)
        };

        Blob { oid, data, content }
    }

    pub fn create_content(data: &str) -> String {
        format!("blob {}\0{}", data.len(), data)
    }

    pub fn object_type(&self) -> String {
        "blob".to_string()
    }
}
