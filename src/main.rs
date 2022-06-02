use std::{
    collections::BTreeMap,
    env,
    error::Error,
    fmt,
    fs::{self, DirEntry},
    io,
    path::PathBuf,
};

use clap::Parser;

//let x = fs::metadata(path)?.len();
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 10)]
    file_number: u8,
    #[clap(short, long, default_value = "curr")]
    dir: String,
}

#[derive(Debug)]
struct Sizer {
    files: Vec<DirEntry>,
    dir: PathBuf,
}

#[derive(Debug)]
enum SizerError {
    InvalidPath,
}

impl fmt::Display for SizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sizer Error")
    }
}

impl From<io::Error> for SizerError {
    fn from(_: io::Error) -> Self {
        SizerError::InvalidPath
    }
}

impl Error for SizerError {}

impl Sizer {
    fn parse_sizer(args: Args) -> Result<Self, SizerError> {
        let files = Vec::with_capacity(args.file_number as usize);
        let dir;

        match args.dir.as_str() {
            "curr" => dir = env::current_dir()?,
            rest => dir = PathBuf::from(rest),
        }

        Ok(Self { files, dir })
    }

    fn get_largest_n_files(&mut self) -> Result<(), SizerError> {
        let mut files = BTreeMap::new();
        Sizer::_get_largest_n_files_rec(self.dir.clone(), &mut files)?;
        for (path, size) in &files {
            println!("{:?} - {:?}", path, size);
        }
        Ok(())
    }

    fn _get_largest_n_files_rec(
        path: PathBuf,
        files: &mut BTreeMap<u64, PathBuf>,
    ) -> Result<(), SizerError> {
        for curr_file in fs::read_dir(path)? {
            let curr_file = curr_file?;
            let path = curr_file.path();
            let metadata = fs::metadata(&path)?;
            if metadata.is_file() {
                files.insert(metadata.len(), curr_file.path());
            } else {
                Sizer::_get_largest_n_files_rec(curr_file.path(), files)?;
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), SizerError> {
    let args = Args::parse();

    let mut sizer = Sizer::parse_sizer(args)?;
    sizer.get_largest_n_files()?;
    Ok(())
}
