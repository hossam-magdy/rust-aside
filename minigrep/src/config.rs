use std::{cmp, env};

#[derive(Debug, PartialEq)]
pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
    pub case_insensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let query = args
            .get(1)
            .ok_or("Could not extract Search query from the first argument")?;

        let filename = args
            .get(2)
            .ok_or("Could not extract Filename from the second argument")?;

        let env_string = env::var("CASE_INSENSITIVE")
            .unwrap_or("0".to_string())
            .to_lowercase();

        let case_insensitive = match &env_string[..(cmp::min(4, env_string.len()))] {
            "1" => true,
            "yes" => true,
            "true" => true,
            _ => false,
        };

        Ok(Config {
            query,
            filename,
            case_insensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_handles_correct_args() {
        let args = [
            "bin".to_string(),
            "queryStr".to_string(),
            "fileNameStr".to_string(),
        ];

        assert_eq!(
            Config::new(&args).unwrap(),
            Config {
                query: &"queryStr".to_string(),
                filename: &"fileNameStr".to_string()
            }
        );
    }

    #[test]
    fn config_new_handles_1_arg_as_error() {
        assert_eq!(
            Config::new(&["bin".to_string()]),
            Err("Could not extract Search query from the first argument")
        );
    }

    #[test]
    fn config_new_handles_2_args_as_error() {
        assert_eq!(
            Config::new(&["bin".to_string(), "queryStr".to_string()]),
            Err("Could not extract Filename from the second argument")
        );
    }
}
