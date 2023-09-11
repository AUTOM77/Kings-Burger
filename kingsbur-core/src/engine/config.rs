use serde::Deserialize;
use std::{fmt, fs};
use toml;

#[derive(Deserialize)]
pub struct Config {
    token: Token,
}

#[derive(Deserialize)]
struct Token {
    conn: String,
    frame: String,
    data: Vec<Vec<String>>,
    roll: Vec<String>,
}

impl Config {
    pub fn new() -> Result<Self, toml::de::Error> {
        let _str = fs::read_to_string("./data/config.toml").expect("Token: error reading file");
        let _conf = toml::from_str(&_str)?;
        Ok(_conf)
    }

    pub fn conn(&self) -> &str {
        &self.token.conn
    }

    pub fn frame(&self) -> &str {
        &self.token.frame
    }

    pub fn get_data(&self) -> &Vec<Vec<String>> {
        &self.token.data
    }
    
    pub fn get_roll(&self) -> &Vec<String> {
        &self.token.roll
    }

    pub fn debug(&self) {
        print!("{}", self);
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Config {{\n")?;
        write!(f, "\t conn: {}\n", self.token.conn)?;
        for d in &self.token.data {
            write!(f, "\t data: {:?}\n", d)?;
        }
        write!(f, "}}\n")?;
        Ok(())
    }
}
