// Arguments parser functionallity
use phf::phf_map;
use std::collections::HashMap;
use std::io;
use std::matches;

// Actions
#[derive(Debug, Clone, PartialEq)]
pub enum BAction {
    Add,
    List,
    Remove,
}
// a list of first arg options enum
static GLOBAL_ACTIONS: [&str; 3] = ["add", "list", "remove"];

// flag map to actions
static GLOBAL_ACTION_ARGS_MAP: phf::Map<&'static str, BAction> = phf_map! {
    "a" => BAction::Add,
    "add" => BAction::Add,
    "l" => BAction::List,
    "list" => BAction::List,
    "r" => BAction::Remove,
    "remove" => BAction::Remove,
};

fn get_action_from_flag(flag: &str) -> Option<BAction> {
    if !GLOBAL_ACTION_ARGS_MAP.contains_key(flag) {
        return None;
    }
    GLOBAL_ACTION_ARGS_MAP.get(flag).cloned()
}

pub trait BJournRunner {
    fn parse(args: Vec<String>) -> Self;
    fn has_flag(&self, flag: &str) -> bool;
    fn flag_arg(&self, flag: &str) -> Option<String>;
}

#[derive(Debug)]
pub struct BArgs {
    args: Vec<String>,
    action: BAction,
    flags: HashMap<String, String>,
    flag_args: HashMap<String, String>,
    input: Option<String>,
}

impl BJournRunner for BArgs {
    fn parse(args: Vec<String>) -> Self {
        let flags = HashMap::new();
        let flag_args = HashMap::new();
        let mut action: Option<BAction> = None;
        let mut input = None;

        // take standrad in for piped add
        let mut input_buffer = String::new();
        for line in io::stdin().lines() {
            input_buffer.push_str(&line.unwrap());
            input_buffer.push_str("\n");
        }

        // quick short if nothing supplied
        if args.len() == 1 && input.is_none() {
            return BArgs {
                args,
                action: BAction::List,
                flags,
                flag_args,
                input,
            };
        }

        for (i, arg) in args.iter().enumerate() {
            // skip the first arg as it is the program name
            if i == 0 {
                continue;
            }

            // we allow args before or after the action flag, or anywhere in the command really
            if arg.starts_with("--") && !arg.contains(' ') {
                // check the flag is valid, otherwise print a warning
                // we check for the space in case someone has added dashes in their note at the beginning
                let flag = arg.replace("--", "");
                if let Some(a) = get_action_from_flag(&flag) {
                    action = Some(a);
                } else {
                    eprintln!("Warning: Unknown flag: {}", flag);
                }

                continue;
            } else if arg.starts_with("-") && !arg.contains(' ') {
                let flag = arg.replace("-", "");
                // each character is a flag to set
                for c in flag.chars() {
                    if let Some(a) = get_action_from_flag(&c.to_string()) {
                        action = Some(a);
                    } else {
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
                new_input.push_str(" ");
                new_input.push_str(arg);
                input = Some(new_input);
            }
        }

        BArgs {
            args,
            action: action.unwrap_or(BAction::Add), // default to add because we have some input
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
    let args = BArgs::parse(args_input);
    return args;
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
        assert!(matches!(get_action_from_flag("invalid"), None));
    }

    #[test]
    fn test_add_command_variants() {
        let args1 = BArgs::parse(vec![
            "bjourn".to_string(),
            "add".to_string(),
            "this".to_string(),
            "is".to_string(),
            "a".to_string(),
            "test".to_string(),
        ]);
        assert!(matches!(args1.action, BAction::Add));
        assert_eq!(args1.input.unwrap(), "this is a test");

        let args2 = BArgs::parse(vec![
            "bjourn".to_string(),
            "this".to_string(),
            "is".to_string(),
            "a".to_string(),
            "test".to_string(),
        ]);
        assert!(matches!(args2.action, BAction::Add));
        assert_eq!(args2.input.unwrap(), "this is a test");

        let args3 = BArgs::parse(vec![
            "bjourn".to_string(),
            "--add".to_string(),
            "this".to_string(),
            "is".to_string(),
            "a".to_string(),
            "test".to_string(),
        ]);
        assert!(matches!(args3.action, BAction::Add));
        assert_eq!(args3.input.unwrap(), "this is a test");

        let args4 = BArgs::parse(vec![
            "bjourn".to_string(),
            "-a".to_string(),
            "this".to_string(),
            "is".to_string(),
            "a".to_string(),
            "test".to_string(),
        ]);
        assert!(matches!(args4.action, BAction::Add));
        assert_eq!(args4.input.unwrap(), "this is a test");

        let args5 = BArgs::parse(vec!["bjourn".to_string(), "this is a test".to_string()]);
        assert!(matches!(args5.action, BAction::Add));
        assert_eq!(args5.input.unwrap(), "this is a test");
    }
}
