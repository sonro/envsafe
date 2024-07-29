#![allow(dead_code, unused_imports)]

use std::{env, fmt, io, path::Path, sync::OnceLock};

use ahash::AHashMap;

type Map = AHashMap<String, String>;

pub fn load() -> Result<EnvSafe, Error> {
    EnvSafeConfig::<1>::default().load()
}

#[derive(Debug)]
pub enum Error {
    Parse,
    Io(io::Error),
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnvSafe {
    map: Map,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnvSafeConfig<'config, const MAX_FILES: usize = 2, const MAX_ENVS: usize = 0> {
    config: EnvConfig<'config, MAX_FILES>,
    app_envs: Option<AppEnvConfig<'config, MAX_FILES, MAX_ENVS>>,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EnvConfig<'config, const MAX_FILES: usize = 2> {
    envfiles: EnvFiles<'config, MAX_FILES>,
    sequence: EnvSequence,
    errors: ErrorReturn,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum ErrorReturn {
    None,
    Parse,
    Io,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum EnvSequence {
    EnvThenFiles,
    FilesThenEnv,
    FilesThenEnvOverride,
    EnvOnly,
    FilesOnly,
}

impl Default for ErrorReturn {
    fn default() -> Self {
        Self::Parse
    }
}

impl Default for EnvSequence {
    fn default() -> Self {
        Self::EnvThenFiles
    }
}

impl EnvSafe {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(String::as_str)
    }
}

impl<'appenv, const MAX_FILES: usize> AppEnv<'appenv, MAX_FILES> {
    fn new(name: &'appenv str, config: EnvConfig<'appenv, MAX_FILES>) -> Self {
        Self { name, config }
    }
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

impl<'config, const MAX_FILES: usize> EnvConfig<'config, MAX_FILES> {
    pub fn new() -> Self {
        Self {
            envfiles: EnvFiles::new(),
            sequence: EnvSequence::default(),
            errors: ErrorReturn::default(),
        }
    }

    pub fn add_envfile<P>(mut self, path: &'config P) -> Self
    where
        P: AsRef<Path> + ?Sized,
    {
        self.envfiles.add(EnvFile {
            path: path.as_ref(),
            override_existing: false,
        });
        self
    }

    pub fn add_envfile_override<P>(mut self, path: &'config P) -> Self
    where
        P: AsRef<Path> + ?Sized,
    {
        self.envfiles.add(EnvFile {
            path: path.as_ref(),
            override_existing: true,
        });
        self
    }

    pub fn sequence(mut self, sequence: EnvSequence) -> Self {
        self.sequence = sequence;
        self
    }

    pub fn errors(mut self, errors: ErrorReturn) -> Self {
        self.errors = errors;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct EnvFiles<'fpath, const MAX_FILES: usize> {
    paths: [&'fpath Path; MAX_FILES],
    overrides: [bool; MAX_FILES],
    len: u8,
}

impl<'fpath, const MAX_FILES: usize> EnvFiles<'fpath, MAX_FILES> {
    fn new() -> Self {
        Self {
            paths: [Path::new(""); MAX_FILES],
            overrides: [false; MAX_FILES],
            len: 0,
        }
    }

    fn add(&mut self, envfile: EnvFile<'fpath>) {
        if self.len == MAX_FILES as u8 {
            panic!("Cannot have more than {} .env files", MAX_FILES);
        }
        self.paths[self.len as usize] = envfile.path;
        self.overrides[self.len as usize] = envfile.override_existing;
        self.len += 1;
    }

    fn get(&self, index: usize) -> Option<EnvFile<'fpath>> {
        if index >= self.len as usize {
            None
        } else {
            Some(EnvFile {
                path: self.paths[index],
                override_existing: self.overrides[index],
            })
        }
    }

    fn len(&self) -> usize {
        self.len as usize
    }

    fn iter(&self) -> impl Iterator<Item = EnvFile<'fpath>> + '_ {
        (0..self.len).map(move |i| self.get(i as usize).unwrap())
    }
}

impl<const MAX_FILES: usize> Default for EnvFiles<'_, MAX_FILES> {
    fn default() -> Self {
        if MAX_FILES == 0 {
            panic!("Must have at least one .env file");
        }
        let mut files = Self::new();
        files.paths[0] = Path::new(".env");
        files.len = 1;
        files
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct EnvFile<'fpath> {
    path: &'fpath Path,
    override_existing: bool,
}
