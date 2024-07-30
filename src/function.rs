use crate::error::Error;
use crate::safe::EnvSafe;
use crate::safe_config::EnvSafeConfig;

pub fn load() -> Result<EnvSafe, Error> {
    EnvSafeConfig::<1>::default().load()
}
