# This repo is created to demonstrate how to work with environments while developing on the internet computer

To test this repo simply run
`ENV=production cargo test`
or any other environment that is specified in the `canister_ids.json`

# How this repo was set-up
- Started by creating a new project by running a 
`dfx new ic_project`

- naviged to the `src` folder and run
`cargo new env_file_generator`

- add a `Cargo.toml` to the root of the project and copy contents

- edit the `Cargo.toml` from the rust project folder and copy the contents
- change the `main.rs` to `lib.rs` and copy over the contents
- create a `generate.rs`, copy over the contents and edit as needed for your project
