mod todo;

use std::error::Error;
use tezos_data_encoding::nom::NomReader;
use tezos_smart_rollup::{kernel_entry, prelude::*, storage::path::OwnedPath};
use crate::todo::{Action, MAX_TODO_PAYLOAD_SIZE, MAX_TODO_SIZE, Todo, TodoActions};

const TODO_PATH: &str = "/todo/";

pub fn create_todo_path(id: i64) -> OwnedPath {
    // make path like "/todo/1"
    let path : String = format!("{}{}", TODO_PATH, id);
    path.as_bytes().to_vec().try_into().unwrap()
}

pub fn entry<Host: Runtime>(host: &mut Host) {
    execute(host);
    host.write_debug("End of executing Kernel\n");
}

fn execute<Host: Runtime>(host: &mut Host) {
    // Read the input
    let input = host.read_input();
    match input {
        // If it's an error or no messages then does nothing
        Err(_) | Ok(None) => {
            host.write_debug("read_input with EOF or error.\n");
        }
        Ok(Some(message)) => {
            debug_msg!(host, "--------------Start next input.\n");

            let data = message.as_ref();
            //debug_msg!(host, "data: {:?}\n", data);
            handle_todo_payloads(host, data).unwrap();
            // Process next input
            debug_msg!(host, "--------------Process next input.\n");
            execute(host);
        }
    }
}

fn handle_todo_payloads<Host: Runtime>(host: &mut Host, data: &[u8]) -> Result<(), Box<dyn Error>> {
    match data {
        [0x00, ..] => {
            host.write_debug("Message from the kernel:{}.\n");
            //debug_msg!(host, "Message from the kernel: {:?}\n", payload);
        }
        [0x01, payload @ ..] => {
            host.write_debug("Message from the user.\n");
            let (_, mut ta) = TodoActions::nom_read(payload).unwrap();
            debug_msg!(host, "TodoActions: {:?}\n", ta);
            let todo_path: OwnedPath = create_todo_path(ta.id);
            match ta.action {
                Action::Create => {
                    //let todo: [u8; MAX_TODO_SIZE] = ta.todo.clone().try_into().unwrap();
                    debug_msg!(host, "write to path: {:?}\n", todo_path);
                    if ta.todo.owner == "" {
                        ta.todo.owner = ta.user;
                    }
                    let todo_payload = ta.todo.encrypt();
                    let _ = Runtime::store_write_all(host, &todo_path, &todo_payload);
                }
                //Read for debug purpose only
                Action::Read => {
                    let todo_payload = Runtime::store_read_all(host, &todo_path)?;
                    let todo = Todo::decrypt(&todo_payload, ta.user.as_str());
                    debug_msg!(host, "Read Todo: {:?}\n", todo);
                }
                Action::Delete => {
                    let _ = Runtime::store_delete(host, &todo_path);
                }
                Action::MarkComplete => {
                    let mut todo_payload = Runtime::store_read_all(host, &todo_path)?;
                    let mut todo = Todo::decrypt(&todo_payload, ta.user.as_str());
                    todo.completed = true;
                    todo_payload = todo.encrypt();
                    let _ = Runtime::store_write_all(host, &todo_path, &todo_payload);
                }
            }
        }
        e => {
            //host.write_debug("Message from the unknown.\n");
            debug_msg!(host, "Message from the unknown: {:?}\n", e)
        }
    }
    Ok(())
}

kernel_entry!(entry);

// To run:
// 1. cargo build --release --target wasm32-unknown-unknown
// 2. octez-smart-rollup-wasm-debugger ./target/wasm32-unknown-unknown/release/md_dev_kernel.wasm --inputs ./inputs.json <<< $(cat commands.txt)
// For payload of "external" key in the inputs.json, you can generate it from test_ta_into() in todo.rs
#[cfg(test)]
mod tests {

    // This test is intent to create a payload to send to the kernel, like inputs.json.
    // Any message will do. Just see the output in the console.
    #[test]
    fn test_str_to_utf8() {
        let message = "Hello Kernel";
        println!("message: {}", message);
        let message = message.as_bytes();
        let message = message.to_vec();
        print!("exported bytes: ");
        for &byte in &message {
            print!("{:02X}", byte);
        }
        println!();
        let message = String::from_utf8(message).unwrap();
        assert_eq!(message, "Hello Kernel");
    }
}