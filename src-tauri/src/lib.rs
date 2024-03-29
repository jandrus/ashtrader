use anyhow::{anyhow, Result};
use chrono::Utc;
use directories::ProjectDirs;
use phf::phf_map;
use std::fs;
use std::fs::create_dir_all;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;

struct Project<T: AsRef<str>> {
    qualifier: T,
    org: T,
    app: T,
}

pub enum ProjFiles {
    Login,
    Users,
    Log,
}

pub enum LogLevel {
    Debug,
    Error,
    Info,
    Warn,
}

const USER_FILE: &'static str = "users.json";

const LOGIN_FILE: &'static str = "login.json";

const LOG_FILE: &'static str = "ashtrader.log";

const PROJECT: Project<&'static str> = Project {
    qualifier: "io",
    org: "ashtech",
    app: "Ash Trader",
};

pub const ILLEGAL_ASSETS: [&'static str; 18] = [
    "FLOW2", "ETH2", "LUNA2", "USD", "CAD", "AUD", "EUR", "GBP", "CHF", "AED", "JPY", "EURT",
    "FLOWH", "ZEUR", "ZAUD", "ZGBP", "ZCAD", "ZJPY",
];

pub static TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "XBT" => "BTC",
    "XDG" => "DOGE",
    "XXBT" => "BTC",
    "ZUSD" => "USD",
    "XETH" => "ETH",
    "XZEC" => "ZEC",
    "XXLM" => "XLM",
    "XXDG" => "DOGE",
    "XREP" => "REP",
    "XXMR" => "XMR",
    "XMLN" => "MLN",
    "XETC" => "ETC",
    "XXRP" => "XRP",
    "XLTC" => "LTC",
};

pub fn log(level: LogLevel, location: &str, msg: &str) -> Result<()> {
    setup_file_struct()?;
    let now = format!("{:?}", Utc::now());
    let log_msg = match level {
        LogLevel::Debug => {
            format!("{} DEBUG [{}]: {}\n", now, location, msg)
        }
        LogLevel::Error => {
            format!("{} ERROR [{}]: {}\n", now, location, msg)
        }
        LogLevel::Info => {
            format!("{} INFO [{}]: {}\n", now, location, msg)
        }
        LogLevel::Warn => {
            format!("{} WARN [{}]: {}\n", now, location, msg)
        }
    };
    print!("{}", log_msg);
    let log_file = get_file(ProjFiles::Log)?;
    let mut f;
    if Path::new(&log_file).exists() {
        f = OpenOptions::new().append(true).open(log_file)?;
    } else {
        f = File::create(log_file)?;
    }
    f.write_all(log_msg.as_bytes())?;
    Ok(())
}

pub fn setup_file_struct() -> Result<()> {
    if let Some(proj) = ProjectDirs::from(PROJECT.qualifier, PROJECT.org, PROJECT.app) {
        let cache_dir = proj.cache_dir();
        if !cache_dir.exists() {
            create_dir_all(cache_dir)?;
        }
        let data_dir = proj.data_dir();
        if !data_dir.exists() {
            create_dir_all(data_dir)?;
        }
        return Ok(());
    }
    Err(anyhow!("Could not create project directory"))
}

pub fn read_file(path: &str) -> Result<String> {
    let mut s = String::new();
    let mut f = File::open(path)?;
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(anyhow!(e.to_string())),
    }
}

pub fn logout() -> Result<()> {
    let login_file = get_file(ProjFiles::Login)?;
    if Path::new(&login_file).exists() {
        fs::remove_file(login_file)?;
    }
    Ok(())
}

pub fn get_file(file: ProjFiles) -> Result<String> {
    if let Some(proj) = ProjectDirs::from(PROJECT.qualifier, PROJECT.org, PROJECT.app) {
        match file {
            ProjFiles::Login => {
                return Ok(format!(
                    "{}/{}",
                    proj.cache_dir().to_str().unwrap(),
                    LOGIN_FILE
                ));
            }
            ProjFiles::Log => {
                return Ok(format!(
                    "{}/{}",
                    proj.data_dir().to_str().unwrap(),
                    LOG_FILE
                ));
            }
            ProjFiles::Users => {
                return Ok(format!(
                    "{}/{}",
                    proj.data_dir().to_str().unwrap(),
                    USER_FILE
                ));
            }
        }
    }
    Err(anyhow!("Could not get project directory"))
}
