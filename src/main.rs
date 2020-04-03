use git2::Repository;
use clap::{ App, load_yaml };




fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();


    if let Some(cmd) = matches.subcommand_name() {
        match cmd {
            "test" => println!("You just ran the test command!"),
            _ => println!("Whatever you just ran was wrong!")
        }
    };

    /*let url = "https://github.com/0x20F/logger";
    match Repository::clone(url, "repositories/") {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone repo: {}", e)
    };*/
}
