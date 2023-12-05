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
struct Gitmux {
   config: serde_yaml::Value 
}


fn read_yaml_content(path_to_file: PathBuf) -> Result<serde_yaml::Value, serde_yaml::Error> {
    let f = std::fs::File::open(path_to_file).expect(format!("Filepath - {} does not exist or could not be found", path_to_file.display()).as_str());
    let data: serde_yaml::Value = serde_yaml::from_reader(f)?;
    dbg!("Read YAML string: {}", data);
    return Ok(data);
}

impl Default for Gitmux {
    fn default() -> Self {
        if let Some(data) = read_yaml_content(PathBuf::from("default.yaml")) {
            return Gitmux { config: data };
        }
        else {
            return Gitmux { config: read_yaml_content(PathBuf::from("default.yaml")).unwrap() };
        }
    }
}
fn main() {
    let args = Args::parse();
    dbg!(&args);
    // let test = d["tmux"]["styles"]["state"].as_str().unwrap().to_owned();
    // println!("{}", test);
}
