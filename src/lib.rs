use regex::Regex;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::from_utf8;
use tini::Ini;

use myenv::EnvLike;

pub mod myenv;

const ECR_LOGIN_APP: &str = "docker-credential-ecr-login";
const ENV_NAME: &str = "AWS_PROFILE";
const ARN_CONFIG_KEY: &str = "ARN_CONFIG_KEY";

pub struct AccountID(String);

pub enum AccountIdError {
    WrongLength,
    NotOnlyDigits,
}
impl AccountID {
    pub fn try_new(id: impl Into<String> + Display) -> Result<AccountID, AccountIdError> {
        let typed_id = id.into();
        if typed_id.len() != 12 {
            return Err(AccountIdError::WrongLength);
        }

        if !typed_id.chars().all(|c| c.is_digit(10)) {
            return Err(AccountIdError::NotOnlyDigits);
        }

        Ok(AccountID(typed_id))
    }
}

impl Display for AccountID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn find_aws_profile<V: Write, E: EnvLike>(
    stdin_buffer: &str,
    mut err: V,
    home_dir: Option<PathBuf>,
    environment: E,
) -> anyhow::Result<Option<String>> {
    if let Some(account_id) = find_expected_account_id(stdin_buffer) {
        if let Ok(profile) = environment.var(ENV_NAME) {
            writeln!(err, "Using the predefined {}={}", ENV_NAME, &profile)?;
            return Ok(Some(profile));
        }

        if let Some(mut path) = home_dir {
            path.push(".aws");
            path.push("config");
            writeln!(err, "Looking for account_id {}", account_id)?;

            let conf = Ini::from_file(&path)?;
            let resolved_profile = match_profile(account_id, &conf, environment).map(str::to_owned);

            if let Some(profile) = resolved_profile {
                writeln!(err, "Found profile {:?}", &profile)?;

                return Ok(Some(profile));
            };
        }
    }

    Ok(None)
}

pub fn delegate(
    arguments: &[String],
    stdin_buffer: &str,
    aws_profile: Option<String>,
) -> anyhow::Result<String> {
    let mut command = Command::new(ECR_LOGIN_APP);

    if let Some(profile) = aws_profile {
        command.env(ENV_NAME, profile);
    }

    command.stdin(Stdio::piped()).args(arguments);
    let child = command.spawn()?;
    child
        .stdin
        .as_ref()
        .unwrap() // stdin is set above, it is always there
        .write_all(stdin_buffer.as_bytes())?;

    let status = &child.wait_with_output()?;

    Ok(from_utf8(&status.stdout).map(str::to_owned)?)
}

fn match_profile<E: EnvLike>(account_id: AccountID, conf: &Ini, environment: E) -> Option<&str> {
    let cred_proc = Regex::new(&format!(r"arn:aws:iam::{}:role/", account_id)).unwrap();
    let possible_custom_key = &environment.var(ARN_CONFIG_KEY).ok();

    for (section_name, section) in conf.iter() {
        for (ini_key, value) in section.iter() {
            let is_key_found = if let Some(custom_key) = possible_custom_key {
                // compare with user selected key
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
                return Some(trimmed);
            }
        }
    }

    None
}

fn find_expected_account_id(stdin: &str) -> Option<AccountID> {
    let ecr = regex::Regex::new(r"^([0-9]+)\.dkr\.ecr\.[^.]+\.amazonaws\.com").unwrap();
    let first_match = ecr.captures(stdin)?.get(1)?;

    AccountID::try_new(first_match.as_str()).ok()
}
