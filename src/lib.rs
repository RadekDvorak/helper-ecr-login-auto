use crate::cli::Configuration;
use regex::Regex;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};
use tini::Ini;

pub mod cli;
pub struct AccountID(String);

pub enum AccountIdError {
    WrongLength,
    NotOnlyDigits,
}

impl AccountID {
    pub fn validate(value: &str) -> Result<(), AccountIdError> {
        if value.len() != 12 {
            return Err(AccountIdError::WrongLength);
        }

        if !value.chars().all(|c| c.is_ascii_digit()) {
            return Err(AccountIdError::NotOnlyDigits);
        }

        Ok(())
    }
}

impl TryFrom<String> for AccountID {
    type Error = AccountIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        AccountID::validate(&value)?;

        Ok(AccountID(value))
    }
}

impl Display for AccountID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub enum ProfileMatchingError {
    ConfigurationError(String),
    IniError(tini::Error),
}

impl Display for ProfileMatchingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfileMatchingError::ConfigurationError(msg) => {
                write!(f, "Configuration error: {}", msg)
            }
            ProfileMatchingError::IniError(err) => {
                write!(f, "INI parsing error: {:?}", err)
            }
        }
    }
}

impl Error for ProfileMatchingError {}

pub fn find_aws_profile(
    stdin_buffer: &str,
    home_dir: Option<PathBuf>,
    config: &Configuration,
) -> Result<Option<String>, ProfileMatchingError> {
    if let Some(account_id) = find_expected_account_id(stdin_buffer) {
        if let Some(profile) = &config.forced_profile {
            log::info!("Using forced profile {}", &profile);
            return Ok(Some(profile.to_owned()));
        }

        if let Some(mut path) = home_dir {
            path.push(".aws");
            path.push("config");
            log::info!("Looking for account_id {}", account_id);

            let conf = Ini::from_file(&path).map_err(ProfileMatchingError::IniError)?;
            let possible_key = Option::as_ref(&config.arn_config_key).map(|s| s.as_str());
            let resolved_profile = match_profile(account_id, &conf, possible_key)?;

            if let Some(profile) = resolved_profile {
                log::info!("Found profile {:?}", &profile);

                return Ok(Some(profile));
            };
        }
    }

    Ok(None)
}

#[derive(Debug)]
pub enum DelegationError {
    IoError(std::io::Error),
    StdInWriteError(std::io::Error),
}

impl Display for DelegationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DelegationError::IoError(err) => write!(f, "I/O error: {}", err),
            DelegationError::StdInWriteError(err) => write!(f, "Stdin write error: {}", err),
        }
    }
}

pub fn delegate<T: Into<Stdio>, U: Into<Stdio>>(
    arguments: &[String],
    stdin_buffer: &str,
    stdout: T,
    stderr: U,
    aws_profile: Option<String>,
    config: &Configuration,
) -> Result<ExitStatus, DelegationError> {
    let mut command = Command::new(&config.upstream_auth_app);

    if let Some(profile) = aws_profile {
        command.env("AWS_PROFILE", profile); // AWS_PROFILE should override AWS_DEFAULT_PROFILE
    }

    command
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(stdout)
        .stderr(stderr);
    let mut child = command.spawn().map_err(DelegationError::IoError)?;
    child
        .stdin
        .as_ref()
        .unwrap() // stdin is set above, it is always there
        .write_all(stdin_buffer.as_bytes())
        .map_err(DelegationError::StdInWriteError)?;

    let status = child.wait().map_err(DelegationError::IoError)?;

    Ok(status)
}

fn match_profile(
    account_id: AccountID,
    conf: &Ini,
    possible_custom_key: Option<&str>,
) -> Result<Option<String>, ProfileMatchingError> {
    let cred_proc = Regex::new(&format!(r"arn:aws:iam::{}:role/", account_id)).unwrap();

    for (section_name, section) in conf.iter() {
        for (ini_key, value) in section.iter() {
            let is_key_found = if let Some(custom_key) = &possible_custom_key {
                // compare with the user-selected key
                custom_key == ini_key
            } else {
                // compare with default keys
                matches!(
                    ini_key.as_str(),
                    "credential_process" | "role_arn" | "vegas_role_arn"
                )
            };

            if is_key_found && cred_proc.is_match(value) {
                let trimmed = section_name
                    .strip_prefix("profile ")
                    .unwrap_or(section_name);

                return Ok(Some(trimmed.to_owned()));
            }
        }
    }

    Ok(None)
}

fn find_expected_account_id(stdin: &str) -> Option<AccountID> {
    let ecr = Regex::new(r"^([0-9]+)\.dkr\.ecr\.[^.]+\.amazonaws\.com").unwrap();

    let first_match = ecr.captures(stdin)?.get(1)?;

    AccountID::try_from(first_match.as_str().to_owned()).ok()
}
