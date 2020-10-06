use clap::{ App, ArgMatches, load_yaml };
use paris::{ log };

use super::config::Config;





pub fn run() -> Result<(), &'static str> {
    let yaml = load_yaml!("app.yml");
    let matches = App::from(yaml).get_matches();

    if !matches.is_present("cover") {
        return Err("You need to use the <bright blue>cover</> command");
    }

    let args = matches.subcommand_matches("cover").unwrap();
    let mut config = get_config(args)?;

    log!("<bright green>Cloning</> {} packages", config.packages.len());

    let fresh = args.is_present("fresh");

    config
        .build_packages(fresh)?
        .execute_scripts();

    Ok(())
}




/// Get the config file, if no parameter is passed it'll
/// choose the default
fn get_config(matches: &ArgMatches) -> Result<Config, &'static str> {
    let mut path = "packages.toml";

    if matches.is_present("config") {
        path = matches.value_of("config").unwrap()
    }

    match Config::from(path) {
        Ok(config) => Ok(config),
        Err(e) => Err(e)
    }
}