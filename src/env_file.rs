use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnvFiles<'fpath, const MAX_FILES: usize> {
    pub(crate) paths: [&'fpath Path; MAX_FILES],
    pub(crate) overrides: [bool; MAX_FILES],
    pub(crate) len: u8,
}

impl<'fpath, const MAX_FILES: usize> EnvFiles<'fpath, MAX_FILES> {
    pub fn new() -> Self {
        Self {
            paths: [Path::new(""); MAX_FILES],
            overrides: [false; MAX_FILES],
            len: 0,
        }
    }

    pub fn add(&mut self, envfile: EnvFile<'fpath>) {
        if self.len == MAX_FILES as u8 {
            panic!("Cannot have more than {} .env files", MAX_FILES);
        }
        self.paths[self.len as usize] = envfile.path;
        self.overrides[self.len as usize] = envfile.override_existing;
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<EnvFile<'fpath>> {
        if index >= self.len as usize {
            None
        } else {
            Some(EnvFile {
                path: self.paths[index],
                override_existing: self.overrides[index],
            })
        }
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn iter(&self) -> impl Iterator<Item = EnvFile<'fpath>> + '_ {
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
pub struct EnvFile<'fpath> {
    pub(crate) path: &'fpath Path,
    pub(crate) override_existing: bool,
}
