pub mod handle_client_request;
pub mod handle_server_response;
pub mod room;
pub mod ui;
pub mod user;
pub mod user_input;

use std::collections::HashMap;
use std::error::Error;
use std::{
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str,
    sync::{Arc, Mutex},
    thread,
};

use crate::handle_client_request::RequestType;

use bincode::{deserialize, serialize};

extern crate serde;
#[macro_use]
extern crate serde_derive;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        user::user_tcp();
    } else {
        // server

        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        println!("Server is listening on 127.0.0.1:7878");

        let active_clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(vec![]));

        let rooms: Arc<Mutex<HashMap<String, room::Room>>> =
            Arc::new(Mutex::new(HashMap::with_capacity(10)));

        for stream in listener.incoming() {
            let stream = stream.expect("Failed to accept a connection");

            let active_clients = Arc::clone(&active_clients);
            let rooms = Arc::clone(&rooms);

            active_clients
                .lock()
                .unwrap()
                .push(stream.try_clone().expect("Failed to clone"));

            thread::spawn(move || {
                handle_connection(stream, active_clients, rooms).unwrap();
            });
        }
    }
}

fn handle_connection(
    mut stream: TcpStream,
    active_clients: Arc<Mutex<Vec<TcpStream>>>,
    rooms: Arc<Mutex<HashMap<String, room::Room>>>,
) -> Result<(), Box<dyn Error>> {
    println!("Incoming connection from: {}", stream.peer_addr().unwrap());
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read == 0 {
            return Ok(());
        }
        let send = &buf[..bytes_read];
        // let req = send;
        let req: RequestType = deserialize(&send).expect("Deserialization failed!");
        println!("{:?}", req);
        let response = handle_client_request::handle_request(
            req,
            rooms.lock().unwrap(),
            &stream,
            active_clients.lock().unwrap(),
        )?;
        println!("{:?}", response);

        let response = serialize(&response).expect("Serialization Failed");
        let response = format!("{}\n", str::from_utf8(&response).unwrap());
        stream.write(&response.as_bytes())?;
    }
}
