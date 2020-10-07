use clap::{ App, ArgMatches, load_yaml };
use paris::{ log };

use super::config::Config;
use std::fs::read_to_string;


pub fn run() -> Result<(), &'static str> {
    let yaml = load_yaml!("app.yml");
    let matches = App::from(yaml).get_matches();

    if !matches.is_present("cover") {
        return Err("You need to use the <bright blue>cover</> command");
    }

    let args = matches.subcommand_matches("cover").unwrap();
    let mut config = load_config(args)?;

    log!("<bright green>Cloning</> {} packages", config.packages.len());

    config
        .load_fresh(args.is_present("fresh"))
        .build_packages()?;

    Ok(())
}




/// Get the config file, if no parameter is passed it'll
/// choose the default
fn find_config(matches: &ArgMatches) -> String {
    let mut path = "packages.toml";

    if matches.is_present("config") {
        path = matches.value_of("config").unwrap()
    }

    path.to_string()
}


fn load_config(matches: &ArgMatches) -> Result<Config, &'static str> {
    let path = find_config(matches);
    let file = read_to_string(path);

    if file.is_err() {
        return Err("Could not find the given configuration file"); // TODO: Better message
    }

    match toml::from_str::<Config>(&file.unwrap()) {
        Ok(c) => Ok(c),
        Err(_) => Err("Could not parse the config file") // TODO: Better message
    }
}