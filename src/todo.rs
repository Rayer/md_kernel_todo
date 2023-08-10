use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use serde_json_wasm as serde_json;

#[cfg(not(target_arch = "wasm32"))]
use serde_json;

pub const MAX_TODO_PAYLOAD_SIZE: usize = 2048;
pub const MAX_TODO_SIZE: usize = MAX_TODO_PAYLOAD_SIZE - 1;



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub created_time: u64,
    pub due_time: u64,
    pub completed: bool,
    pub owner: String,
}

impl Default for Todo {
    fn default() -> Self {
        Todo::new(0, "".to_string(), 0, 0, false, "".to_string())
    }
}

impl TryFrom<Vec<u8>> for Todo {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, String> {
        // deserialize Vec<u8> to Todo
        let todo_str = String::from_utf8(value).unwrap();
        let todo_str = todo_str.trim_end_matches(char::from(0));
        //debug_msg!("todo_str: {}", todo_str);
        let todo: Todo = serde_json_wasm::from_str(&todo_str).unwrap();
        //let todo: Todo = serde_json_wasm::from_slice(&value).unwrap();
        Ok(todo)
    }
}

impl Into<[u8; MAX_TODO_SIZE]> for Todo {
    fn into(self) -> [u8; MAX_TODO_SIZE] {
        // make Todo as json str
        let todo_str = serde_json_wasm::to_string(&self).unwrap();
        // return this str
        let mut ret: [u8; MAX_TODO_SIZE] = [0; MAX_TODO_SIZE]; // Initialize the array with zeros
        let bytes = todo_str.as_bytes();
        if bytes.len() <= ret.len() {
            ret[..bytes.len()].copy_from_slice(bytes);
            //println!("Converted array: {:?}", ret);
        } else {
            //println!("String is too long to fit in the array");
            panic!("String is too long to fit in the array")
        }
        ret
    }
}

impl Todo {
    pub fn new(id: u64, title: String, created_time: u64, due_time: u64, completed: bool, owner: String) -> Self {
        Self {
            id,
            title,
            created_time,
            due_time,
            completed,
            owner,
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

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Create,
    Read,
    Delete,
    MarkComplete,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoActions {
    pub action: Action,
    pub user: String,
    pub todo: Todo,
}

impl TryFrom<Vec<u8>> for TodoActions {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let str = String::from_utf8(value).unwrap();
        let str = str.trim_end_matches(char::from(0));
        serde_json::from_str(&str).map_err(|e| e.to_string())
    }
}

impl Into<Vec<u8>> for TodoActions {
    fn into(self) -> Vec<u8> {
        let todo_str = serde_json::to_string(&self).unwrap();
        let mut ret: Vec<u8> = Vec::with_capacity(MAX_TODO_PAYLOAD_SIZE);
        let bytes = todo_str.as_bytes();
        if bytes.len() <= ret.capacity() {
            ret.extend_from_slice(bytes);
        } else {
            panic!("String is too long to fit in the array")
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ta_into() {
        let t = TodoActions {
            action: Action::MarkComplete,
            todo: Todo::new(1, "Test1".to_string(), 0, 0, false, "Rayer".to_string()),
            user: "Rayer".to_string(),
        };
        let ta : Vec<u8> = t.into();
        for i in ta {
            print!("{:02X}", i);
        }
        println!()
    }

    #[test]
    fn test_ta_try_from() {
        let todo_str = r#"{"id":0,"title":"test","created_time":0,"due_time":0,"completed":false,"deleted":false,"owner":""}"#;
        println!("{}", todo_str);
        let todo: Todo = serde_json_wasm::from_str(&todo_str).unwrap();
        println!("{:?}", todo);
    }

}