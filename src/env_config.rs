use std::path::Path;

use crate::env_file::EnvFile;
use crate::env_file::EnvFiles;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EnvConfig<'config, const MAX_FILES: usize = 2> {
    pub(crate) envfiles: EnvFiles<'config, MAX_FILES>,
    pub(crate) sequence: EnvSequence,
    pub(crate) errors: ErrorReturn,
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
