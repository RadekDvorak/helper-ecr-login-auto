use crate::cli::LogStyle::Auto;
use clap::{Parser, ValueEnum};
use env_logger::fmt::WriteStyle;
use std::fmt::Display;

#[derive(Parser)]
#[command(version, about="Docker credential helper with automatic AWS profile detection for ECR", long_about = None)]
pub struct Configuration {
    #[arg(value_enum)]
    pub command: Commands,

    #[arg(
        long = "ela-log-level",
        env = "ELA_LOG_LEVEL",
        default_value = "warn",
        help = "Set the logging level (error, warn, info, debug, trace, off)"
    )]
    pub log_level: String,

    #[arg(
        long = "ela-log-style",
        env = "ELA_LOG_STYLE",
        default_value = "auto",
        help = "Set the logging style. Use environment variable only."
    )]
    pub log_style: LogStyle,

    #[arg(
        long = "ela-upstream-auth-app",
        env = "ELA_UPSTREAM_AUTH_APP",
        default_value = "docker-credential-ecr-login",
        help = "Set the upstream credential helper application to use. Use environment variable only."
    )]
    pub upstream_auth_app: String,

    #[arg(
        long = "ela-arn-config-key",
        env = "ELA_ARN_CONFIG_KEY",
        help = "Set the config key to read the role ARNs from. Use environment variable only."
    )]
    pub arn_config_key: Option<String>,

    #[arg(
        long = "ela-forced-profile",
        hide(true),
        env = "ELA_FORCED_PROFILE",
        help = "Force using this AWS profile for all operations. Use environment variable only."
    )]
    pub forced_profile: Option<String>,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Commands {
    /// Retrieve stored credentials for a registry server
    Get,
    /// Store credentials for a registry server after a successful login
    Store,
    /// Delete stored credentials for a registry server after logout
    Erase,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            command: Commands::Get,
            log_level: "warn".to_string(),
            log_style: Auto,
            upstream_auth_app: "docker-credential-ecr-login".to_string(),
            arn_config_key: None,
            forced_profile: None,
        }
    }
}

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Commands::Get => write!(f, "get"),
            Commands::Store => write!(f, "store"),
            Commands::Erase => write!(f, "erase"),
        }
    }
}

#[derive(Clone, ValueEnum)]
pub enum LogStyle {
    Always,
    Never,
    Auto,
}

impl Display for LogStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogStyle::Always => write!(f, "always"),
            LogStyle::Never => write!(f, "never"),
            LogStyle::Auto => write!(f, "auto"),
        }
    }
}

impl From<&LogStyle> for WriteStyle {
    fn from(style: &LogStyle) -> Self {
        match style {
            LogStyle::Always => WriteStyle::Always,
            LogStyle::Never => WriteStyle::Never,
            LogStyle::Auto => WriteStyle::Auto,
        }
    }
}
