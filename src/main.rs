mod tree;
mod config;
mod download;
mod output;
mod inject;

use clap::{ App, ArgMatches, load_yaml };
use paris::formatter::colorize_string;
use paris::{ log, error, info };

use config::Config;



fn main() -> Result<(), &'static str> {
    let yaml = load_yaml!("app.yml");
    let matches = App::from(yaml).get_matches();

    if !matches.is_present("cover") {
        error!("You need to use the <bright blue>cover</> command");
        return Ok(());
    }

    let args = matches.subcommand_matches("cover").unwrap();
    let config = get_config(args)?;

    info!("<bright green>Cloning</> {} packages", config.packages.len());

    let fresh = args.is_present("fresh");
    let packages = config.build_packages(fresh);

    config.execute_scripts(packages);
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
