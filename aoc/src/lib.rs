extern crate reqwest;
extern crate cachedir;
extern crate preferences;
extern crate clap;
extern crate select;
extern crate serde;
extern crate chrono;
extern crate chrono_tz;
#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;

use preferences::{AppInfo, Preferences};
use reqwest::header::Cookie;
use cachedir::{CacheDirConfig, CacheDir};
use failure::ResultExt;
use std::io::{Read, Write};
use std::fs::File;
use std::collections::HashMap;

pub use self::leaderboard::Leaderboard;

pub mod cli;
pub mod leaderboard;
pub mod config;

pub type Result<T> = ::std::result::Result<T, failure::Error>;

const APP_INFO: &AppInfo = &AppInfo {
    name: "advent_of_code",
    author: "panicbit"
};

pub struct Client {
    event: String,
    session_token: String,
    client: reqwest::Client,
    cache_dir: CacheDir,
}

impl Client {
    pub fn new<E, S>(event: E, session_token: S) -> Result<Self> where
        E: Into<String>,
        S: Into<String>,
    {
        let event = event.into();

        Ok(Self {
            cache_dir: CacheDirConfig::new(&format!("advent_of_code/{}", event)).get_cache_dir()?,
            event: event.into(),
            session_token: session_token.into(),
            client: reqwest::Client::new(),
        })
    }

    pub fn get_input(&self, day: u8) -> Result<String> {
        if let Ok(input) = self.get_cached_input(day) {
            return Ok(input);
        }

        let input = self.download_input(day)?;
        self.cache_input(day, &input)?;
        
        Ok(input)
    }

    fn get_cached_input(&self, day: u8) -> Result<String> {
        let path = self.cache_dir.join(format!("input_day_{}", day));
        let mut file = File::open(path)?;
        let mut input = String::new();
        file.read_to_string(&mut input)?;

        Ok(input)
    }

    fn cache_input(&self, day: u8, input: &str) -> Result<()> {
        let path = self.cache_dir.join(format!("input_day_{}", day));
        let mut file = File::create(path)?;

        file.write_all(input.as_bytes())?;

        Ok(())
    }

    fn download_input(&self, day: u8) -> Result<String> {
        let url = format!("https://adventofcode.com/{}/day/{}/input", self.event, day);
        let mut cookie = Cookie::new();
        cookie.set("session", self.session_token.clone());
        let input = self.client
            .get(&url)
            .header(cookie)
            .send()?
            .error_for_status()?
            .text()?;
        
        Ok(input)
    }

    pub fn submit_solution(&self, day: u8, level: u8, solution: &str) -> Result<String> {
        use select::document::Document;
        use select::predicate::Name;

        let url = format!("https://adventofcode.com/{}/day/{}/answer", self.event, day);
        let mut cookie = Cookie::new();
        cookie.set("session", self.session_token.clone());

        let mut params = HashMap::new();
        params.insert("level", level.to_string());
        params.insert("answer", solution.into());

        let response = self.client
            .post(&url)
            .header(cookie)
            .form(&params)
            .send()?
            .error_for_status()?
            .text()?;

        let doc = Document::from(response.as_str());
        let node = doc.find(Name("main")).next().ok_or_else(|| format_err!("Response element not found"))?;
        let text = node.text();
        // let text = text.trim().split(".  ").next().unwrap_or("");
        let text = format!("{}.", text.trim());

        Ok(text)
    }
}

pub fn get_session_token() -> Result<String> {
    let token = String::load(APP_INFO, "session_token")
        .context("Failed to load session token")?;
    Ok(token)
}

pub fn set_session_token<S: Into<String>>(token: S) -> Result<()> {
    token.into().save(APP_INFO, "session_token")
        .context("Failed to save session token")?;
    Ok(())
}
