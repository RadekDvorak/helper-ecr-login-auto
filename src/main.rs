#![feature(exit_status_error)]

use std::env::args;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

use regex::Regex;
use tini::Ini;

const ENV_NAME: &str = "AWS_PROFILE";
const ECR_LOGIN_APP: &str = "docker-credential-ecr-login";

fn main() -> anyhow::Result<()> {
    let mut stdin_buffer = String::new();
    let arguments: Vec<String> = args().skip(1).collect();
    let mut command = Command::new(ECR_LOGIN_APP);

    if matches!(arguments.first().map(String::as_str), Some("get")) {
        io::stdin().read_to_string(&mut stdin_buffer)?;

        if let Some(account_id) = find_expected_account_id(&stdin_buffer) {
            if let Some(mut path) = home::home_dir() {
                path.push(".aws");
                path.push("config");
                eprintln!("Looking for account_id {:?}", account_id);

                let conf = tini::Ini::from_file(&path)?;
                let resolved_profile = std::env::var(ENV_NAME)
                    .ok()
                    .or_else(|| find_profile(account_id, &conf).map(str::to_owned));

                if let Some(profile) = resolved_profile {
                    eprintln!("Using profile {:?}", &profile);
                    command.env(ENV_NAME, &profile);
                };
            }
        }
    }

    let _ = &command.stdin(Stdio::piped()).args(&arguments);
    let child = command.spawn()?;
    child
        .stdin
        .as_ref()
        .unwrap() // stdin is set above, it is always there
        .write_all(stdin_buffer.as_bytes())?;

    let status = &child.wait_with_output()?;

    io::stdout().lock().write_all(&status.stdout)?;

    Ok(status.status.exit_ok()?)
}

fn find_profile(account_id: u64, conf: &Ini) -> Option<&str> {
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
