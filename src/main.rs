mod tree;
mod error;
mod config;
mod download;

use clap::{ App, ArgMatches, load_yaml };
use paris::formatter::colorize_string;

use config::{ Package, Config };
use std::thread;
use crate::error::Error;


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

    let packages = build_packages(&config, matches.is_present("fresh"));

    if let Some(scripts) = &config.scripts {
        for package in packages {
            package.exec(scripts).unwrap();
        }
    } else {
        // For now
        Error::NoScripts.display();
    }

    Ok(())
}




/// Get the config file, if no parameter is passed it'll
/// choose the default
fn get_config(matches: &ArgMatches) -> Result<Config, String> {
    let mut path = "packages.toml";

    if matches.is_present("config") {
        path = matches.value_of("config").unwrap()
    }

    match Config::from(path) {
        Ok(config) => Ok(config),
        Err(_) => Err(colorize_string("<bright red>Failed to read config</>"))
    }
}




fn build_packages(config: &Config, fresh: bool) -> Vec<Package> {
    let mut threads = vec![];
    let mut survivors = vec![];

    for (name, mut package) in config.packages.clone() {
        threads.push(thread::spawn(move || {
            package.give(name);

            // If it's not an error, give back the
            // package so others can use it
            match package.build(fresh) {
                Ok(_) => Ok(package),
                Err(e) => Err(e)
            }
        }));
    }

    // Wait for all threads to finish before exiting
    for thread in threads {
        // If thread didn't die, display error or save package
        if let Ok(res) = thread.join() {
            match res {
                Ok(p) => survivors.push(p),
                Err(e) => e.display()
            }
        }
    }

    survivors
}