pub struct Item {
    pub name: String,
    pub description: String,
}

impl Item {
    pub  fn  new(name: &str, description: &str) -> Item{
        Item{
            name: name.to_string(),
            description: description.to_string(),
        }
        
    }
    
}