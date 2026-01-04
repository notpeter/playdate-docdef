use clap::{Parser, ValueEnum};
use std::{env, fs::File, io::Read, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(value_enum, default_value = "stub")]
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
    let mut pb = PathBuf::new();
    match env::var_os("PLAYDATE_SDK_PATH") {
        Some(p) => pb.push(p),
        _ => match env::consts::FAMILY {
            "unix" if env::var_os("HOME").is_some() => {
                pb.push(env::var("HOME").unwrap());
                pb.push("Developer");
                pb.push("PlaydateSDK");
            }
            "windows" if env::var_os("USERPROFILE").is_some() => {
                pb.push(env::var("USERPROFILE").unwrap());
                pb.push("Documents");
                pb.push("PlaydateSDK");
            }
            _ => panic!(),
        },
    }
    pb
}

fn fetch_file(path: &PathBuf) -> String {
    let mut response = String::new();
    let filename = path.join("Inside Playdate.html");
    eprintln!("Reading from {}", filename.display());
    let mut file = match File::open(filename) {
        Err(why) => panic!("couldn't open file: {}", why),
        Ok(file) => file,
    };
    file.read_to_string(&mut response).unwrap();
    response
}

fn fetch_url(url: String) -> String {
    let resp = reqwest::blocking::Client::new().get(&url).send();
    match resp {
        Ok(r) if r.status().is_success() => r.text().unwrap(),
        _ => panic!("Error fetching from {}", url),
    }
}

/// Retrieves the contents of the docs (from file or url)
pub fn fetch_docs(args: &Args) -> String {
    match &args.url {
        Some(url) => fetch_url(url.clone()),
        None => fetch_file(args.path.as_ref().unwrap()),
    }
}

/// Parse command line arguments
pub fn parse() -> Args {
    Args::parse()
}
