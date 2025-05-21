use std::collections::HashMap;
use std::env::VarError;
use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VarErrorLike {
    /// The specified environment variable was found, but it did not contain
    /// valid unicode data. The found data is returned as a payload of this
    /// variant.
    NotUnicode(OsString),
}

impl Display for VarErrorLike {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VarErrorLike::NotUnicode(os_str) => {
                write!(f, "environment variable is not valid unicode: {:?}", os_str)
            }
        }
    }
}

impl Error for VarErrorLike {}

pub trait EnvLike {
    fn var<K: AsRef<OsStr>>(&self, key: K) -> Option<Result<String, VarErrorLike>>;

    fn upstream_auth_app(&self) -> Result<String, VarErrorLike> {
        let x = self.var("ELA_UPSTREAM_AUTH_APP");
        match x {
            Some(Ok(x)) => Ok(x),
            Some(err) => err,
            None => Ok("docker-credential-ecr-login".to_string()),
        }
    }
    fn config_key(&self) -> Option<Result<String, VarErrorLike>> {
        self.var("ELA_ARN_CONFIG_KEY")
    }

    fn forced_profile(&self) -> Option<Result<String, VarErrorLike>> {
        self.var("ELA_FORCED_PROFILE")
    }
}

pub struct RealEnv;

impl EnvLike for RealEnv {
    fn var<K: AsRef<OsStr>>(&self, key: K) -> Option<Result<String, VarErrorLike>> {
        match ::std::env::var(key) {
            Ok(v) => Some(Ok(v)),
            Err(e) => match e {
                VarError::NotPresent => None,
                VarError::NotUnicode(contents) => Some(Err(VarErrorLike::NotUnicode(contents))),
            },
        }
    }
}

pub struct MockEnv(pub HashMap<String, String>);

impl EnvLike for MockEnv {
    fn var<K: AsRef<OsStr>>(&self, key: K) -> Option<Result<String, VarErrorLike>> {
        let c = key.as_ref().to_str();
        let k = match c {
            Some(k) => k,
            None => return Some(Err(VarErrorLike::NotUnicode(key.as_ref().to_os_string()))),
        };

        self.0.get(k).map(|x| Ok(x.clone()))
    }
}
