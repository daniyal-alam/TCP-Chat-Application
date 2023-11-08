use crate::handle_client_request::RequestType;
use std::io;

pub fn get_category_input() -> RequestType {
    match choice_input(1, 2) {
        1 => {
            let req = RequestType::CreateRoom(get_room_name(), get_username());
            // println!("{:?}", req);
            req
        }
        2 => RequestType::JoinRoom(get_room_name(), get_username()),
        _ => panic!("Category does not exist"),
    }
}

fn get_username() -> String {
    println!("Enter User Name: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to get username");
    // println!("{}", name);
    name.trim().to_string()
}

fn get_room_name() -> String {
    println!("Enter Room Name: ");
    let mut id = String::new();
    io::stdin()
        .read_line(&mut id)
        .expect("Failed to get room name")
        .to_string();
    // println!("{}", id);
    id.trim().to_string()
}

pub fn choice_input(start: u32, end: u32) -> u32 {
    let mut cat_str = String::new();
    let mut cat: u32;

    loop {
        cat_str.clear();
        io::stdin()
            .read_line(&mut cat_str)
            .expect("Failed to read input");
        cat = cat_str
            .trim()
            .parse()
            .expect("Failed to convert to integer");

        match UserInput::new(cat, start, end) {
            Ok(user_input) => return user_input.value(),
            Err(()) => continue,
        }
    }
}

struct UserInput {
    value: u32,
}

impl UserInput {
    fn new(value: u32, start: u32, end: u32) -> Result<Self, ()> {
        if value < start || value > end {
            println!("Out of range. Category doesn't exist");
            return Err(());
        }
        Ok(UserInput { value })
    }

    fn value(&self) -> u32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_user_input() {
        match UserInput::new(2, 1, 3) {
            Ok(user_input) => assert_eq!(user_input.value, 2),
            Err(()) => panic!("Value out of bounds"),
        }
    }

    #[should_panic]
    #[test]
    fn test_user_input_fail() {
        match UserInput::new(5, 1, 3) {
            Ok(user_input) => assert_eq!(user_input.value, 5),
            Err(()) => panic!("Value out of bounds"),
        }
    }
}
