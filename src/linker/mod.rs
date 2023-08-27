mod link;
mod to_pathbuf;
use std::collections::HashSet;
use std::ffi::OsString;
use std::path::PathBuf;
pub trait ToPathbuf {
    fn path(&self) -> PathBuf;
    fn home_path(&self) -> PathBuf;
}
pub struct Linker {
    pub input: PathBuf,
    pub destination: PathBuf,
    pub input_inodes: HashSet<OsString>,
    pub dest_inodes: HashSet<OsString>,
}
