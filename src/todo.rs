
pub const MAX_TODO_PAYLOAD_SIZE: usize = 2048;
pub const MAX_TODO_SIZE: usize = MAX_TODO_PAYLOAD_SIZE - 1;

use magic_crypt::{MagicCryptTrait, new_magic_crypt};
use tezos_data_encoding;
use tezos_data_encoding::enc::BinWriter;
use tezos_data_encoding::nom::NomReader;

#[derive(NomReader, BinWriter, Debug)]
pub struct Todo {
    pub title: String,
    pub created_time: i64,
    pub due_time: i64,
    pub completed: bool,
    pub owner: String,
}

impl Default for Todo {
    fn default() -> Self {
        Todo::new("".to_string(), 0, 0, false, "".to_string())
    }
}


impl Todo {
    pub fn new(title: String, created_time: i64, due_time: i64, completed: bool, owner: String) -> Self {
        Self {
            title,
            created_time,
            due_time,
            completed,
            owner,
        }
    }

    pub fn encrypt(&self) -> Vec<u8> {
        let mut bytes = Vec::default();
        let result = self.bin_write(&mut bytes);
        match result {
            Ok(_) => (),
            Err(e) => {
                println!("Error encrypting todo: {:?}", e);
                return Vec::default();
            }
        }
        let mc = new_magic_crypt!(&self.owner, 256);
        mc.encrypt_to_bytes(bytes.as_slice())
    }

    pub fn decrypt(bytes: &[u8], owner: &str) -> Self {
        let mc = new_magic_crypt!(owner, 256);
        let decrypted = mc.decrypt_bytes_to_bytes(bytes).unwrap();
        let result = Todo::nom_read(decrypted.as_slice());
        match result {
            Ok((_, res)) => (res),
            Err(e) => {
                println!("Error decrypting todo: {:?}", e);
                return Todo::default();
            }
        }
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn is_due(&self) -> bool {
        self.due_time > 0 && self.due_time < self.created_time
    }

    pub fn accessible_by(&self, user: &str) -> bool {
        self.owner == user
    }
}

#[derive(Debug, Clone, NomReader, BinWriter)]
pub enum Action {
    Create,
    Read,
    Delete,
    MarkComplete,
}

#[derive(Debug, NomReader, BinWriter)]
pub struct TodoActions {
    pub id: i64,
    pub action: Action,
    pub user: String,
    pub todo: Todo,
}


#[cfg(test)]
mod tests {
    use tezos_data_encoding::encoding::Encoding;
    use super::*;

    #[test]
    fn test_ta_into() {
        let t = TodoActions {
            id: 1,
            action: Action::MarkComplete,
            todo: Todo::default(),
            user: "Rayer".to_string(),
        };

        let mut output = Vec::default();
        let result = t.bin_write(&mut output);
        for i in &output {
            print!("{:02X}", i);
        }
        println!();

        let (_, expected) = TodoActions::nom_read(&output).unwrap();
        println!("{:?}", expected);
    }

    // #[test]
    // fn test_ta_try_from() {
    //     let todo_str = r#"{"id":0,"title":"test","created_time":0,"due_time":0,"completed":false,"deleted":false,"owner":""}"#;
    //     println!("{}", todo_str);
    //     let todo: Todo = serde_json_wasm::from_str(&todo_str).unwrap();
    //     println!("{:?}", todo);
    // }

}