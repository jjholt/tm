use std::{
    env,
    process,
    io
};


mod behaviour;
mod command;
mod config;
mod action;

use command::Command;
use config::ConfigMetadata;

fn main() -> Result<(), io::Error>{
    let root = env::var("HOME").unwrap() + "/";
    let _config = ConfigMetadata::new(&root)?;
    let command = Command::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Unable to run: {err}");
        process::exit(1)
    });
    let _ = command.apply();

    // println!("{:#?}", project);

    Ok(())
}
