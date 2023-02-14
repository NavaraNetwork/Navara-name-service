use crate::*;

#[near_bindgen]
impl Contract {
    pub fn set_text_records(&mut self, records: HashMap<String, String>) {
        Self::require_owner();
        for (key, value) in records {
            self.text_records.insert(key, value);
        }
        assert!(self.text_records.len() <= 10, "Too many records")
    }

    pub fn get_text_records(&self) -> HashMap<String, String> {
        self.text_records.to_owned()
    }
}