use json::{object, JsonError, JsonValue, array};

pub struct Config
{
    json: JsonValue,
}

impl Config 
{
    pub fn create_default() -> Config {
        Config{
            json: object!{
                vmt_path: "./bin/vpk.exe",
                vpks: array![
                    object!{regex: ".*", name: "all.vpk"},
                    object!{regex: "^_+", name: "some.vpk"}
                ]
            }
        }
    }

    pub fn create(json: &String) -> Result<Config, JsonError> {
        let parsed_obj = json::parse(json)?;
        Ok(Config { json: parsed_obj })
    }
}

impl Config
{
    pub fn vmt_path(&self) -> String {
        self.json["vmt_path"].to_string()
    }

    ///Convert config to json string
    pub fn to_json(&self) -> String {
        self.json.pretty(4)
    }
}