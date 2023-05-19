use std::path::PathBuf;

use anyhow::{anyhow, Result, Context};

use crate::opts::Opts;

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Opts> for Config {
    // Need to setup the anyhow 
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        // Converting it one thing at a time
        let operation = value.args.try_into()?;
        let config = get_config(value.config)?;
        let pwd = get_pwd(value.pwd)?;

        // could have inlined in the struct creation
        return Ok(Config {
            operation,
            pwd,
            config,
        });
    }
}


#[derive(Debug, PartialEq)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String)
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let mut value = value;

        let term = value.get(0).expect("expect to exist");
        if term == "add" {
            if value.len() != 3 {
                let err = anyhow!("add expects 2 arguments, got {}", value.len() - 1);
                return Err(err);
            }

            // drain instead of having to mess with reversing, etc. when popping
            // trying to get first two items out.
            let mut drain = value.drain(1..=2);
            return Ok(
                Operation::Add(
                    drain.next().expect("to exist"),
                    drain.next().expect("to exist")
                ));
        }

        if term == "rm" {
            if value.len() != 2 {
                let err = anyhow!("remove expects 1 argument, got {}", value.len() - 1);
                return Err(err);
            }

            let arg = value.pop().expect("to exist");
            return Ok(
                Operation::Remove(arg)
            );
        }

        if value.len() > 1 {
            let err = anyhow!("Print expected 0 or 1 arguments, got {}", value.len());
            return Err(err);
        }

        // Then otherwise it's just the args (and I believe it outputs the current configuration in that case)
        let arg = value.pop().expect("to exist");
        return Ok(Operation::Print(Some(arg)));
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }

    // Need the location that we are supposed to look in.
    let location = "HOME";
    let location = std::env::var(location).context("Unable to get the location of the config directory")?;
    
    let mut location = PathBuf::from(location);
    location.push(".config");
    // add projector's location and the config file name
    location.push("projector");
    location.push("projector.json");

    return Ok(location);
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }

    let pwd = std::env::current_dir().context("error getting the current_dir")?;
    return Ok(pwd);
}

#[cfg(test)]
mod test {
    use anyhow::Ok;
    use anyhow::Result;

    use crate::opts::Opts;
    use crate::config::Config;
    use crate::config::Operation;

    #[test]
    fn test_print_all() -> Result<()> {
        let opts: Config = Opts {
            args: vec![],
            config: None,
            pwd: None,
        }.try_into()?;

        assert_eq!(opts.operation, Operation::Print(None));
        return Ok(());
    }

    #[test]
    fn test_print_key() -> Result<()> {
        let opts: Config = Opts {
            args: vec!["foo".to_string()],
            config: None,
            pwd: None,
        }.try_into()?;

        assert_eq!(opts.operation, Operation::Print(Some("foo".to_string())));
        return Ok(());
    }

    #[test]
    fn test_add() -> Result<()> {
        let opts: Config = Opts {
            args: vec![String::from("add"), String::from("key"), String::from("value")],
            config: None,
            pwd: None,
        }.try_into()?;

        assert_eq!(opts.operation, Operation::Add(String::from("key"), String::from("value")));
        return Ok(());
    }

    #[test]
    fn test_remove() -> Result<()> {
        let opts: Config = Opts {
            args: vec![String::from("rm"), String::from("key")],
            config: None,
            pwd: None
        }.try_into()?;

        assert_eq!(opts.operation, Operation::Remove(String::from("key")));
        return Ok(());
    }
}