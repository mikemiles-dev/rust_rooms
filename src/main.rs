use std::collections::HashMap;
use std::error::Error;

#[derive(Default, Clone, Debug, PartialEq)]
struct Room<'a> {
    name: String,
    rooms: HashMap<String, &'a Box<Room<'a>>>,
}

impl<'a> Room<'a> {
    fn new(name: String) -> Box<Room<'a>> {
        let room = Room {
            name,
            ..Default::default()
        };
        Box::new(room)
    }

    fn add_direction(&mut self, direction: String, room: &'a Box<Room>) -> Result<(), Box<dyn Error>> {
        if *self == **room {
            return Err("It's myself!".into())
        }
        self.rooms.insert(direction, room);
        Ok(())
    }

    fn is_connected_internal(room: &'a Box<Room<'a>>, end_room: &'a Box<Room>, visited: &mut Vec<&'a Box<Room<'a>>>) -> bool {
        if room == end_room {
            return true
        }
        let copy_room = room.clone();
        visited.push(room);
        let mut results = vec![];
        for i in copy_room.rooms.iter() {
            results.push(Room::is_connected_internal(i.1, end_room, visited));
        }
        results.contains(&true)
    }

    fn is_connected(room: &'a Box<Room>, end_room: &'a Box<Room>) -> bool {
        Room::is_connected_internal(room, end_room, &mut Vec::new())
    }
}

fn main() {
    let mut garage = Room::new("garage".to_owned());
    let mut bathroom = Room::new("bathroom".to_owned());
    let kitchen = Room::new("kitchen".to_owned());

    bathroom.add_direction("West".to_string(), &kitchen).unwrap();
    garage.add_direction("North".to_string(), &bathroom).unwrap();

    println!("Is {} connected to {}?: {}",
        garage.clone().name,
        kitchen.clone().name,
        Room::is_connected(&garage, &kitchen));
}
