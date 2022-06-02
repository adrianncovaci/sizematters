use std::{
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
    files: Vec<(DirEntry, u64)>,
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
        let mut files = Vec::new();
        Sizer::_get_largest_n_files_rec(self.dir.clone(), &mut files)?;
        files.sort_by(|a, b| b.1.cmp(&a.1));
        files.truncate(self.files.capacity());
        self.files = files;
        Ok(())
    }

    fn _get_largest_n_files_rec(
        path: PathBuf,
        files: &mut Vec<(DirEntry, u64)>,
    ) -> Result<(), SizerError> {
        for curr_file in fs::read_dir(path)? {
            let curr_file = curr_file?;
            let path = curr_file.path();
            let metadata = fs::metadata(&path)?;
            if metadata.is_file() {
                files.push((curr_file, metadata.len()));
            } else if metadata.is_dir() {
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

    for file in sizer.files {
        println!("{:?} - {:?}", file.0.path(), file.1);
    }
    Ok(())
}
