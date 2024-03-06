use std::fmt::Display;

use crate::entry::Entry;

const MODE: &str = "100644";

pub struct Tree {
    oid: String,
    entries: Vec<Entry>,
}

impl Tree {
    pub fn new(oid: String, entries: Vec<Entry>) -> Self {
        Tree { oid, entries }
    }

    pub fn object_type() -> String {
        "tree".to_string()
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut entries = self.entries.clone();

        entries.sort_by(|a, b| a.name.cmp(&b.name));

        let result = entries
            .iter()
            .map(|entry| format!("{} {} {}\0", MODE, entry.oid, entry.name))
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{}", result)
    }
}
