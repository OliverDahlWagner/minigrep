use std::{ env, error::Error, fs }; // fs is filesystem

// Box<dyn Error> a way to say any error, the ? says that it should make a value, if it aint ok its an error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = search_case_insensitive(&config.query, &contents);

    println!("---RESULT---");
    for line in results {
        println!("{}", line);
    }
    println!("");

    return Ok(());
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // constructer (new)  ---  return Result is ok or error return conditions
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // getting rid of the first element, the path to the program

        // arguments
        let mut query = None;
        let mut filename = None;

        // return result here so error happens here, if just option error should happen everywhere this is used
        fn parse_argument_value(
            arg: &str,
            args: &mut env::Args
        ) -> Result<Option<String>, &'static str> {
            match args.next() {
                Some(val) if !val.is_empty() && !val.starts_with("--") => Ok(Some(val)),
                Some(_) => {
                    let err_msg = format!("Invalid target for {}", arg);
                    Err(Box::leak(err_msg.into_boxed_str()))
                }
                None => {
                    let err_msg = format!("Missing target for {}", arg);
                    Err(Box::leak(err_msg.into_boxed_str())) // leaks to heap from stack, and get 'static
                }
            }
        }

        //
        // Come back and make better later
        //
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--query" => {
                    query = parse_argument_value("--query", &mut args)?;
                }
                "--file" => {
                    filename = parse_argument_value("--file", &mut args)?;
                }
                _ => {
                    return Err("Unknown Argument Given: try --help");
                }
            }
        }

        // while let Some(arg) = args.next() {
        //     match arg.as_str() {
        //         "--query" => {
        //             query = args.next();
        //         }
        //         "--file" => {
        //             filename = args.next();
        //         }
        //         _ => {
        //             return Err("Unknown Argument Given: try --help");
        //         }
        //     }
        // }

        if query.is_none() || filename.is_none() {
            return Err("Missing required arguments");
        }

        return Ok(Config { query: query.unwrap(), filename: filename.unwrap() });
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    return results;
}

#[cfg(test)] // <-- configuration test
mod tests {
    use super::*; // import everything from the parent module

    #[test] // <-- test attribute, making it a test function
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents)); // search() returns the line with duct
    }

    #[test] // <-- test attribute, making it a test function
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me";

        assert_eq!(vec!["Rust:", "Trust me"], search_case_insensitive(query, contents)); // search_ca..() return both lines and dont care about case
    }
}
