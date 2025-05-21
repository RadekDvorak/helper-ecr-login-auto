use std::collections::HashMap;
use std::env::VarError;
use std::ffi::{OsStr, OsString};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VarErrorLike {
    /// The specified environment variable was not present in the current
    /// process's environment.
    NotPresent,

    /// The specified environment variable was found, but it did not contain
    /// valid unicode data. The found data is returned as a payload of this
    /// variant.
    NotUnicode(OsString),
}

impl From<VarError> for VarErrorLike {
    fn from(value: VarError) -> Self {
        match value {
            VarError::NotPresent => VarErrorLike::NotPresent,
            VarError::NotUnicode(x) => VarErrorLike::NotUnicode(x),
        }
    }
}

pub trait EnvLike {
    fn var<K: AsRef<OsStr>>(&self, key: K) -> Result<String, VarErrorLike>;

    fn upstream_auth_app(&self) -> String {
        self.var("ELA_UPSTREAM_AUTH_APP")
            .unwrap_or_else(|_| "docker-credential-ecr-login".to_string())
    }
    fn config_key(&self) -> Result<String, VarErrorLike> {
        self.var("ELA_ARN_CONFIG_KEY")
    }

    fn forced_profile(&self) -> Result<String, VarErrorLike> {
        self.var("ELA_FORCED_PROFILE")
    }
}

pub struct RealEnv;

impl EnvLike for RealEnv {
    fn var<K: AsRef<OsStr>>(&self, key: K) -> Result<String, VarErrorLike> {
        ::std::env::var(key).map_err(VarErrorLike::from)
    }
}

pub struct MockEnv(pub HashMap<String, String>);

impl EnvLike for MockEnv {
    fn var<K: AsRef<OsStr>>(&self, key: K) -> Result<String, VarErrorLike> {
        let k = key
            .as_ref()
            .to_str()
            .ok_or_else(|| VarErrorLike::NotUnicode(key.as_ref().to_os_string()))?;

        match self.0.get(k) {
            None => Err(VarErrorLike::NotPresent),
            Some(x) => Ok(x.clone()),
        }
    }
}
