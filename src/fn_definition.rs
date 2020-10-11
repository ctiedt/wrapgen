use regex::{Captures, Regex};
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct FnDefinition<'a> {
    pub name: &'a str,
    pub params: Option<&'a str>,
    pub returns: Option<&'a str>,
}

impl<'a> FnDefinition<'a> {
    pub fn from_str(function: &'a str) -> Result<Self, &str> {
        let re =
            Regex::new(r"fn\s([a-z_0-9]+)\s?\(([a-z_:&*0-9,\s]*)\)\s?(->\s([a-z_:&*0-9\s]*))?;?")
                .unwrap();
        // This one is slightly different from the one used for files - it doesn't require a semicolon

        match re.captures(function) {
            Some(cap) => FnDefinition::from_cap(cap),
            None => Err("Couldn't parse function"),
        }
    }

    pub fn from_cap(cap: Captures<'a>) -> Result<Self, &str> {
        match cap.get(1) {
            Some(name) => Ok(Self {
                name: name.as_str(),
                params: match cap.get(2) {
                    Some(params) => Some(params.as_str()),
                    None => None,
                },
                returns: match cap.get(4) {
                    Some(returns) => Some(returns.as_str()),
                    None => None,
                },
            }),
            None => Err("Functions need to be named"),
        }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_params(&self) -> &str {
        match &self.params {
            Some(params) => params,
            None => "",
        }
    }

    pub fn get_returns(&self) -> &str {
        match &self.returns {
            Some(returns) => returns,
            None => "",
        }
    }

    pub fn get_param_names(&self) -> Vec<&str> {
        match &self.params {
            Some(params) => {
                let mut params_vec = Vec::new();
                for param in params.split_terminator(", ") {
                    let name = param;
                    params_vec.push(name.split(":").next().unwrap());
                }
                params_vec
            }
            None => Vec::new(),
        }
    }
}

impl<'a> Display for FnDefinition<'a> {
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
