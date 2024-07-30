use std::path::Path;

use crate::app_env::AppEnvConfig;
use crate::env_config::EnvConfig;
use crate::env_file::EnvFile;
use crate::error::Error;
use crate::safe::EnvSafe;
use crate::safe::Map;

#[derive(Debug, Clone, PartialEq)]
pub struct EnvSafeConfig<'config, const MAX_FILES: usize = 2, const MAX_ENVS: usize = 0> {
    pub(crate) config: EnvConfig<'config, MAX_FILES>,
    pub(crate) app_envs: Option<AppEnvConfig<'config, MAX_FILES, MAX_ENVS>>,
}

impl<'config, const MAX_FILES: usize, const MAX_ENVS: usize>
    EnvSafeConfig<'config, MAX_FILES, MAX_ENVS>
{
    pub fn new() -> Self {
        Self {
            config: EnvConfig::new(),
            app_envs: None,
        }
    }

    pub fn app_env_config(mut self, config: AppEnvConfig<'config, MAX_FILES, MAX_ENVS>) -> Self {
        self.app_envs = Some(config);
        self
    }

    pub fn add_envfile<P>(mut self, path: &'config P) -> Self
    where
        P: AsRef<Path> + ?Sized,
    {
        self.config.envfiles.add(EnvFile {
            path: path.as_ref(),
            override_existing: false,
        });
        self
    }

    pub fn load(self) -> Result<EnvSafe, Error> {
        let map = Map::new();
        let safe = EnvSafe { map };
        Ok(safe)
    }
}

impl<const MAX_FILES: usize, const MAX_ENVS: usize> Default
    for EnvSafeConfig<'_, MAX_FILES, MAX_ENVS>
{
    fn default() -> Self {
        Self {
            config: EnvConfig::default(),
            app_envs: None,
        }
    }
}
