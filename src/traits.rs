pub trait MongoObject {
    fn new(name: &str) -> Self;
    fn id(&self) -> String;
    fn set_id(&mut self, id: &str);
}
