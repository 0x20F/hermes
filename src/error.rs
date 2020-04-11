use paris::{ log };



#[derive(Debug)]
pub enum Error {
    Clone,
    Remote,
    Config,
    Save,
    NoScripts
}


impl Error {
    pub fn display(&self) {
        let message = match *self {
            Error::Clone => "when cloning a repository",
            Error::NoScripts => "you have no defined scripts in your config",
            _ => ""
        };

        log!("<bright red>Error</> {}", message);
    }
}