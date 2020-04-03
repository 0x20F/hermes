use git2::Repository;
use clap::{ App, load_yaml };
use paris::{ log };
use serde::{ Deserialize };
use std::fs::read_to_string;
use indexmap::IndexMap;




#[derive(Deserialize)]
struct Config {
    packages: IndexMap<String, Package>
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String
}







fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let file = read_to_string("packages.toml").unwrap();
    let config: Config = toml::from_str(&file).unwrap();

    if let Some(cmd) = matches.subcommand_name() {
        match cmd {
            "cover" => {
                for (_, package) in config.packages.into_iter() {
                    println!("Found package: {}", package.name);
                }

                log!("<bright blue>Info</>: directory this was run in: {}", std::env::current_dir().unwrap().display());
                log!("<bright green>Status</>: you just ran the <u>test</u> command");
            }
            _ => unreachable!()
        }
    };

    /*let url = "https://github.com/0x20F/logger";
    match Repository::clone(url, "repositories/") {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone repo: {}", e)
    };*/
}
