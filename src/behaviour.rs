/// Used to convert from a `char` to the appropriate behaviour.
pub trait ToBehaviour {
    fn to_behaviour(self) -> Result<Behaviour, &'static str>;
}

pub enum Behaviour {
    New,
    Delete,
    Add,
}

impl ToBehaviour for char {
    fn to_behaviour(self) -> Result<Behaviour, &'static str> {
        Ok(match self {
            'n' => Behaviour::New,
            'd' => Behaviour::Delete,
            'a' => Behaviour::Add,
            _ => return Err("Invalid behaviour") 
        })
    }
}

