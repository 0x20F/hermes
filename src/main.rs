mod tree;
mod config;
mod download;
mod output;
mod hermes;

use paris::{ log, error };



fn main() {
    match hermes::run() {
        Ok(_) => log!("Everything is set up!"),
        Err(msg) => error!("{}", msg)
    }
}
