use crate::env_config::EnvConfig;

#[derive(Debug, Clone, PartialEq)]
pub struct AppEnvConfig<'appenv, const MAX_FILES: usize = 2, const MAX_ENVS: usize = 2> {
    key: &'appenv str,
    envs: [AppEnv<'appenv, MAX_FILES>; MAX_ENVS],
    len: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct AppEnv<'appenv, const MAX_FILES: usize = 2> {
    name: &'appenv str,
    config: EnvConfig<'appenv, MAX_FILES>,
}

impl<'appenv, const MAX_FILES: usize, const MAX_ENVS: usize>
    AppEnvConfig<'appenv, MAX_FILES, MAX_ENVS>
{
    pub fn new(key: &'appenv str) -> Self {
        Self {
            key,
            envs: [AppEnv::new("", EnvConfig::new()); MAX_ENVS],
            len: 0,
        }
    }

    pub fn add_app_env(
        mut self,
        name: &'appenv str,
        config: EnvConfig<'appenv, MAX_FILES>,
    ) -> Self {
        if self.len == MAX_ENVS as u8 {
            panic!("Cannot have more than {} app envs", MAX_ENVS);
        }
        self.envs[self.len as usize] = AppEnv { name, config };
        self.len += 1;
        self
    }
}

impl<'appenv, const MAX_FILES: usize> AppEnv<'appenv, MAX_FILES> {
    fn new(name: &'appenv str, config: EnvConfig<'appenv, MAX_FILES>) -> Self {
        Self { name, config }
    }
}
