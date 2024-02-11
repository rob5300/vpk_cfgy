use config::Config;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;
use std::env;
use std::process::Command;
use std::process::ExitStatus;
use walkdir::WalkDir;
use tempfile::{self, NamedTempFile};
use once_cell::sync::Lazy;

mod config;

static IGNORED_EXTENSIONS: Lazy<Vec<&str>> = Lazy::new(|| { vec!("exe", "vpk", "json") });

fn main() {
    println!("[VPK cfgy] by rob5300");

    let args: Vec<String> = env::args().collect();
    let exe_path = Path::new(&args[0]).parent().unwrap();
    let mut working_path = match args.len() >= 2 {
        true => Path::new(&args[1]),
        false => Path::new(".")
    };

    let config_path = exe_path.join("config.json");
    if config_path.exists() {
        let mut file = match File::open(&config_path) {
            Err(why) => panic!("Failed to open config at {}: {}", config_path.canonicalize().unwrap().display(), why),
            Ok(file) => file,
        };
        
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        
        println!("-> Loaded config from '{}'", config_path.canonicalize().unwrap().display());
        let config = Config::create(&s).unwrap();

        if config.dir.as_ref().len() > 0 {
            working_path = Path::new(config.dir.as_ref());
        }

        let vmt_executable_path = Path::new(&config.vpk_path);
        if !vmt_executable_path.exists() {
            panic!("The value for 'vmt_path' in the config does not exist");
        }

        if cfg!(target_os = "windows") && vmt_executable_path.extension().unwrap() != "exe" {
            panic!("vmt path is not to an executable?");
        }

        match process_vpk_entries(&config, &working_path) {
            Ok(_) => {},
            Err(e) => {println!("{e}"); return;},
        }
    }
    else {
        let config = Config::create_default();
        let mut new_config_file = match File::create(&config_path) {
            Ok(file) => file,
            Err(_) => panic!("Failed to write config file"),
        };
        new_config_file.write_all(config.to_json().as_bytes()).unwrap();
        println!("-> Wrote default config to '{}'", config_path.canonicalize().unwrap().display());
    }
}

fn process_vpk_entries(config: &Config, working_dir: &Path) -> Result<(), Box<dyn Error>>
{
    println!("-> Working dir will be '{}'", working_dir.display());

    let vpk_entries = config.get_vpk_entries()?;
    let mut entry_num = 0;
    for entry in vpk_entries {
        //Filter files that match this vpk entry regex
        let regex_str = entry.regex.as_str();
        println!("#{} Finding files for vpk entry with regex '{}' ...", entry_num, &regex_str);
        let matching_files: Vec<String> = WalkDir::new(working_dir).into_iter().filter_map(|f| {
            let dir_entry = f.unwrap();
            let path = dir_entry.path();
            
            //Ignore some file extensions
            let extension = path.extension()?.to_str()?;
            if IGNORED_EXTENSIONS.contains(&extension) {
                return None;
            }
            
            let relative_path = path.strip_prefix(working_dir).unwrap();

            //Does this file name and path match the configured expressions?
            if path.is_file() && entry.regex.is_match(path.file_name()?.to_str()?) && entry.dir_regex.is_match(relative_path.parent().unwrap().to_str()?) {
                let new_path_arg = relative_path.to_str()?.to_owned();
                Some(new_path_arg)
            }
            else {
                None
            }
        }).collect();

        println!("Found {} files", matching_files.len());

        //Skip when 0 files match
        if matching_files.len() == 0 {
            continue;
        }

        //Build arg list first with user arg strin
        let vpk_name = entry.name;
        if cfg!(debug_assertions) {
            println!("[Debug] VPK Name: '{}'.", vpk_name)
        }

        let mut args = Vec::<String>::new();

        args.push("-v".to_owned());

        //Add user args if any
        if entry.args.len() > 0 {
            let split_args = entry.args.split(" ");
            for arg in split_args {
                args.push(arg.to_string());
            }
        }

        args.push("a".to_owned());
        args.push(vpk_name.clone());

        //Add response file arg containing the filtered file paths
        let response_file = get_files_response_file(&matching_files)?;
        let response_file_tmp_path = response_file.into_temp_path();
        let response_file_path_str = response_file_tmp_path.to_str().ok_or("Failed to get path for temp response file")?;

        if cfg!(debug_assertions) {
            println!("[Debug] Temp response file location is '{response_file_path_str}'.")
        }

        let response_file_arg = "@".to_owned() + response_file_path_str;
        args.push(response_file_arg);

        if cfg!(debug_assertions) {
            println!("[Debug] Args: '{}'.", args.join(" "));
        }

        match execute_vpk(config, working_dir, args) {
            Ok(exit_status) => {
                if exit_status.success() {
                    println!("VPK created with name '{}'", &vpk_name);
                }
            },
            Err(e) => {println!("⚠️ Failed to execute vpk: {e}")},
        };
        entry_num += 1;
    }

    Ok(())
}

fn execute_vpk(config: &Config, working_dir: &Path, args: Vec<String>) -> Result<ExitStatus, Box<dyn Error>> {
    println!("-- 🚀 Executing VPK --");
    let mut command = Command::new(&config.vpk_path)
    .current_dir(working_dir)
    .args(args)
    .spawn()?;

    let exit_status = command.wait()?;
    if exit_status.success() {
        println!("-- ✅ VPK Completed Successfully --");
    }
    else {
        println!("-- ❌ VPK Failed to complete --");
    }
    Ok(exit_status)
}

fn get_files_response_file(paths: &Vec<String>) -> Result<NamedTempFile, Box<dyn Error>>
{
    let mut file = NamedTempFile::new()?;
    for path in paths {
        let line = path.to_owned() + "\n";
        file.write(&line.as_bytes())?;
    }

    Ok(file)
}