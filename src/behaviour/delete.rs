use std::{error::Error, unimplemented};

use crate::action::Category;

pub fn choose(category: Category, mut args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    unimplemented!()
}
