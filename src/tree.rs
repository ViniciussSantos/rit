use crate::entry::Entry;

pub struct Tree {
    oid: String,
    entries: Vec<Entry>,
}

impl Tree {
    pub fn new(oid: String, entries: Vec<Entry>) -> Self {
        Tree { oid, entries }
    }

    pub fn object_type(&self) -> String {
        "tree".to_string()
    }

    pub fn to_string() -> String {
        unimplemented!()
    }
}
