use config::Config;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod config;

fn main() {
    println!("[VPK cfgy]");

    let path = Path::new("./config.json").canonicalize().unwrap();
    let display = path.display();
    if path.exists() {
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        
        println!("-> Loaded config from '{}'", display);
        let config = Config::create(&s);
    }
    else {
        let config = Config::create_default();
        let mut new_config_file = match File::create(&path) {
            Ok(file) => file,
            Err(_) => panic!("Failed to write config file"),
        };
        new_config_file.write_all(config.to_json().as_bytes()).unwrap();
        println!("-> Wrote default config to '{}'", display);
    }
}
