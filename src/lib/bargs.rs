// Arguments parser functionallity
use phf::phf_map;
use std::collections::HashMap;
use std::io;
use std::io::IsTerminal;
use std::matches;

// Actions
#[derive(Debug, Clone, PartialEq)]
pub enum BAction {
    Add,
    List,
    ListDefault, // when it's just the default no args passed
    Remove,
    Help,
    Version,
}
// a list of first arg options enum
static GLOBAL_ACTIONS: [&str; 5] = ["add", "list", "remove", "help", "version"];

// flag map to actions
static GLOBAL_ACTION_ARGS_MAP: phf::Map<&'static str, BAction> = phf_map! {
    "a" => BAction::Add,
    "add" => BAction::Add,
    "l" => BAction::List,
    "list" => BAction::List,
    "r" => BAction::Remove,
    "remove" => BAction::Remove,
    "h" => BAction::Help,
    "help" => BAction::Help,
    "v" => BAction::Version,
    "version" => BAction::Version,
    "V" => BAction::Version,
};

fn get_action_from_flag(flag: &str) -> Option<BAction> {
    if !GLOBAL_ACTION_ARGS_MAP.contains_key(flag) {
        return None;
    }
    GLOBAL_ACTION_ARGS_MAP.get(flag).cloned()
}

// flags map to options flag and weather or not it takes an argument
static GLOBAL_ACTION_FLAGS_MAP: phf::Map<&'static str, (&'static str, bool)> = phf_map! {
    "o" => ("output", true),
    "output" => ("output", true),
    "f" => ("format", true),
    "format" => ("format", true),
};

fn get_flag_from_flag(flag: &str) -> Option<(&str, bool)> {
    if !GLOBAL_ACTION_FLAGS_MAP.contains_key(flag) {
        return None;
    }
    GLOBAL_ACTION_FLAGS_MAP.get(flag).cloned()
}

pub trait BJournRunner {
    fn parse(args: Vec<String>, input_txt: Option<String>) -> Self;
    fn has_flag(&self, flag: &str) -> bool;
    fn flag_arg(&self, flag: &str) -> Option<String>;
}

#[derive(Debug)]
pub struct BArgs {
    args: Vec<String>,
    pub action: BAction,
    pub flags: HashMap<String, bool>,
    pub flag_args: HashMap<String, String>,
    pub input: Option<String>,
}

impl BJournRunner for BArgs {
    fn parse(args: Vec<String>, input_txt: Option<String>) -> Self {
        let mut flags: HashMap<String, bool> = HashMap::new();
        let mut flag_args = HashMap::new();
        let mut action: Option<BAction> = None;
        let mut input = input_txt.map(|txt| txt.trim().to_string());

        // quick short if nothing supplied
        if args.len() == 1 && input.is_none() {
            return BArgs {
                args,
                action: BAction::ListDefault,
                flags,
                flag_args,
                input,
            };
        }

        let mut skip_next = false;
        for (i, arg) in args.iter().enumerate() {
            // skip the first arg as it is the program name
            if i == 0 {
                continue;
            }

            // skip next arg
            if skip_next {
                skip_next = false;
                continue;
            }

            // we allow args before or after the action flag, or anywhere in the command really
            if arg.starts_with("--") && !arg.contains(' ') {
                // check the flag is valid, otherwise print a warning
                // we check for the space in case someone has added dashes in their note at the beginning
                let mut ffound = false;
                let flag = arg.replace("--", "");
                if let Some(a) = get_action_from_flag(&flag) {
                    action = Some(a);
                    ffound = true;
                }
                if let Some((flag_item, takes_arg)) = get_flag_from_flag(&flag) {
                    if takes_arg {
                        if i < args.len() {
                            let next_arg = &args[i + 1];
                            if !next_arg.starts_with("-") {
                                flag_args.insert(flag_item.to_string(), next_arg.to_string());
                                skip_next = true;
                            } else {
                                eprintln!("Warning: Flag {} requires an argument", flag);
                            }
                        }
                    } else {
                        flags.insert(flag_item.to_string(), true);
                    }
                    ffound = true;
                }

                if !ffound {
                    eprintln!("Warning: Unknown flag: {}", flag);
                }

                continue;
            } else if arg.starts_with("-") && !arg.contains(' ') {
                let flag = arg.replace("-", "");
                // each character is a flag to set
                for (x, c) in flag.chars().enumerate() {
                    let mut ffound = false;
                    if let Some(a) = get_action_from_flag(&c.to_string()) {
                        action = Some(a);
                        ffound = true;
                    }

                    if let Some((flag_item, takes_arg)) = get_flag_from_flag(&c.to_string()) {
                        // only the last arg can have input
                        if takes_arg && x + 1 == flag.chars().count() {
                            if i < args.len() {
                                let next_arg = &args[i + 1];
                                if !next_arg.starts_with("-") {
                                    flag_args.insert(flag_item.to_string(), next_arg.to_string());
                                    skip_next = true;
                                } else {
                                    eprintln!("Warning: Flag {} requires an argument", flag);
                                }
                            }
                        } else if takes_arg {
                            eprintln!("Warning: Flag {} requires an argument, so must be the last argument in the list {} {}", flag, x, flag.chars().count());
                        } else {
                            flags.insert(flag_item.to_string(), true);
                        }
                        ffound = true;
                    }

                    if !ffound {
                        eprintln!("Warning: Unknown flag: {}", c);
                    }
                }

                continue;
            }

            if action.is_none() {
                // this checks for a globalaction without the flag prefix
                if GLOBAL_ACTIONS.contains(&arg.as_str()) {
                    action = match get_action_from_flag(arg) {
                        Some(a) => Some(a),
                        None => Some(BAction::Add),
                    };
                    continue; // dont add to the input
                } else {
                    action = Some(BAction::Add)
                };
            }

            // we have user input, append each as input
            if input.is_none() {
                input = Some(arg.clone());
            } else {
                let mut new_input = input.unwrap();
                new_input.push(' ');
                new_input.push_str(arg);
                input = Some(new_input);
            }
        }

        // default to add if we have input but no action
        let action_default = match input.clone() {
            Some(a) if a.is_empty() => BAction::Add,
            Some(_) => BAction::ListDefault,
            None => BAction::ListDefault,
        };

        BArgs {
            args,
            action: action.unwrap_or(action_default),
            flags,
            flag_args,
            input,
        }
    }

    fn has_flag(&self, flag: &str) -> bool {
        self.flags.contains_key(flag)
    }

    fn flag_arg(&self, flag: &str) -> Option<String> {
        self.flag_args.get(flag).cloned()
    }
}

pub fn parse_args() -> BArgs {
    let args_input: Vec<String> = std::env::args().collect();

    let mut input_buffer = String::new();

    if !std::io::stdin().is_terminal() {
        let stdin = io::stdin();
        for line in stdin.lines() {
            let line = line.unwrap();
            input_buffer.push_str(&line);
            input_buffer.push('\n');
        }
    }

    if !input_buffer.trim().is_empty() {
        let args = BArgs::parse(args_input, Some(input_buffer));
        return args;
    }

    BArgs::parse(args_input, None)
}

#[cfg(test)]
mod tests {
    use crate::db::BItem;

    use super::*;

    #[test]
    fn test_get_action_from_flag() {
        assert!(matches!(get_action_from_flag("a"), Some(BAction::Add)));
        assert!(matches!(get_action_from_flag("add"), Some(BAction::Add)));
        assert!(matches!(get_action_from_flag("l"), Some(BAction::List)));
        assert!(matches!(get_action_from_flag("list"), Some(BAction::List)));
        assert!(matches!(get_action_from_flag("r"), Some(BAction::Remove)));
        assert!(matches!(
            get_action_from_flag("remove"),
            Some(BAction::Remove)
        ));
        assert!(get_action_from_flag("invalid").is_none());
    }

    #[test]
    fn test_add_command_variants() {
        let args1 = BArgs::parse(
            vec![
                "bjourn".to_string(),
                "add".to_string(),
                "this".to_string(),
                "is".to_string(),
                "a".to_string(),
                "test".to_string(),
            ],
            None,
        );
        assert!(matches!(args1.action, BAction::Add));
        assert_eq!(args1.input.unwrap(), "this is a test");

        let args2 = BArgs::parse(
            vec![
                "bjourn".to_string(),
                "this".to_string(),
                "is".to_string(),
                "a".to_string(),
                "test".to_string(),
            ],
            None,
        );
        assert!(matches!(args2.action, BAction::Add));
        assert_eq!(args2.input.unwrap(), "this is a test");

        let args3 = BArgs::parse(
            vec![
                "bjourn".to_string(),
                "--add".to_string(),
                "this".to_string(),
                "is".to_string(),
                "a".to_string(),
                "test".to_string(),
            ],
            None,
        );
        assert!(matches!(args3.action, BAction::Add));
        assert_eq!(args3.input.unwrap(), "this is a test");

        let args4 = BArgs::parse(
            vec![
                "bjourn".to_string(),
                "-a".to_string(),
                "this".to_string(),
                "is".to_string(),
                "a".to_string(),
                "test".to_string(),
            ],
            None,
        );
        assert!(matches!(args4.action, BAction::Add));
        assert_eq!(args4.input.unwrap(), "this is a test");

        let args5 = BArgs::parse(
            vec!["bjourn".to_string(), "this is a test".to_string()],
            None,
        );
        assert!(matches!(args5.action, BAction::Add));
        assert_eq!(args5.input.unwrap(), "this is a test");

        let args6 = BArgs::parse(
            vec!["bjourn".to_string(), "this".to_string(), "is".to_string()],
            Some("This is stdin input".to_string()),
        );
        assert!(matches!(args6.action, BAction::Add));
        assert_eq!(args6.input.unwrap(), "This is stdin input this is"); // appends teh text
    }

    #[test]
    fn test_option_inputs() {
        let args1 = BArgs::parse(
            vec![
                "bjourn".to_string(),
                "--output".to_string(),
                "json".to_string(),
                "this".to_string(),
                "is".to_string(),
                "a".to_string(),
                "test".to_string(),
            ],
            None,
        );
        assert!(matches!(args1.action, BAction::Add));
        assert_eq!(args1.input.clone().unwrap(), "this is a test");
        assert_eq!(args1.flag_arg("output"), Some("json".to_string()));

        let args2 = BArgs::parse(
            vec!["bjourn".to_string(), "-o".to_string(), "json".to_string()],
            None,
        );
        assert!(matches!(args2.action, BAction::ListDefault));
        assert_eq!(args2.input, None);
        assert_eq!(args2.flag_arg("output"), Some("json".to_string()));
    }
}
