use std::error::Error;

use crate::action::Category;

pub mod add;
pub mod delete;
pub mod new;

macro_rules! implement {
    ($function_name:ident) => {
        fn $function_name(args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>>{
            let _ = args;
            // println!("function {} called successfully", stringify!($function_name));
            Err("Undefined behaviour".into())
        }
    };
}

pub trait Choose {
    implement!(coursework);
    implement!(notes);
    implement!(paper);
    implement!(chapter);
    implement!(section);

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
