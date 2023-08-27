use super::ToPathbuf;
use std::path::PathBuf;
impl ToPathbuf for &str {
    fn path(&self) -> PathBuf {
        PathBuf::from(self)
    }
    fn home_path(&self) -> PathBuf {
        let path = format!("{}/{}", std::env::var("HOME").unwrap(), self);
        PathBuf::from(path)
    }
}
impl ToPathbuf for String {
    fn path(&self) -> PathBuf {
        PathBuf::from(self)
    }
    fn home_path(&self) -> PathBuf {
        let path = format!("{}/{}", std::env::var("HOME").unwrap(), self);
        PathBuf::from(path)
    }
}
