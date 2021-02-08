use std::path::{Path, PathBuf};
use std::fs;
use thiserror::Error;

const DEFAULT_LIB_PATH: &str = "phot";

#[derive(Error, Debug)]
pub enum LibError {
    #[error("could not create library")]
    CouldNotCreate,

    #[error("io error")]
    IO(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, LibError>;

pub struct Library {
    base_path: PathBuf,
}

impl Library {
    pub fn new() -> Self {
        Library {
            base_path: Library::get_default_dir(),
        }
    }

    pub fn from_path(path: &str) -> Self {
        Library {
            base_path: PathBuf::from(path),
        }
    }

    fn get_default_dir() -> PathBuf {
        let mut dir = dirs::data_dir().expect("could not get user data directory");
        dir.push(DEFAULT_LIB_PATH);
        dir
    }

    fn get_objects_dir(&self) -> PathBuf {
        let mut dir = self.base_path.clone();
        dir.push("objects");
        dir
    }

    pub fn create(&self) -> Result<()> {
        // Make the base path first
        fs::create_dir_all(self.base_path.as_path())?;
        fs::create_dir_all(self.get_objects_dir())?;
        Ok(())
    }

    pub fn get_path(&self) -> &Path {
        self.base_path.as_path()
    }
}
