macro_rules! implement {
    ($function_name:ident) => {
        fn $function_name(args: &Vec<String>) -> Result<String, Box<dyn Error>>{
            let _ = args;
            // println!("function {} called successfully", stringify!($function_name));
            Err("Undefined behaviour.".into())
        }
    };
}

pub mod add;
pub mod delete;
pub mod new;

use std::{
    error::Error,
    fs,
    io::Write
};

use crate::{
    behaviour::Behaviour,
    category::Category
};

/// Describes a command that is received from the terminal in it's entirety.
/// It consists of a `behaviour`, i.e. add, new, delete, and a category,
/// which is the class of the thing being added, e.g. a new coursework from template or a new
/// section within a chapter.
pub struct Action {
    pub behaviour: Behaviour,
    pub category: Category,
}

trait ToFilename {
    fn to_filename(&self)->Self;
    fn from_filename(&self) -> Self;
}

pub trait Assign {
    implement!(coursework);
    implement!(notes);
    implement!(paper);
    implement!(chapter);
    implement!(section);

    fn assign(category: &Category, args: &Vec<String>) -> Result<String, Box<dyn Error>> { 
        match category {
            Category::Coursework  => Self::coursework(args),
            Category::Notes       => Self::notes(args),
            Category::Paper       => Self::paper(args),
            Category::Chapter     => Self::chapter(args),
            Category::Section     => Self::section(args),
        }
    }
}

impl Action {
    pub fn apply(&self, target: &Vec<String>, templates_path: &str) -> Result<String, Box<dyn Error>> {
        match self.behaviour {
            Behaviour::New => new::New::assign(templates_path, &self.category, target),
            Behaviour::Delete => delete::Delete::assign(&self.category, target),
            Behaviour::Add => add::Add::assign(&self.category, target),
        }
    }
}

impl ToFilename for String {
    fn to_filename(&self)-> Self {
        self.to_ascii_lowercase().replace(" ", "-")
    }
    
    fn from_filename(&self) -> Self {
        let binding = self.replace("-", " ");
        let mut c = binding.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}
fn push_to_file(buffer: String, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut parent_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;
    parent_file.write_all(buffer.as_bytes())?;
    parent_file.flush()?;
    Ok(())
}
