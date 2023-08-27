# Example: Todo Item

In our second kernel, we will demonstrate how to write and read data in durable state of the rollup.

## Running the example

First, compile the kernel to WASM with Cargo:

<!-- $MDX skip -->

```sh
$ cargo build --release --target wasm32-unknown-unknown
```

Then you can execute the kernel locally against the provided inputs (empty in this example) and commands:

```sh
$ octez-smart-rollup-wasm-debugger ./target/wasm32-unknown-unknown/release/md_dev_kernel.wasm --inputs ./inputs.json <<< $(cat commands.txt)

```

Additionally, you can omit the `<<< $(cat ./commands.txt)` to enter a REPL mode and
explore the execution of the kernel interactively.

## In input.json

We need to provide payload of user input. You can take a look into input.json:

```json
[
  [
    {
      "external": "0000000000000001000000000552617965720000000554657374310000000000000000000000000000000000000000055261796572"
    },
    {
      "external": "00000000000000010100000005526179657200000000000000000000000000000000000000000000000000"
    },
    {
      "external": "00000000000000010300000005526179657200000000000000000000000000000000000000000000000000"
    },
    {
      "external": "00000000000000010100000005526179657200000000000000000000000000000000000000000000000000"
    }
  ]
]
```

This payload can be generated from `test_ta_into()` in `todo.rs`. This 4 payloads means:
1. Create a todo with title "Test" and author "Rayer" in key `/todo/1`
2. Read this todo
3. Mark complete of this todo
4. Read this todo again.

Base concept is
- Provide TodoAction and encode in `external` field
- TodoAction will be decoded and execute these actions:
  - Create : Encrypt TodoAction::todo into Todo and store in key `/todo/{id}`
  - Read : Decrypt Todo in key `/todo/{id}` into TodoAction::todo, it is for debug propose only
  - MarkComplete : Decrypt Todo in key `/todo/{id}` into TodoAction::todo, and update TodoAction::todo.completed = true, then encrypt TodoAction::todo into Todo and store in key `/todo/{id}`
  - Delete : Delete key `/todo/{id}`

You can refer to `test_ta_into()` in `todo.rs` to see how to generate this payload.

## Authentication and Authorization

You need to provide correct user in TodoAction.User in order to access Todo Item. If user is not matched, transaction will be failed for failed to decrypt Todo payload into Todo.

<img width="579" alt="image" src="https://github.com/Rayer/md_kernel_todo/assets/156013/a0468a4c-e319-4e5a-892d-40d1b19767a5">
