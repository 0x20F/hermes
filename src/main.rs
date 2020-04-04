mod config;



use git2::Repository;
use clap::{App, SubCommand, Arg};
use paris::{ log };

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

        let config = if matches.is_present("config") {
            matches.value_of("config").unwrap()
        } else {
            "packages.toml"
        };

        let config = Config::from(config);

        println!("{:?}", config);

        log!("<bright blue>Info</>: directory this was run in: {}", std::env::current_dir().unwrap().display());
        log!("<bright green>Status</>: you just ran the <u>test</u> command");
    };

    /*let url = "https://github.com/0x20F/logger";
    match Repository::clone(url, "repositories/") {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone repo: {}", e)
    };*/
}
