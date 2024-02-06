use std::error::Error;

use json::{object, JsonError, JsonValue, array};
use regex::Regex;

pub struct Config
{
    json: JsonValue,
}

pub struct VpkEntry
{
    regex: Regex,
    name: String,
    args: String
}

impl Config 
{
    pub fn create_default() -> Config {
        Config{
            json: object!{
                vmt_path: "./bin/vpk.exe",
                vpks: array![
                    object!{regex: ".*", name: "all.vpk", args: "-P"},
                    object!{regex: "^_+", name: "some.vpk", args: "-P"}
                ]
            }
        }
    }

    pub fn create(json: &String) -> Result<Config, JsonError> {
        let parsed_obj = json::parse(json)?;
        Ok(Config { json: parsed_obj })
    }
}

impl VpkEntry {
    pub fn create(json_value: &JsonValue) -> VpkEntry {
        VpkEntry {
            args: json_value["args"].to_string(),
            name: json_value["name"].to_string(),
            regex: Regex::new(&json_value["regex"].to_string()).unwrap(),
        }
    }
}

impl Config
{
    pub fn vmt_path(&self) -> String {
        self.json["vmt_path"].to_string()
    }

    pub fn get_vpk_entries(&self) -> Result<Vec<VpkEntry>, Box<dyn Error>>{
        let vpks = &self.json["vpks"];
        if vpks.is_array() {
            let mut vpks_vec  = Vec::<VpkEntry>::new();
            for entry in vpks.members() {
                vpks_vec.push(VpkEntry::create(entry));
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