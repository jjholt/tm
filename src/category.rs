use std::fmt;

/// Category groups Class and Section. This allows us to define behaviour that makes sense, e.g.
/// new coursework, without having to represent states that are not valid, e.g. new section. A
/// section must be _added_.
#[derive(Debug)]
pub enum Category {
    Coursework,
    Notes,
    Paper,
    Chapter,
    Section,
}


/// Used to convert from a `char` to the appropriate category (Class + Section).
pub trait ToCategory {
    fn to_category(self) -> Result<Category, &'static str>;
}


impl ToCategory for char {
    fn to_category(self) -> Result<Category, &'static str> {
        Ok(match self {
            'w' => Category::Coursework,
            'n' => Category::Notes,
            'p' => Category::Paper,
            'c' => Category::Chapter,
            's' => Category::Section,
            _ => return Err("Invalid category"),
        })
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}
