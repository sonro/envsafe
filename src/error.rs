use std::io;

#[derive(Debug)]
pub enum Error {
    Parse,
    Io(io::Error),
}
