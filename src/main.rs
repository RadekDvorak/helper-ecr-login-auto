use std::env::args;
use std::io::{self, Read, Write};

use home::home_dir;

use helper_ecr_login_auto::myenv::RealEnv;
use helper_ecr_login_auto::{delegate, find_aws_profile};

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

    let environment = RealEnv {};

    let arguments: Vec<String> = args().skip(1).collect();
    let aws_profile = match arguments.first() {
        Some(v) if v == "get" => find_aws_profile(
            &stdin_buffer,
            &mut io::stderr().lock(),
            home_dir(),
            environment,
        )?,
        _ => None,
    };

    let answer = delegate(&arguments, &stdin_buffer, aws_profile)?;
    io::stdout().lock().write_all(answer.as_bytes())?;

    Ok(())
}
