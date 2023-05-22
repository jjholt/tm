use std::error::Error;

use crate::action::Category;

pub mod add;
pub mod delete;
pub mod new;

macro_rules! category{
    ($function_name:ident) => {
        fn $function_name(args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>>{
            let _ = args;
            Ok(())
        }
    };
}

pub trait Choose {
    category!(coursework);
    category!(notes);
    category!(paper);
    category!(chapter);
    category!(section);

    fn choose(category: Category, args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> { 
        match category {
            Category::Coursework => Self::coursework(args),
            Category::Notes => Self::notes(args),
            Category::Paper => Self::paper(args),
            Category::Chapter => Self::chapter(args),
            Category::Section => Self::section(args),
        }
    }
}
