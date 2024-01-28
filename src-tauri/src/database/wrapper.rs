pub trait DBWrapped {
    fn new(data: &serde_json::Value) -> Self;
    fn exists(&self) -> Option<i32>;  // Returns the id of the existing row if any
    fn save(&self) -> Result<i32, diesel::result::Error>;
}
