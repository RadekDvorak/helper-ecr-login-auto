#![feature(exit_status_error)]

use std::env::args;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::str::from_utf8;

use regex::Regex;
use tini::Ini;

const ECR_LOGIN_APP: &str = "docker-credential-ecr-login";
const ENV_NAME: &str = "AWS_PROFILE";
const STDIN_READ_LIMIT: usize = 1 << 20; // 1MB

fn main() -> anyhow::Result<()> {
    let mut stdin_buffer = String::new();
    io::stdin()
        .take(STDIN_READ_LIMIT as u64 + 1)
        .read_to_string(&mut stdin_buffer)?;

    anyhow::ensure!(
        stdin_buffer.len() <= STDIN_READ_LIMIT,
        "standard input  up to {} bytes is allowed, got more",
        STDIN_READ_LIMIT
    );

    let arguments: Vec<String> = args().skip(1).collect();
    let aws_profile = match arguments.first() {
        Some(v) if v == "get" => find_aws_profile(&stdin_buffer, &mut io::stderr())?,
        _ => None,
    };

    let answer = delegate(&arguments, &stdin_buffer, aws_profile)?;
    io::stdout().lock().write_all(answer.as_bytes())?;

    Ok(())
}

fn find_aws_profile<V: Write>(stdin_buffer: &str, mut err: V) -> anyhow::Result<Option<String>> {
    if let Some(account_id) = find_expected_account_id(stdin_buffer) {
        if let Some(mut path) = home::home_dir() {
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

fn delegate(
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

    match status.status.exit_ok() {
        Ok(_) => Ok(from_utf8(&status.stdout).map(str::to_owned)?),
        Err(e) => Err(e.into()),
    }
}

fn match_profile(account_id: u64, conf: &Ini) -> Option<&str> {
    let cred_proc = Regex::new(r"arn:aws:iam::([0-9]+):role/").unwrap();

    for (section_name, section_iter) in conf.iter() {
        for (key, value) in section_iter {
            match key.as_str() {
                "credential_process" | "role_arn" => {
                    if let Some(c) = &cred_proc.captures(value) {
                        if c[1] == account_id.to_string() {
                            return section_name.strip_prefix("profile ").or(Some(section_name));
                        }
                    }
                }
                _ => {}
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
