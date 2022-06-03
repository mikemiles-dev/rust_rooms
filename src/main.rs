use std::collections::HashMap;
use std::error::Error;

#[derive(Default, Debug, PartialEq)]
struct Room {
    name: String,
    rooms: HashMap<String, Option<Box<Room>>>,
}

impl Room {
    fn new(name: String) -> Box<Room> {
        let room = Room {
            name,
            ..Default::default()
        };
        Box::new(room)
    }

    fn add_direction(&mut self, direction: String, room: Box<Room>) -> Result<(), Box<dyn Error>> {
        if *self == *room {
            return Err("It's myself!".into())
        }
        self.rooms.insert(direction, Some(room));
        Ok(())
    }
}

fn main() {
    let mut garage = Room::new("garage".to_owned());
    let mut bathroom = Room::new("bathroom".to_owned());
    let mut kitch = Room::new("kitchen".to_owned());

    garage.add_direction("North".to_string(), bathroom).unwrap();

    println!("{:?}", garage);
}
