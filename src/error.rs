use crate::event_output::Type;



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

        Type::Error(message).show();
    }
}