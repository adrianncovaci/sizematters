use std::{
    convert::Infallible,
    env,
    error::Error,
    fmt,
    fs::File,
    io,
    path::{Path, PathBuf},
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
    files: Vec<File>,
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
}

fn main() -> Result<(), SizerError> {
    let args = Args::parse();

    let sizer = Sizer::parse_sizer(args)?;

    println!("{:?}", sizer);

    Ok(())
}
