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
    pub deleted: bool,
    pub owner: String,
}

impl Default for Todo {
    fn default() -> Self {
        Todo::new(0, "".to_string(), 0, 0, false)
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
    pub fn new(id: u64, title: String, created_time: u64, due_time: u64, completed: bool) -> Self {
        Self {
            id,
            title,
            created_time,
            due_time,
            completed,
            deleted: false,
            owner: "".to_string(),
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

    //once deleted, it cannot be recovered
    pub fn delete(&mut self) {
        self.deleted = true;
    }

    pub fn accessible_by(&self, user: &str) -> bool {
        self.owner == user
    }
}

#[derive(Debug)]
pub enum Action {
    Create,
    Read,
    Delete,
    MarkComplete,
}

impl TryFrom<Vec<u8>> for Action {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match value.first() {
            Some(&b'c') => Ok(Action::Create),
            Some(&b'r') => Ok(Action::Read),
            Some(&b'd') => Ok(Action::Delete),
            Some(&b'u') => Ok(Action::MarkComplete),
            _ => Err("Deserialization is not respected".to_string()),
        }
    }
}

impl Into<[u8;1]> for Action {
    fn into(self) -> [u8;1] {
        match self {
            Action::Create => [b'c'],
            Action::Read => [b'r'],
            Action::Delete => [b'd'],
            Action::MarkComplete => [b'u'],
        }
    }
}

#[derive(Debug)]
pub struct TodoActions {
    pub action: Action,
    //pub user: String,
    pub todo: Todo,
}

impl TryFrom<Vec<u8>> for TodoActions {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let action = Action::try_from(value.clone())?;
        let todo = Todo::try_from(value[1..].to_vec())?;
        Ok(TodoActions { action, todo })
    }
}

impl Into<Vec<u8>> for TodoActions {
    fn into(self) -> Vec<u8> {
        let mut ret  = Vec::<u8>::new();
        ret.push(<Action as Into<[u8;1]>>::into(self.action)[0]);
        ret.extend(<Todo as Into<[u8;MAX_TODO_SIZE]>>::into(self.todo)[0..].to_vec());
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
            todo: Todo::new(0, "test".to_string(), 0, 0, false),
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