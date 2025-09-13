use clap::Parser;
use env_logger::Builder;
use helper_ecr_login_auto::cli::{Commands, Configuration};
use helper_ecr_login_auto::{delegate, find_aws_profile};
use home::home_dir;
use log::LevelFilter;
use std::io::{self, Read};
use std::process::{ExitCode, ExitStatus, Stdio};
use std::str::FromStr;

const STDIN_READ_LIMIT: usize = 1 << 20; // 1MB

fn main() -> ExitCode {
    let cli = Configuration::parse();

    Builder::new()
        .filter_level(LevelFilter::from_str(&cli.log_level).unwrap_or(LevelFilter::Info))
        .write_style((&cli.log_style).into())
        .init();

    match real_main(cli) {
        Ok(status) => match status.code() {
            Some(code) => ExitCode::from(code as u8),
            None => {
                eprintln!("Upstream auth process terminated by signal");
                ExitCode::from(1)
            }
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            ExitCode::from(1)
        }
    }
}
fn real_main(cli: Configuration) -> Result<ExitStatus, String> {
    let mut stdin_buffer = String::new();
    io::stdin()
        .lock()
        .take(STDIN_READ_LIMIT as u64 + 1)
        .read_to_string(&mut stdin_buffer)
        .map_err(|err| err.to_string())?;

    if stdin_buffer.len() > STDIN_READ_LIMIT {
        return Err(format!(
            "standard input up to {} bytes is allowed, got more",
            STDIN_READ_LIMIT
        ));
    }

    let aws_profile = match cli.command {
        Commands::Get => {
            find_aws_profile(&stdin_buffer, home_dir(), &cli).map_err(|e| e.to_string())?
        }
        _ => None,
    };

    delegate(
        &[cli.command.to_string()],
        &stdin_buffer,
        Stdio::inherit(),
        Stdio::inherit(),
        aws_profile,
        &cli,
    )
    .map_err(|err| err.to_string())
}
