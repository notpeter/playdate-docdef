use std::{fs::File, io::Read, path::PathBuf, env};
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(value_enum, default_value="stub")]
    pub action: Action,

    /// Filename to load from
    #[arg(short, long, value_hint = clap::ValueHint::DirPath, default_value = get_sdk_dir().into_os_string())]
    pub path: Option<std::path::PathBuf>,

    #[arg(short, long, value_hint = clap::ValueHint::Url, conflicts_with("path"))]
    pub url: Option<std::string::String>,

    /// Verbose logging (-v, -vv, -vvv, etc.)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

}

// CLI Action: Generate Function Stubs or full Lua with annotation comments
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Action {
    Stub,
    Annotate,
}

fn get_sdk_dir() -> PathBuf {
    if env::var_os("PLAYDATE_SDK_PATH") != None {
        return PathBuf::from(env::var_os("PLAYDATE_SDK_PATH").unwrap());
    }
    let mut path = PathBuf::new();
    path.push(home::home_dir().unwrap());
    path.push("Developer");
    path.push("PlaydateSDK");
    path
}

fn fetch_or_file(args: &Args) -> String {
    let mut response = String::new();
    if args.url.is_some() {
        let url = args.url.as_ref().unwrap();
        eprintln!("Fetching from {}", url);
        let resp = reqwest::blocking::Client::new().get(url).send();
        match resp {
            Ok(r) if r.status().is_success() => {
                response = r.text().unwrap();
            }
            _ => { panic!("Error fetching from {}", url); }
        }

    } else {
        let filename = args.path.as_ref().unwrap().join("Inside Playdate.html");
        eprintln!("Reading from {}", filename.display());
        let mut file = match File::open(filename) {
            Err(why) => panic!("couldn't open file: {}", why),
            Ok(file) => file,
        };
        file.read_to_string(&mut response).unwrap();
    }
    response
}

pub fn setup() -> (Args, String) {
    let args = Args::parse();
    let response = fetch_or_file(&args);
    (args, response)
}
