use paris::formatter::colorize_string;



#[derive(Debug)]
pub enum Error {
    Clone,
    Remote,
    Config,
    Save
}


impl Error {
    pub fn display(&self) {
        let begin = colorize_string("<bright red>Error</>");

        let message = match *self {
            Error::Clone => "when cloning a repository",
            _ => ""
        };

        println!("{} {}", begin, message);
    }
}