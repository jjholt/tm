use std::error::Error;

use crate::action::Action;
use crate::behaviour::ToBehaviour;
use crate::category::ToCategory;

pub struct Command {
    action: Action,
    target: Vec<String>,
}


impl Command {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next(); //Program name

        let action = match args.next() {
            Some(action) => {
                let mut arg = action.chars();
                Action {
                    behaviour: arg.next()
                        .ok_or("Missing behaviour.")?
                        .to_behaviour()?,
                    category: arg.next()
                        .ok_or("Missing Category.")? 
                        .to_category()?,
                }
            }
            None => return Err("Missing command. It should include one character for behaviour and one character for category."),
        };

        let target: Vec<String> = args.collect();
        if target.is_empty() {
            return Err("Missing file name.");
        };

        Ok(Self{action, target})
    }

    pub fn apply(&self, templates_path: &str) -> Result<String, Box<dyn Error>> {
        self.action.apply(&self.target, templates_path)
    }

}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn take_input() -> Result<(), Box<dyn Error>> {
        let user_input = ["Program name".to_string(), "nn".to_string(), "My Coursework".to_string()];
        Command::build(user_input.into_iter())?;
        Ok(())
    }
}
