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
    let mut results: Vec<_> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<_> = Vec::new();

    for line in contents.lines() {
        //force both to lowercase
        let line_lower = line.to_lowercase();
        let query_lower = query.to_lowercase();
        if line_lower.contains(&query_lower) {
            results.push(line);
        }
    }
    results
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
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
        let result = Config::build(&["program-name-no-args".to_string()]);

        assert!(!result.is_ok());
        assert!(result.is_err());
        let expected_err_message = "arguments";
        let error_message = result.err().unwrap();
        assert!(
            error_message.contains(expected_err_message),
            "expected err to contain '{expected_err_message}' but was '{error_message}'",
        );
    }
    #[test]
    fn build_config_with_one_params() {
        let result = Config::build(&["program-name-no-args".to_string(), "foo".to_string()]);

        assert!(!result.is_ok());
        assert!(result.is_err());
        let expected_err_message = "arguments";
        let error_message = result.err().unwrap();
        assert!(
            error_message.contains(expected_err_message),
            "expected err to contain '{expected_err_message}' but was '{error_message}'",
        );
    }
    #[test]
    fn build_config_with_two_params() {
        let result = Config::build(&[
            "program-name-no-args".to_string(),
            "foo".to_string(),
            "bar".to_string(),
        ]);

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
