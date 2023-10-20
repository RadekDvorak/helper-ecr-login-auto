use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::from_utf8;

use regex::Regex;
use tini::Ini;

const ECR_LOGIN_APP: &str = "docker-credential-ecr-login";
const ENV_NAME: &str = "AWS_PROFILE";
const ARN_CONFIG_KEY: &str = "ARN_CONFIG_KEY";

pub fn find_aws_profile<V: Write>(
    stdin_buffer: &str,
    mut err: V,
    home_dir: Option<PathBuf>,
) -> anyhow::Result<Option<String>> {
    if let Some(account_id) = find_expected_account_id(stdin_buffer) {
        if let Some(mut path) = home_dir {
            path.push(".aws");
            path.push("config");
            writeln!(err, "Looking for account_id {:?}", account_id)?;

            let conf = tini::Ini::from_file(&path)?;
            let resolved_profile = std::env::var(ENV_NAME)
                .ok()
                .or_else(|| match_profile(account_id, &conf).map(str::to_owned));

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

fn match_profile(account_id: u64, conf: &Ini) -> Option<&str> {
    let cred_proc = Regex::new(&format!(r"arn:aws:iam::{:0>12}:role/", account_id)).unwrap();
    let possible_custom_key = &std::env::var(ARN_CONFIG_KEY).ok();

    for (section_name, section_iter) in conf.iter() {
        for (ini_key, value) in section_iter {
            let is_key_matched = if let Some(custom_key) = possible_custom_key {
                // compare with user selected key
                custom_key == ini_key
            } else {
                // compare with default keys
                matches!(ini_key.as_str(), "credential_process" | "role_arn" | "vegas_role_arn" )
            };

            if is_key_matched {
                if cred_proc.is_match(value) {
                    let trimmed = section_name
                        .strip_prefix("profile ")
                        .unwrap_or(section_name);
                    return Some(trimmed);
                }
            }
        }
    }

    None
}

fn find_expected_account_id(stdin: &str) -> Option<u64> {
    let ecr = regex::Regex::new(r"^([0-9]+)\.dkr\.ecr\.[^\.]+\.amazonaws\.com").unwrap();
    let ecr_captures = ecr.captures(stdin);

    ecr_captures
        .as_ref()
        .map(|c| c[1].to_owned())
        .and_then(|c| c.parse::<u64>().ok())
}
