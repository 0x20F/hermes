mod tree;
mod error;
mod config;
mod download;
mod output;

use clap::{ App, ArgMatches, load_yaml };
use paris::formatter::colorize_string;
use paris::{ log, error, info };

use crate::output::{Type, Out };
use config::Config;



fn main() -> Result<(), String> {
    let yaml = load_yaml!("app.yml");
    let matches = App::from(yaml).get_matches();

    if !matches.is_present("cover") {
        error!("You need to use the <bright blue>cover</> command");
        return Ok(());
    }


    let matches = matches.subcommand_matches("cover").unwrap();
    let config = get_config(matches)?;

    info!("<bright green>Cloning</> {} packages", config.packages.len());

    let packages = config.build_packages(matches.is_present("fresh"));
    config.execute_scripts(packages);

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
        Err(_) => Err(String::new())
    }
}
