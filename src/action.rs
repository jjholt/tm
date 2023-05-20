use std::fmt;

pub struct Action {
    pub behaviour: Behaviour,
    pub category: Category,
}

pub enum Behaviour {
    New,
    Delete,
    Add,
}

impl Behaviour {
    pub fn from(behaviour: char) -> Result<Self, &'static str> {
        Ok(match behaviour {
            'n' => Self::New,
            'd' => Self::Delete,
            'a' => Self::Add,
            _ => return Err("Invalid behaviour -- {behaviour}"),
        })
    }
}


#[derive(Debug)]
pub enum Category {
    Coursework,
    Notes,
    Paper,
    Chapter,
    Section,
}
impl Category {
    pub fn from(category: char) -> Result<Self, &'static str> {
        Ok(match category {
            'w' => Self::Coursework,
            'n' => Self::Notes,
            'p' => Self::Paper,
            'c' => Self::Chapter,
            's' => Self::Section,
            _ => return Err("Invalid category -- {category}"),
        })
    } 
    pub fn get_directories<'a>() -> Vec<&'a str> {
        vec!["coursework", "notes", "paper"]
    }
}


impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_dir() {
        Category::get_directories();
    }
}
