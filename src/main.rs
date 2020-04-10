mod tree;
mod error;
mod config;
mod download;

use clap::{ App, ArgMatches, load_yaml };
use paris::{ log };
use paris::formatter::colorize_string;

use config::Config;
use error::Error;
use std::sync::Arc;
use std::thread;



fn main() -> Result<(), String> {

    let yaml = load_yaml!("app.yml");
    let matches = App::from(yaml).get_matches();


    if !matches.is_present("cover") {
        let message = colorize_string(
            "You need to use the <bright blue>cover</> command"
        );

        return Err(message);
    }



    let matches = matches.subcommand_matches("cover").unwrap();
    let config = get_config(matches)?;
    let mut threads = vec![];
    let mut errors = vec![];

    log!("<cyan>Cloning</> {} packages", config.packages.len());



    for (name, mut package) in config.packages.clone() {
        let fresh = matches.is_present("fresh");
        let config = config.clone();

        threads.push(thread::spawn(move || {
            package.give(name, config);

            package.build(fresh)
        }));
    }

    // Wait for all threads to finish before exiting
    for thread in threads {
        let val = thread.join();

        // Save all returned errors so they can
        // be addressed properly when all threads
        // are finished
        if let Ok(res) = val {
            if let Err(e) = res {
                errors.push(e);
            }
        }
    }

    display_errors(&errors);

    Ok(())
}




/// Get the config file, if no parameter is passed it'll
/// choose the default
fn get_config(matches: &ArgMatches) -> Result<Arc<Config>, String> {
    let mut path = "packages.toml";

    if matches.is_present("config") {
        path = matches.value_of("config").unwrap()
    }

    match Config::from(path) {
        Ok(config) => Ok(Arc::new(config)),
        Err(_) => Err(colorize_string("<bright red>Failed to read config</>"))
    }
}




/// Match the given error types, and output the
/// proper message to the console
fn display_errors(errors: &[Error]) {
    for error in errors {
        error.display();
    }
}