use ahash::AHashMap;

pub type Map = AHashMap<String, String>;

#[derive(Debug, Clone, PartialEq)]
pub struct EnvSafe {
    pub(crate) map: Map,
}

impl EnvSafe {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(String::as_str)
    }
}
