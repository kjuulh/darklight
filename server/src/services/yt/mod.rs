use std::fmt::{Display, Formatter};
use std::fs::{canonicalize, create_dir_all};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::process::{Output, Stdio};

use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

#[derive(Error, Debug)]
pub enum YoutubeDLError {
    #[error("failed to execute youtube-dl")]
    IOError(#[from] std::io::Error),
    #[error("failed to convert path")]
    UTF8Error(#[from] std::string::FromUtf8Error),
    #[error("youtube-dl exited with: {0}")]
    Failure(String),
}

type Result<T> = std::result::Result<T, YoutubeDLError>;

const YOUTUBE_DL_COMMAND: &str = "yt-dlp";

#[derive(Clone, Debug)]
pub struct Arg {
    arg: String,
    input: Option<String>,
}

impl Arg {
    pub fn new(argument: &str) -> Self {
        Self {
            arg: argument.to_string(),
            input: None,
        }
    }

    pub fn new_with_args(argument: &str, input: &str) -> Self {
        Self {
            arg: argument.to_string(),
            input: Option::from(input.to_string()),
        }
    }
}

impl Display for Arg {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.input {
            Some(input) => write!(fmt, "{} {}", self.arg, input),
            None => write!(fmt, "{}", self.arg)
        }
    }
}

#[derive(Clone, Debug)]
pub struct YoutubeDL {
    path: PathBuf,
    links: Vec<String>,
    args: Vec<Arg>,
}

#[derive(Clone, Debug)]
pub struct YoutubeDLResult {
    path: PathBuf,
    output: String,
}

impl YoutubeDLResult {
    fn new(path: &PathBuf) -> Self {
        Self {
            path: path.clone(),
            output: String::new(),
        }
    }

    pub fn output_dir(&self) -> &PathBuf {
        &self.path
    }
}

impl YoutubeDL {
    pub fn new_multiple_links(
        dl_path: &PathBuf,
        args: Vec<Arg>,
        links: Vec<String>,
    ) -> Result<YoutubeDL> {
        let path = Path::new(dl_path);

        if !path.exists() {
            create_dir_all(&path)?;
        }

        if !path.is_dir() {
            return Err(YoutubeDLError::IOError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "path is not a directory",
            )));
        }

        let path = canonicalize(dl_path)?;
        Ok(YoutubeDL { path, links, args })
    }

    pub fn new(dl_path: &PathBuf, args: Vec<Arg>, link: &str) -> Result<YoutubeDL> {
        YoutubeDL::new_multiple_links(dl_path, args, vec![link.to_string()])
    }

    pub async fn download(&self) -> Result<YoutubeDLResult> {
        let output = self.spawn_youtube_dl().await?;
        let mut result = YoutubeDLResult::new(&self.path);

        if !output.status.success() {
            return Err(YoutubeDLError::Failure(String::from_utf8(output.stderr)?));
        }
        result.output = String::from_utf8(output.stdout)?;

        Ok(result)
    }

    async fn spawn_youtube_dl(&self) -> Result<Output> {
        let mut cmd = Command::new(YOUTUBE_DL_COMMAND);
        cmd.current_dir(&self.path)
            .env("LC_ALL", "en_US.UTF-8")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        for arg in self.args.iter() {
            match &arg.input {
                Some(input) => cmd.arg(&arg.arg).arg(input),
                None => cmd.arg(&arg.arg),
            };
        }

        for link in self.links.iter() {
            cmd.arg(&link);
        }

        let mut pr = cmd.spawn()?;

        {
            let stdout = pr.stdout.as_mut().unwrap();
            let stdout_reader = BufReader::new(stdout);
            let mut stdout_lines = stdout_reader.lines();

            while let Ok(Some(line)) = stdout_lines.next_line().await {
                if let Some(Ok(percentage)) = parse_line(line) {
                    println!("percentage: {}", percentage)
                }
            }
        }

        Ok(pr.wait_with_output().await?)
    }
}

fn parse_line(line: String) -> Option<core::result::Result<u32, ParseIntError>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[download\]\s+(\d+)").unwrap();
    }

    let capture: regex::Captures = RE.captures(line.as_str())?;
    if capture.len() != 2 {
        return None;
    }
    let str = &capture[1];
    Some(str.to_string().parse::<u32>())
}

#[cfg(test)]
mod tests {
    use crate::services::yt::parse_line;

    #[test]
    fn parse() {
        let percentage = parse_line("[download]  95.4% of ~215.85MiB at  9.61MiB/s ETA 00:01 (frag 144/151)".into());

        assert_eq!(percentage, Some(Ok(95)))
    }

    #[test]
    fn parse_get_nothing() {
        let nothing = parse_line("[download] Got server HTTP error: The read operation timed out. Retrying (attempt 1 of 10) ...".into());

        assert_eq!(nothing, None)
    }
}