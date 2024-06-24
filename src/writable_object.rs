pub trait WritableObject {
    fn create_content(data: &str) -> String;
    fn get_oid() -> String;
    fn get_content() -> String;
    fn object_type(&self) -> String;
}
