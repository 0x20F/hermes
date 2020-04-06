mod config;
mod download;



use clap::{App, Arg};
use paris::{ log, Formatter };

use config::Config;


fn main() -> Result<(), String> {

    let matches = App::new("smoke")
        .subcommand(
            App::new("cover")
                .about("Starts downloading everything")
                .arg(
                    Arg::with_name("config")
                        .short('c')
                        .long("config")
                        .takes_value(true)
                        .value_name("FILE")
                        .help("Path to the config file")
                )
                .arg(
                    Arg::with_name("fresh")
                        .short('f')
                        .help("Remove already downloaded repos and download fresh copies")
                )
        ).get_matches();


    if !matches.is_present("cover") && !matches.is_present("temp") {
        let message = Formatter::colorize_string(
            "You need to use either the <bright blue>cover</> or <bright blue>other</> commands"
        );
        return Err(message);
    }



    if let Some(ref matches) = matches.subcommand_matches("cover") {
        let config_path = if matches.is_present("config") {
            matches.value_of("config").unwrap()
        } else {
            "packages.toml"
        };

        if let Some(config) = Config::from(config_path) {
            // Start parsing somehow
            for (name, mut package) in config.packages {
                package.set_name(name);
                download::package(package, matches.is_present("fresh"))?;
            }
        }
    };

    Ok(())
}
