use crate::player::{Player, PlayerState};
use crate::room::Room;
use crate::item::Item;

pub struct Game {
    rooms: Vec<Room>,
    player: Player,
}

impl Game {
    pub fn new() -> Game {
        let rooms = vec![
            Room::new("Entrance Hall", "You are in a grand entrance hall.", vec![Item::new("Key", "A small rusty key.")]),
            Room::new("Library", "You are surrounded by shelves of books.", vec![]),
          
        ];

        let player = Player::new();

        Game { rooms, player }
    }

    pub fn display_current_room(&self) {
        let room = &self.rooms[self.player.current_room];
        println!("You are in: {}", room.name);
        println!("{}", room.description);
    }

    pub fn handle_command(&mut self, command: &str) -> bool {
        match command.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["move", direction] => self.move_player(direction),
            ["pick", "up", item_name] => self.pick_up_item(item_name),
            ["use", item_name] => self.use_item(item_name),
            ["quit"] => return false,
            _ => println!("Unknown command."),
        }
        true
    }

    fn move_player(&mut self, direction: &str) {
        match direction {
            "north" => {
                if self.player.current_room == 0 {
                    self.player.current_room = 1; 
                    println!("You move to the Library.");
                } else {
                    println!("You can't go that way.");
                }
            }
            _ => println!("Invalid direction."),
        }
    }

    fn pick_up_item(&mut self, item_name: &str) {
        let room = &mut self.rooms[self.player.current_room];
        if let Some(index) = room.items.iter().position(|item| item.name == item_name) {
            let item = room.items.remove(index);
            println!("You picked up: {}", item.name);
            self.player.add_item(item);
        } else {
            println!("Item not found in this room.");
        }
    }

    fn use_item(&self, item_name: &str) {
        println!("Using item: {}", item_name); 
    }
}
