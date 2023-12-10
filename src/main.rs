// use std::fmt;
use clap::Parser;
use std::path::PathBuf;
// use std::error::Error;

// #[derive(Clone, Debug)]
// pub enum ReadFailure {
//     MissingFile(String),
//     BadData(String),
// }

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// use cfgfile when printing git status.
    #[arg(long)]
    cfg: PathBuf,

    /// prints default configuration file.
    #[arg(long)]
    printcfg: bool,

    /// outputs Git status as JSON and print errors.
    #[arg(long)]
    dbg: bool,
    // exits if still running after given duration (ex: 2s, 500ms).
    // #[arg(long)]
    // timeout: u16,
}

#[derive(Debug)]
struct Config {
    branch: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            branch: String::from("⎇ "),
        }
    }
}

#[derive(Debug)]
struct Gitmux {
    raw_yaml: serde_yaml::Value,
    statusline: Option<String>,
    config: Option<Config>,
}

impl Default for Gitmux {
    fn default() -> Self {
        Self {
            raw_yaml: read_yaml_content(&PathBuf::from("default.yaml")).unwrap(),
            statusline: None,
            config: None
        }
    }
}

fn read_yaml_content(path_to_yaml_file: &PathBuf) -> Result<serde_yaml::Value, serde_yaml::Error> {
    // Path cannot be incorrect
    let f = std::fs::File::open(path_to_yaml_file).unwrap_or_else(|_| panic!("Filepath - {} does not exist or could not be found", path_to_yaml_file.display()));
    let data: serde_yaml::Value = serde_yaml::from_reader(f)?;
    Ok(data)
}

fn main() {
    let args = Args::parse();
    dbg!(&args);
    // let test = d["tmux"]["styles"]["state"].as_str().unwrap().to_owned();
    // println!("{}", test);

     
}
