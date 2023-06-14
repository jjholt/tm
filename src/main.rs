//! # LaTeX Manager (tm)
//!
//! `tm` simplifies some of the tedious file creation and linking when working in large projects.
//! Each call of the binary consists of an Action, which is split into the Behaviour and Category.
//! E.g., `new coursework` or `add chapter`, which would be the options `nw` and `ac`, respectively.
//!
//! Certain assumptions are made about the structure of your project.
//! 1. `index.tex` in the root of the project indexes all chapters.
//! 2. Expected structure of the project:  
//! ```bash
//! .
//! ├── index.tex
//! ├── main.tex
//! └── src
//!     ├── my-chapter
//!     │   ├── section-1.tex
//!     │   └── section-2.tex
//!     └── my-chapter.tex
//!```


use std::{
    env,
    process,
    error::Error, println
};


mod behaviour;
mod command;
mod config;
mod category;
mod action;

use command::Command;
use config::{ConfigMetadata, Config};

fn main() -> Result<(), Box<dyn Error>>{
    let root = env::var("HOME").unwrap() + "/";
    let metadata = ConfigMetadata::new(&root)?;
    let config = Config::new(metadata);
    // Command::build(env::args())?
    //     .apply(&config)
    //     .unwrap_or_else(|err| {
    //         eprintln!("Unable to run: {err}");
    //         process::exit(1) });

    match Command::build(env::args())?.apply(&config.metadata.templates_path) {
        Ok(message) => {
            println!("Successfully {}", message);
            Ok(())
        },
        Err(err) => {
            eprintln!("Unable to run: {err}");
            process::exit(1)
        },
    }
}
