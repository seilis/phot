use std::path::{Path, PathBuf};
use std::fs;
use std::io::Read;
use thiserror::Error;
use sha1::Sha1;

const DEFAULT_LIB_PATH: &str = "phot";

#[derive(Error, Debug)]
pub enum LibError {
    #[error("could not create library")]
    CouldNotCreate,

    #[error("io error")]
    IO(#[from] std::io::Error),

    #[error("unsupported file type")]
    UnsupportedFileType
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

    fn get_object_path(&self, hash: &str) -> PathBuf {
        let mut path = self.get_objects_dir();
        path.push(&hash[0..4]);
        path
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

    pub fn add_file(&mut self, path: &Path) -> Result<()> {
        let attrs = fs::metadata(path)?;

        if !attrs.is_file() {
            return Err(LibError::UnsupportedFileType);
        }

        let mut file = fs::File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Calculate the sha1 sum for our datastore
        let mut sum = Sha1::default();
        sum.update(&buffer);

        let sum_string = sum.digest().to_string();

        let mut lib_path = self.get_object_path(&sum_string);

        fs::create_dir_all(&lib_path)?;
        lib_path.push(&sum_string);
        fs::copy(path, lib_path)?;
        println!("success! sum: {:?}", sum.digest().to_string());

        Ok(())
    }
}
