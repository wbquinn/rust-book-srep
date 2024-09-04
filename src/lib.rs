use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = match config.ignore_case {
        true => search_case_insensitive(&config.query, &contents),
        false => search(&config.query, &contents),
    };

    for result in results {
        println!("{result}");
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lower_query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&lower_query))
        .collect()
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip the commandline

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Need to provide both query and file_path"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Need to provide file_path - query provided"),
        };
        let ignore_case = env::var("SREP_IGNORE_CASE").is_ok_and(|var| var.starts_with("t"));

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod cfg_tests {
    use super::*;

    #[test]
    fn build_config_with_no_params() {
        let command_line = "program-name-no-args";
        let args = command_line.split_whitespace().map(|arg| String::from(arg));
        let result = Config::build(args);

        assert!(!result.is_ok());
        assert!(result.is_err());
        let expected_err_message = "query and file_path";
        let error_message = result.err().unwrap();
        assert!(
            error_message.contains(expected_err_message),
            "expected err to contain '{expected_err_message}' but was '{error_message}'",
        );
    }
    #[test]
    fn build_config_with_one_params() {
        let command_line = "program-name-no-args foo";
        let args = command_line.split_whitespace().map(|arg| String::from(arg));
        let result = Config::build(args);

        assert!(!result.is_ok());
        assert!(result.is_err());
        let expected_err_message = "provide file_path";
        let error_message = result.err().unwrap();
        assert!(
            error_message.contains(expected_err_message),
            "expected err to contain '{expected_err_message}' but was '{error_message}'",
        );
    }
    #[test]
    fn build_config_with_two_params() {
        let command_line = "program-name-no-args foo bar";
        let args = command_line.split_whitespace().map(|arg| String::from(arg));
        let result = Config::build(args);

        match result {
            Ok(config) => {
                assert_eq!("bar", config.file_path);
                assert_eq!("foo", config.query);
            }
            Err(err) => panic!("Should not get an error with 2 params: {}", err),
        }
    }
}

#[cfg(test)]
mod search_tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn two_results() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
safe, fast, productive.
Pick three.";

        assert_eq!(
            search(query, contents).len(),
            2,
            "should be 2 results but were {}",
            search(query, contents).len()
        );
    }
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rusT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
