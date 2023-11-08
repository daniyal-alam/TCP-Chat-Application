use crate::handle_server_response::Response;
use crate::room::Room;
use std::collections::HashMap;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::sync::MutexGuard;

use std::error::Error;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestType {
    CreateRoom(String, String),
    JoinRoom(String, String),
    SendMessage(String, SocketAddr),
}

const MAX_ROOMS: usize = 10;

pub fn handle_request(
    req_type: RequestType,
    mut rooms: MutexGuard<'_, HashMap<std::string::String, Room>>,
    stream: &TcpStream,
    active_clients: MutexGuard<'_, Vec<TcpStream>>,
) -> Result<Response, Box<dyn Error>> {
    // let mut rooms = rooms.lock().unwrap();

    match req_type {
        RequestType::CreateRoom(room_name, _) => {
            if rooms.contains_key(room_name.as_str()) {
                return Ok(Response {
                    code: 1,
                    message: "Room Already exists. Try Again!".to_string(),
                });
            } else if rooms.len() == MAX_ROOMS {
                return Ok(Response {
                    code: 1,
                    message: "Server at full capacity. Try Again Later!".to_string(),
                });
            } else {
                rooms.insert(
                    room_name.clone(),
                    Room::new(room_name.clone(), stream.peer_addr().unwrap()),
                );
                return Ok(Response {
                    code: 0,
                    message: format!("Created room {}", &room_name),
                });
            }
        }
        RequestType::JoinRoom(room_name, _) => {
            println!("Room Name in join request: {}", room_name);
            let room = rooms.get_mut(&room_name);
            let room = match room {
                Some(room) => room,
                None => {
                    return Ok(Response {
                        code: 1,
                        message: "Room does not exist".to_string(),
                    })
                }
            };
            match room.join_room(stream.peer_addr()?) {
                Ok(()) => {
                    return Ok(Response {
                        code: 0,
                        message: format!("Joined Room {} successfully!", room_name).to_string(),
                    });
                }
                Err(()) => {
                    return Ok(Response {
                        code: 1,
                        message: "Room is full. Try Again".to_string(),
                    })
                }
            };
        }
        RequestType::SendMessage(message, addr) => {
            let message = format!("{}\n", message);
            broadcast(message.as_bytes(), active_clients, addr, rooms);
            Ok(Response {
                code: 0,
                message: "Successfully sent!".to_string(),
            })
        }
    }
}

fn broadcast(
    message: &[u8],
    mut active_clients: MutexGuard<'_, Vec<TcpStream>>,
    addr: SocketAddr,
    rooms: MutexGuard<'_, HashMap<std::string::String, Room>>,
) {
    // let mut active_clients = active_clients.lock().unwrap();
    let mut broadcast_room: Option<&Room> = None;
    let hashmap_room = rooms;
    for room in hashmap_room.values() {
        println!("{:?}", room.members);
        if room.members.contains(&addr) {
            broadcast_room = Some(room);
            break;
        }
    }

    let broadcast_room = match broadcast_room {
        Some(room) => room,
        None => panic!("Cannot find room"),
    };
    for client in &mut *active_clients {
        if broadcast_room
            .members
            .contains(&client.peer_addr().unwrap())
        {
            println!("Broadcasting to client");
            println!("Active client: {}", client.peer_addr().unwrap());
            let _ = client.write(message);
        }
    }
}
