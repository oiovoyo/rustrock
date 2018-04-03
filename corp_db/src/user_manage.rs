pub struct DataBase {
    corps: Vec<Corp>,
}

pub struct Corp {
    name: String,
    people: Vec<String>,
}
impl Corp {
    pub fn add_people(name: &str, corp: &str) {}
    pub fn list_people(corp: &str) -> Vec<String> {}
}
impl DataBase {
    fn load_db(&self) {}
    fn save_db(&self) {}
}
