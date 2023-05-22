use std::{
    fs,
    error::Error,
    io::Write
};

use super::Choose;
pub struct Add;
impl Choose for Add {
    // fn coursework(args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    //     let _ = args;
    //     Ok(())
    // }
    // fn notes(args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>>{
    //     let _ = args;
    //     Ok(())
    // }
    // fn paper(args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>>{
    //     let _ = args;
    //     Ok(())
    // }
    fn section(mut args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
        let mut buffer = String::new();

        let folder = args.next().ok_or("No folder provided")?;
        let sections: Vec<String> = args.collect();
        if sections.is_empty() {
            return Err("No filename provided".into());
        };

        fs::create_dir_all(&folder)?;
        for section in sections {
            let filename = section.to_ascii_lowercase().replace(" ", "-");
            fs::File::create(format!("{}/{}.tex", folder, filename))?;

            buffer.push_str(&format!("n\\section{{{}}}\n\t\\subimport{{{}}}{{{}}}", section, folder, filename));
        }

        let mut parent_file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("{}.tex", folder))?;
        parent_file.write_all(buffer.as_bytes())?;
        println!("Successfully added sections to {}", folder);
        parent_file.flush()?;
        Ok(())
    }


    fn chapter(args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
        let mut buffer = String::new();
        for chapter in args {
            let filename = chapter.to_ascii_lowercase().replace(" ", "-");
            fs::create_dir_all(format!("src/{}", &filename))?;
            fs::File::create(format!("src/{}.tex", &filename))?;

            buffer.push_str(&format!("\n\\chapter{{{}}}\n\t\\subimport{{src/}}{{{}}}", chapter, filename));
        }

        let mut parent_file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("index.tex")?;
        parent_file.write_all(buffer.as_bytes())?;
        println!("Successfully added chapters to index");
        parent_file.flush()?;
        Ok(())
    }
}


