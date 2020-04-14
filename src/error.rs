use crate::event_output::{Type, Out};



#[derive(Debug)]
pub enum Error {
    Clone,
    Remote,
    Config,
    Save,
}


impl Error {
    pub fn display(&self) {
        let message = match *self {
            Error::Clone => "when cloning a repository",
            _ => ""
        };

        Out::write(Type::Error, message);
    }
}