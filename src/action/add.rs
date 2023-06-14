use std::{
    fs,
    error::Error,
};

use crate::action::{self, ToFilename};
use super::Assign;
pub struct Add;
impl Assign for Add {
    fn section(args: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let mut buffer = String::new();

        let mut args = args.iter();
        let folder = args.next().ok_or("No folder provided.")?;

        let sections: Vec<&String> = args.collect();
        if sections.is_empty() {
            return Err("No filename provided.".into());
        };

        fs::create_dir_all(&folder)?;
        for section in sections {
            fs::File::create(format!("{}/{}.tex", folder.to_filename(), section.to_filename()))?;

            buffer.push_str(
                &format!("\\section{{{}}}\n\t\\subimport{{{}}}{{{}}}\n",
                         section, folder.to_filename(), section.to_filename())
                );
        }

        let index_filename = format!("{}.tex", folder);
        action::push_to_file(buffer, &index_filename)?;
        Ok(format!("added sections to {}.", folder))
    }


    fn chapter(args: &Vec<String>) -> Result<String, Box<dyn Error>> {
        let args = args.into_iter();
        let mut buffer = String::new();
        for chapter in args {
            let filename = chapter.to_filename();
            fs::create_dir_all(format!("src/{}", &filename))?;
            fs::File::create(format!("src/{}.tex", &filename))?;

            buffer.push_str(&format!("\\chapter{{{}}}\n\t\\subimport{{src/}}{{{}}}\n", chapter.from_filename(), filename));
        }

        action::push_to_file(buffer, "index.tex")?;
        Ok(format!("added chapters to index."))
    }

    fn assign(category: &crate::category::Category, args: &Vec<String>) -> Result<String, Box<dyn Error>> { 
        match category {
            crate::category::Category::Chapter     => Self::chapter(args),
            crate::category::Category::Section     => Self::section(args),
            _ => Err("Not a category that can be added.".into()),
        }
    }

}
