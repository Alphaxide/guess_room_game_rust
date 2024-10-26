use  crate::item::Item;

pub struct  Room {
    pub name: String,
    pub description: String,
    pub items: Vec<Item>
}

impl Room {
    pub fn new(name: &str, description: &str, items: Vec<Item>) -> Room {
        Room { name: name.to_string(), description: description.to_string(), items: items }
    }
}