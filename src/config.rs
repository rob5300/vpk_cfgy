use std::{error::Error, io, rc::Rc};

use json::{object, JsonError, JsonValue, array};
use fancy_regex::Regex;

pub struct Config
{
    json: JsonValue,
    pub vpk_path: String,
    pub dir: Rc<String>
}

pub struct VpkEntry
{
    pub regex: Regex,
    pub dir_regex: Regex,
    pub name: String,
    pub args: String
}

fn get_string(json: &JsonValue, key_name: &str) -> String {
    let val = &json[key_name];
    match val.is_null() {
        true => "".to_owned(),
        false => val.to_string()
    }
}

impl Config 
{
    pub fn create_default() -> Config {
        Config{
            json: object!{
                vpk_path: "./bin/vpk.exe",
                dir: "",
                vpks: array![
                    object!{regex: ".*", dir_regex: ".*", name: "all.vpk", args: "-P"},
                    object!{regex: "(_low)$", dir_regex: ".*", name: "low.vpk", args: "-P"}
                ]
            },
            vpk_path: "".to_string(),
            dir: Rc::new("".to_owned()),
        }
    }

    pub fn create(json: &String) -> Result<Config, JsonError> {
        let parsed_obj = json::parse(json)?;
        let vmt_path = parsed_obj["vpk_path"].to_string();

        let dir = get_string(&parsed_obj, "dir_regex");
        Ok(Config { json: parsed_obj, vpk_path: vmt_path, dir: Rc::new(dir) })
    }
}

impl VpkEntry {
    pub fn create(json_value: &JsonValue) -> Result<VpkEntry, Box<dyn Error>> {
        let regex_string = get_string(&json_value, "regex");
        if regex_string.len() == 0 {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "'regex' string was empty or missing")));
        }
        let dir_regex_string = get_string(&json_value, "dir_regex");
        if dir_regex_string.len() == 0 {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "'dir_regex' string was empty or missing")));
        }

        Ok(VpkEntry {
            args: json_value["args"].to_string(),
            name: json_value["name"].to_string(),
            regex: Regex::new(&regex_string)?,
            dir_regex: Regex::new(&dir_regex_string)?
        })
    }
}

impl Config
{
    pub fn get_vpk_entries(&self) -> Result<Vec<VpkEntry>, Box<dyn Error>>{
        let vpks = &self.json["vpks"];
        if vpks.is_array() {
            let mut vpks_vec  = Vec::<VpkEntry>::new();
            let mut index = 0;
            for entry in vpks.members() {
                vpks_vec.push(match VpkEntry::create(entry) {
                    Ok(x) => {x},
                    Err(e) => {return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, format!("Error in vpk entry #{index}: {e}"))));},
                });
                index += 1;
            }

            Ok(vpks_vec)
        }
        else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "vpks value is not an array?").into())
        }
    }

    ///Convert config to json string
    pub fn to_json(&self) -> String {
        self.json.pretty(4)
    }
}