use std::fmt;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Token{
    config: Config
}

#[derive(Deserialize)]
struct Config {
    id: u16,
    data: Vec<Vec<String>>,
}

impl Token {
    // pub fn new() -> Result<Self, toml::de::Error> {
    //     let config_text = fs::read_to_string("./data/config.toml").expect("Token: error reading file");
    //     let config = toml::from_str(&config_text)?;
    //     Ok(Token { config })
    // }

    pub fn debug(&self) {
        print!("{}", self);
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token {{\n")?;
        write!(f, "\tid: {}\n", self.config.id)?;
        for d in &self.config.data {
            write!(f, "\tdata: {:?}\n", d)?;
        }
        write!(f, "}}\n")?;
        Ok(())
    }
}
