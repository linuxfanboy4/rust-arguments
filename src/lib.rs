use std::collections::HashMap;                             use std::sync::Arc;
use std::fmt;

#[derive(Clone)]
pub struct Arg {
    pub name: String,
    pub short: Option<char>,
    pub long: Option<String>,
    pub takes_value: bool,
    pub required: bool,
    pub default: Option<String>,
    pub validator: Option<Arc<dyn Fn(&str) -> bool + Send + Sync>>,
}

impl fmt::Debug for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Arg")
            .field("name", &self.name)
            .field("short", &self.short)
            .field("long", &self.long)
            .field("takes_value", &self.takes_value)
            .field("required", &self.required)
            .field("default", &self.default)
            .finish()
    }
}

#[derive(Debug)]
pub struct ArgMatches {
    pub values: HashMap<String, String>,
    pub flags: HashMap<String, bool>,
    pub positionals: Vec<String>,
}

pub struct ArgParser {
    args: Vec<Arg>,
    subcommands: HashMap<String, ArgParser>,
}

impl ArgParser {
    pub fn new() -> Self {
        Self {
            args: Vec::new(),
            subcommands: HashMap::new(),
        }
    }

    pub fn arg(mut self, name: &str) -> Self {
        self.args.push(Arg {
            name: name.to_string(),
            short: None,
            long: None,
            takes_value: false,
            required: false,
            default: None,
            validator: None,
        });
        self
    }

    pub fn short(mut self, name: &str, short: char) -> Self {
        if let Some(arg) = self.args.iter_mut().find(|a| a.name == name) {
            arg.short = Some(short);
        }
        self
    }

    pub fn long(mut self, name: &str, long: &str) -> Self {
        if let Some(arg) = self.args.iter_mut().find(|a| a.name == name) {
            arg.long = Some(long.to_string());
        }
        self
    }

    pub fn takes_value(mut self, name: &str) -> Self {
        if let Some(arg) = self.args.iter_mut().find(|a| a.name == name) {
            arg.takes_value = true;
        }
        self
    }

    pub fn required(mut self, name: &str) -> Self {
        if let Some(arg) = self.args.iter_mut().find(|a| a.name == name) {
            arg.required = true;
        }
        self
    }

    pub fn default(mut self, name: &str, default: &str) -> Self {
        if let Some(arg) = self.args.iter_mut().find(|a| a.name == name) {
            arg.default = Some(default.to_string());
        }
        self
    }

    pub fn validator<F>(mut self, name: &str, validator: F) -> Self
    where
        F: 'static + Fn(&str) -> bool + Send + Sync,
    {
        if let Some(arg) = self.args.iter_mut().find(|a| a.name == name) {
            arg.validator = Some(Arc::new(validator));
        }
        self
    }

    pub fn subcommand(mut self, name: &str, parser: ArgParser) -> Self {
        self.subcommands.insert(name.to_string(), parser);
        self
    }

    pub fn parse(mut self, args: &[String]) -> ArgMatches {
        let mut values = HashMap::new();
        let mut flags = HashMap::new();
        let mut positionals = Vec::new();
        let mut iter = args.iter().skip(1).peekable();

        while let Some(arg) = iter.next() {
            if arg.starts_with("--") {
                let name = &arg[2..];
                if let Some(a) = self.args.iter().find(|a| a.long.as_deref() == Some(name)) {
                    if a.takes_value {
                        if let Some(value) = iter.next() {
                            if let Some(validator) = &a.validator {
                                if !validator(value) {
                                    panic!("Invalid value for argument: {}", name);
                                }
                            }
                            values.insert(a.name.clone(), value.clone());
                        }
                    } else {
                        flags.insert(a.name.clone(), true);
                    }
                }
            } else if arg.starts_with('-') {
                let chars: Vec<char> = arg.chars().skip(1).collect();
                for &c in &chars {
                    if let Some(a) = self.args.iter().find(|a| a.short == Some(c)) {
                        if a.takes_value {
                            if let Some(value) = iter.next() {
                                if let Some(validator) = &a.validator {
                                    if !validator(value) {
                                        panic!("Invalid value for argument: -{}", c);
                                    }
                                }
                                values.insert(a.name.clone(), value.clone());
                            }
                        } else {
                            flags.insert(a.name.clone(), true);
                        }
                    }
                }
            } else if self.subcommands.contains_key(arg) {
                let sub = self.subcommands.remove(arg).unwrap();
                return sub.parse(&args[1..]);
            } else {
                positionals.push(arg.clone());
            }
        }

        for arg in &self.args {
            if arg.required && !values.contains_key(&arg.name) {
                if let Some(default) = &arg.default {
                    values.insert(arg.name.clone(), default.clone());
                } else {
                    panic!("Missing required argument: {}", arg.name);
                }
            }
        }

        ArgMatches {
            values,
            flags,
            positionals,
        }
    }
}
