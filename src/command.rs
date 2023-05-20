use std::error::Error;

use crate::behaviour::{add, delete, new};
use crate::action::{Category, Behaviour, Action};

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
                    behaviour: Behaviour::from(arg.next().unwrap()).unwrap(),
                    category: Category::from(arg.next().unwrap()).unwrap(),
                }
            }
            None => return Err("Missing command."),
        };

        let target: Vec<String> = args.collect();
        if target.is_empty() {
            return Err("Missing file name");
        };

        Ok(Self{action, target})
    }

    pub fn apply(self) -> Result<(), Box<dyn Error>> {
        match self.action.behaviour {
            Behaviour::New => new::choose(
                self.action.category, self.target.into_iter()
            ),
            Behaviour::Delete => delete::choose(
                self.action.category, self.target.into_iter()
            ),
            Behaviour::Add => add::choose(
                self.action.category, self.target.into_iter()
            ),
        }
    }

}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn take_input() {
        let user_input = ["Program name".to_string(), "nn".to_string(), "My Coursework".to_string()];
        Command::build(user_input.into_iter());
    }
}
