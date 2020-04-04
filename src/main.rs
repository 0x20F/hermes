mod config;



use git2::Repository;
use clap::{App, SubCommand, Arg};
use paris::{ log };
use std::fs::read_to_string;

use config::Config;





fn main() {

    let matches = App::new("smoke")
        .subcommand(
            App::new("cover")
                .about("Starts downloading everything")
                .arg(
                    Arg::with_name("config")
                        .long("config")
                        .takes_value(true)
                        .value_name("FILE")
                        .help("Path to the config file")
                )
        ).get_matches();



    if let Some(ref matches) = matches.subcommand_matches("cover") {
        println!("{:?}", matches.value_of("config"));

        /*let file = read_to_string("packages.toml").unwrap();
        let config: Config = toml::from_str(&file).unwrap();

        for (_, package) in config.packages.into_iter() {
            println!("Found package: {}", package.name);
        }*/

        log!("<bright blue>Info</>: directory this was run in: {}", std::env::current_dir().unwrap().display());
        log!("<bright green>Status</>: you just ran the <u>test</u> command");
    };

    /*let url = "https://github.com/0x20F/logger";
    match Repository::clone(url, "repositories/") {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone repo: {}", e)
    };*/
}
