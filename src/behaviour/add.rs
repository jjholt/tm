use std::{
    fs,
    error::Error,
    io::Write
};
use crate::action::Category;

pub fn choose(category: Category, mut args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    match category {
        Category::Coursework => coursework(args),
        Category::Notes => notes(args),
        Category::Paper => paper(args),
        Category::Chapter => chapter(args),
        Category::Section => section(args),
    }
}

pub fn coursework(mut args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {unimplemented!()}
pub fn notes(mut args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>>{unimplemented!()}
pub fn paper(mut args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>>{unimplemented!()}

pub fn section(mut args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    args.next(); //discard program name

    let section = args.next().ok_or("No filename provided")?;
    let folder = args.next().ok_or("No folder provided")?;
    let filename = section.to_ascii_lowercase().replace(" ", "-");

    if args.next().is_some() {
        return Err("Too many arguments".into())
    }

    fs::create_dir_all(&folder)?;
    fs::File::create(format!("{}/{}.tex", folder, filename))?;
    let mut parent_file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("{}.tex", folder))?;
    parent_file.write_all(format!("\\section{{{}}}\n\t\\subimport{{{}}}{{{}}}", section, folder, filename).as_bytes())?;
    println!("Successfully added {} to {}", section, folder);
    parent_file.flush()?;
    Ok(())
}


pub fn chapter(mut args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    args.next(); //discard program name

    let mut buffer = String::new();
    for chapter in args {
        let filename = chapter.to_ascii_lowercase().replace(" ", "-");
        fs::create_dir_all(format!("src/{}", &filename))?;
        fs::File::create(format!("src/{}.tex", &filename))?;

        buffer.push_str(&format!("\\chapter{{{}}}\n\t\\subimport{{src/}}{{{}}}\n", chapter, filename));
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
