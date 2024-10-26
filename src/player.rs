use crate::item::Item;

pub enum PlayerState {
    Alive,
    Dead,
}

pub struct Player {
    pub state: PlayerState,
    pub current_room: usize,
    inventory: Vec<Item>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            state: PlayerState::Alive,
            current_room: 0,
            inventory: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.inventory.push(item);
    }

    pub fn show_inventory(&self) {
        if self.inventory.is_empty() {
            println!("You have no items in your inventory.");
        } else {
            println!("Inventory:");
            for item in &self.inventory {
                println!("- {}", item.name);
            }
        }
    }
}
