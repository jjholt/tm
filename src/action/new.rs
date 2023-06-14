use std::{process, error::Error};

use crate::category::Category;

pub struct New;

impl New {
    pub fn assign(templates_path: &str, category: &Category, args: &Vec<String>) -> Result<String, Box<dyn Error>> {
        match category {
            Category::Chapter | Category::Section => return Err("New(n) cannot be used for this category.".into()),
            _ => {
                let path = format!("{}{}/", templates_path, category.to_string().to_lowercase());
                let filename = args.iter().next().ok_or("Missing filename.")?;
                process::Command::new("cp")
                    .args(["-r", &path , &filename] )
                    .output()?;
            }
        }
        Ok(String::from("generated template."))
    }
}
