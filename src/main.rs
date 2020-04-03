use git2::Repository;



fn main() {
    let url = "https://github.com/0x20F/logger";

    match Repository::clone(url, "repositories/") {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone repo: {}", e)
    };
}
