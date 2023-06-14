use std::{
    io,
    fs, process,
};

pub struct ConfigMetadata {
    pub config_file: String, 
    pub templates_path: String,
}

pub struct Config {
    pub metadata: ConfigMetadata,
}

impl Config {
    pub fn new(metadata: ConfigMetadata) -> Self {
        Self {metadata}
    }
}

impl ConfigMetadata {
    pub fn new(root: &str) -> Result<Self, io::Error> {
        let config_path = format!("{root}.config/tm/");
        let config_files = [
            format!("{config_path}config"), format!("{root}.tm.conf")
        ];
        let template_folders = [
            format!("{config_path}templates/"), format!("{root}Templates/")
        ];

        // Ask for permission only once before creating the config file
        if Self::get_path(&config_files).is_none() {Self::create_config_file(&config_path)?};
        if Self::get_path(&template_folders).is_none() {Self::create_template_folders(&template_folders[0])?};

        Ok(Self { 
            config_file: format!("{config_path}config"),
            templates_path: format!("{config_path}templates/")
        })
    }

    fn get_path(paths: &[String]) -> Option<&String> {
        paths.iter()
            // .map(|filepath| PathBuf::from(filepath))
            .find(|path| fs::metadata(path)
                  .map(|metadata| metadata.is_file() | metadata.is_dir())
                  .unwrap_or(false))
    }

    fn create_template_folders(template_route: &str) -> Result<(), io::Error> {
        for category in ["coursework", "notes", "paper"] { 
            fs::create_dir_all(format!("{template_route}{category}"))?;
        };
        Ok(())
    }

    fn create_config_file(config_path: &str) -> Result<(), io::Error>{
        println!("No configuration files found. New configuration files will be created in {}.", config_path);
        for _ in 0..2 {
            println!("Would you like to proceed? (y/n)");

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            match input.trim().to_lowercase().as_str() {
                "y" => {
                    fs::create_dir_all(config_path)?;
                    fs::File::create(format!("{config_path}config")).expect("Unable to create config file");
                    break
                },
                "n" => process::exit(0),
                _ => continue
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod test {
    // use super::*;
    #[test]
    fn load_configs() {
        // let _ = super::load_configs();
    }
    #[test]
    fn serde_config() {
    }
}
