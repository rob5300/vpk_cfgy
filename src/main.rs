use config::Config;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fs;
use std::process::Command;

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
        let config = Config::create(&s).unwrap();

        let vmt_executable_path = Path::new(&config.vmt_path);
        if !vmt_executable_path.exists() {
            panic!("The value for 'vmt_path' in the config does not exist");
        }

        if cfg!(target_os = "windows") && vmt_executable_path.extension().unwrap() != "exe" {
            
        }

        match process_vpk_entries(&config, ".".to_string()) {
            Ok(_) => {},
            Err(e) => {println!("Failed to process vpk entries and execute vpk executable: {e}")},
        }
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

fn process_vpk_entries(config: &Config, working_dir: String) -> Result<(), Box<dyn Error>>
{
    let working_dir_path = Path::new(&working_dir);
    println!("Working dir: '{}'", working_dir_path.display());

    let vpk_entries = config.get_vpk_entries()?;
    for entry in vpk_entries {
        //Filter files that match this vpk entry regex
        let mut matching_files: Vec<String> = fs::read_dir(working_dir_path)?.filter_map(|f| {
            let path = f.ok()?.path();
            if path.is_file() && entry.regex.is_match(path.file_name()?.to_str()?) {
                Some(path.to_str()?.to_string())
            }
            else {
                None
            }
        }).collect();

        let mut args: Vec<String> = entry.args.split(" ").map(|s| s.to_string()).collect();
        args.append(&mut matching_files);
        Command::new(&config.vmt_path).args(args).output()?;
    }

    Ok(())
}