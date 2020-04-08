mod config;
mod tree;
mod download;

use clap::{App, Arg};
use paris::formatter::colorize_string;
use paris::{ log };
use std::thread;

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




    if !matches.is_present("cover") {
        let message = colorize_string(
            "You need to use either the <bright blue>cover</> command"
        );

        return Err(message);
    }


    let mut threads = vec![];


    if let Some(ref matches) = matches.subcommand_matches("cover") {
        let config_path = if matches.is_present("config") {
            matches.value_of("config").unwrap()
        } else {
            "packages.toml"
        };


        if let Some(config) = Config::from(config_path) {
            log!("<cyan>Cloning</> {} packages", config.packages.len());

            for (name, mut package) in config.packages {
                let fresh = matches.is_present("fresh").clone();

                threads.push(thread::spawn(move || {
                    package.set_name(name);

                    match download::package(package, fresh) {
                        Ok(package_name) => log!("\t<bright green>Finished</> downloading <u>{}</u>", package_name),
                        Err(e) => log!("\t<bright red>Error</> {}", e)
                    }
                }));
            }
        }


        // Wait for all threads to finish before exiting
        for thread in threads {
            let _ = thread.join();
        }
    };

    Ok(())
}
