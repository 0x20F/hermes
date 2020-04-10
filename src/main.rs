mod tree;
mod error;
mod config;
mod download;

use clap::{App, Arg};
use paris::{ log };
use paris::formatter::colorize_string;

use config::Config;
use std::sync::Arc;
use std::thread;



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




    if !matches.is_present("cover") {
        let message = colorize_string(
            "You need to use the <bright blue>cover</> command"
        );

        return Err(message);
    }


    if let Some(ref matches) = matches.subcommand_matches("cover") {
        let config_path = if matches.is_present("config") {
            matches.value_of("config").unwrap()
        } else {
            "packages.toml"
        };


        let config = match Config::from(config_path) {
            Ok(config) => Arc::new(config),
            Err(_) => return Err(colorize_string("<bright red>Failed to read config</>"))
        };

        let mut threads = vec![];



        log!("<cyan>Cloning</> {} packages", config.packages.len());

        for (name, mut package) in config.packages.clone() {
            let fresh = matches.is_present("fresh").clone();
            let config = config.clone();

            threads.push(thread::spawn(move || {
                package.give(name, config);

                package.build(fresh)
            }));
        }



        // Wait for all threads to finish before exiting
        for thread in threads {
            let val = thread.join();
            println!("{:?}", val);
        }


    };

    Ok(())
}
