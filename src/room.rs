use std::net::SocketAddr;

#[allow(dead_code)]
pub struct Room {
    room_name: String,
    pub members: Vec<SocketAddr>,
    room_size: usize,
}

impl Room {
    const MAX_ROOM_SIZE: usize = 4;

    pub fn new(room_name: String, addr: SocketAddr) -> Room {
        let members = vec![addr];
        println!("Created Room {}", room_name);

        Room {
            room_name,
            members,
            room_size: 0,
        }
    }

    pub fn join_room(&mut self, addr: SocketAddr) -> Result<(), ()> {
        if self.room_size == Room::MAX_ROOM_SIZE {
            println!("Room is at full capacity. Try Again Later!");
            return Err(());
        }
        self.room_size += 1;
        self.members.push(addr);
        Ok(())
    }
}
