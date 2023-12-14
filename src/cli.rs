use std::path::PathBuf;

use lexopt::{Arg, Parser};

#[derive(Debug)]
pub struct Options {
    pub recursive: bool,
    pub paths: Vec<PathBuf>,
    pub print_last_log: bool,
}

pub fn options() -> anyhow::Result<Options> {
    let mut parser = Parser::from_env();
    let mut paths = vec![];
    let mut recursive = false;
    let mut print_last_log = false;

    while let Some(arg) = parser.next()? {
        match arg {
            Arg::Short('r') | Arg::Short('R') | Arg::Long("recursive") => {
                recursive = true;
            }
            Arg::Long("get") => {
                print_last_log = true;
            }
            Arg::Value(path) => paths.push(PathBuf::from(path)),
            _ => (),
        }
    }

    if print_last_log && recursive {
        anyhow::bail!("`--get` is mutually exclusive with all other options");
    }

    Ok(Options {
        recursive,
        paths,
        print_last_log,
    })
}
