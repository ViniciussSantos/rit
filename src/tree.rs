use sha1::{Digest, Sha1};

use crate::entry::Entry;

#[derive(Debug)]
pub struct Tree {
    pub oid: String,
    pub entries: Vec<Entry>,
    pub content: String,
}

impl Tree {
    const MODE: &'static str = "100644";

    pub fn new(mut entries: Vec<Entry>) -> Self {
        entries.sort_by(|a, b| a.name.cmp(&b.name));

        let content = Self::create_content(&entries);

        let oid = {
            let mut hasher = Sha1::new();
            hasher.update(&content);
            let result = hasher.finalize();
            hex::encode(result)
        };

        Tree {
            oid,
            entries,
            content,
        }
    }

    pub fn create_content(entries: &[Entry]) -> String {
        let data = entries
            .iter()
            .map(|entry| format!("{:?} {:?}\0{:}", Self::MODE, entry.name, entry.oid))
            .collect::<Vec<String>>();

        data.join("")
    }
}
