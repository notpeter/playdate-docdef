use clap::{Parser, ValueEnum};
use std::{env, fs::File, io::Read, path::PathBuf};

const SCOREBOARD_API_URL: &str = "https://help.play.date/catalog-developer/scoreboard-api/";

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

fn fetch_url(url: &str) -> String {
    eprintln!("Fetching from {}", url);
    let resp = reqwest::blocking::Client::new().get(url).send();
    match resp {
        Ok(r) if r.status().is_success() => r.text().unwrap(),
        _ => panic!("Error fetching from {}", url),
    }
}

fn is_scoreboard_api_url(url: &str) -> bool {
    url.trim_end_matches('/') == SCOREBOARD_API_URL.trim_end_matches('/')
}

/// Retrieves the primary SDK docs and the Catalog Scoreboard API docs.
pub fn fetch_docs(args: &Args) -> Vec<String> {
    let primary = match &args.url {
        Some(url) => fetch_url(url),
        None => fetch_file(args.path.as_ref().unwrap()),
    };

    let mut docs = vec![primary];
    if !args.url.as_deref().is_some_and(is_scoreboard_api_url) {
        docs.push(fetch_url(SCOREBOARD_API_URL));
    }
    docs
}

/// Parse command line arguments
pub fn parse() -> Args {
    Args::parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scoreboard_url_is_recognized_with_or_without_trailing_slash() {
        assert!(is_scoreboard_api_url(SCOREBOARD_API_URL));
        assert!(is_scoreboard_api_url(
            "https://help.play.date/catalog-developer/scoreboard-api"
        ));
        assert!(!is_scoreboard_api_url(
            "https://sdk.play.date/Inside%20Playdate.html"
        ));
    }
}
