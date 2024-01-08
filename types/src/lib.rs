use std::collections::HashMap;

pub trait ListOfValues {
    /// List all fields in the value.
    fn get(&self) -> HashMap<String, String>;
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl ListOfValues for Color {
    fn get(&self) -> HashMap<String, String> {
        HashMap::from([
            ("r".to_owned(), self.r.to_string()),
            ("g".to_owned(), self.g.to_string()),
            ("b".to_owned(), self.b.to_string()),
        ])
    }
}
