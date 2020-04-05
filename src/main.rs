mod config;
mod git;



use clap::{App, Arg};
use paris::{ log };

use config::Config;
use git::Git;





fn main() {

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
                    Arg::with_name("override")
                        .short('o')
                        .help("Remove already downloaded repos and download fresh copies")
                )
        ).get_matches();


    if !matches.is_present("cover") && !matches.is_present("temp") {
        log!("You need to use either the <bright blue>cover</> or <bright blue>other</> commands");
        return;
    }



    if let Some(ref matches) = matches.subcommand_matches("cover") {
        let config_path = if matches.is_present("config") {
            matches.value_of("config").unwrap()
        } else {
            "packages.toml"
        };

        if let Some(config) = Config::from(config_path) {
            // Start parsing somehow
            for package in config.packages {
                log!("\t<cyan>Cloning</>: {}", package.1.git);
                Git::clone(package, matches.is_present("override"));
            }
        } else {
            log!("<black><on bright red>No config file was found</>");
            return;
        }


        log!("<bright blue>Info</>: directory this was run in: {}", std::env::current_dir().unwrap().display());
        log!("<bright green>Status</>: you just ran the <u>test</u> command");
    };
}
