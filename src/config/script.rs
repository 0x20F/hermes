use serde::Deserialize;
use paris::log;
use shells::execute_with;
use key_list::KeyList;
use crate::config::Package;



#[derive(Clone, Debug, Deserialize)]
pub struct Script {
    script: String
}


impl Script {
    pub fn exec(&self, package: &Package) {
        log!("<magenta>Running</> script");

        let (_, stdout, _) = execute_with("sh", &self.prepare_script(package));
        println!("{}", stdout);
    }


    fn prepare_script(&self, package: &Package) -> String {
        let cmd = &self.script;
        let keys = KeyList::new(cmd, '{', '}');
        let mut res = String::from(cmd);

        for key in keys {
            let clean = key.replace(&['{', '}'][..], "");

            match clean.trim() {
                "directory" => res = res.replace(&key, &package.full_path()),
                "file" => res = res.replace(&key, &package.filename()),
                "name" => res = res.replace(&key, &package.name),

                _ => () // Key shouldn't be replaced if not defined
            }
        }

        res
    }
}