use regex::{Captures, Regex};
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct FnDefinition {
    pub name: String,
    pub params: Option<String>,
    pub returns: Option<String>,
}

impl FnDefinition {
    pub fn from_str(function: &str) -> Result<Self, &str> {
        let re =
            Regex::new(r"fn\s([a-z_0-9]+)\s?\(([a-z_:&*0-9,\s]*)\)\s?(\s->\s([a-z_:&*0-9\s]*))?;?")
                .unwrap();
        // This one is slightly different from the one used for files - it doesn't require a semicolon

        match re.captures(function) {
            Some(cap) => FnDefinition::from_cap(cap),
            None => Err("Couldn't parse function"),
        }
    }

    pub fn from_cap(cap: Captures) -> Result<Self, &str> {
        match cap.get(1) {
            Some(name) => Ok(Self {
                name: String::from(name.as_str()),
                params: match cap.get(2) {
                    Some(params) => Some(String::from(params.as_str())),
                    None => None,
                },
                returns: match cap.get(4) {
                    Some(returns) => Some(String::from(returns.as_str())),
                    None => None,
                },
            }),
            None => Err("Functions need to be named"),
        }
    }

    pub fn get_name(&self) -> String {
        String::from(self.name.as_str())
    }

    pub fn get_params(&self) -> String {
        String::from(match &self.params {
            Some(params) => params.as_str(),
            None => "",
        })
    }

    pub fn get_returns(&self) -> String {
        String::from(match &self.returns {
            Some(returns) => returns.as_str(),
            None => "",
        })
    }

    pub fn get_param_names(&self) -> Vec<String> {
        match &self.params {
            Some(params) => {
                let mut params_vec = Vec::new();
                for param in params.split_terminator(", ") {
                    let name = String::from(param);
                    params_vec.push(String::from(name.split(":").next().unwrap()));
                }
                params_vec
            }
            None => Vec::new(),
        }
    }
}

impl Display for FnDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.returns {
            Some(_) => write!(
                f,
                "fn {}({}) -> {};",
                self.get_name(),
                self.get_params(),
                self.get_returns()
            ),
            None => write!(f, "fn {}({});", self.get_name(), self.get_params(),),
        }
    }
}
