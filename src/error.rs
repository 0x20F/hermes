use paris::{ log };



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

        log!("<bright red>Error</> {}", message);
    }
}