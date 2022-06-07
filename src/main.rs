use std::{
    env,
    error::Error,
    fmt,
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    num,
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
    #[clap(short, long)]
    list: bool,
    #[clap(short, long)]
    index_delete: Option<u8>,
}

#[derive(Debug)]
struct Sizer {
    files: Vec<(PathBuf, u64)>,
    dir: PathBuf,
    file: File,
}

#[derive(Debug)]
enum SizerError {
    InvalidPath,
    InvalidSize,
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

impl From<num::ParseIntError> for SizerError {
    fn from(_: num::ParseIntError) -> Self {
        SizerError::InvalidSize
    }
}

impl Error for SizerError {}

impl Sizer {
    fn parse_sizer(args: &Args) -> Result<Self, SizerError> {
        let files = Vec::with_capacity(args.file_number as usize);
        let dir = match args.dir.as_str() {
            "curr" => env::current_dir()?,
            rest => PathBuf::from(rest),
        };

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("/tmp/sizer.log")?;

        Ok(Self { files, dir, file })
    }

    fn get_largest_n_files(&mut self) -> Result<(), SizerError> {
        let mut files = Vec::new();
        Sizer::_get_largest_n_files_rec(self.dir.clone(), &mut files)?;
        files.sort_by(|a, b| b.1.cmp(&a.1));
        files.truncate(self.files.capacity());
        self.files = files;
        self.write_list_to_log_file()?;
        for file in &self.files {
            println!("{:?} - {:?}", file.0, file.1);
        }
        Ok(())
    }

    fn _get_largest_n_files_rec(
        path: PathBuf,
        files: &mut Vec<(PathBuf, u64)>,
    ) -> Result<(), SizerError> {
        for curr_file in fs::read_dir(path)? {
            let curr_file = curr_file?;
            let path = curr_file.path();
            let metadata = fs::metadata(&path)?;
            if metadata.is_file() {
                files.push((curr_file.path(), metadata.len()));
            } else if metadata.is_dir() {
                Sizer::_get_largest_n_files_rec(curr_file.path(), files)?;
            }
        }
        Ok(())
    }

    fn write_list_to_log_file(&mut self) -> Result<(), SizerError> {
        self.file.set_len(0)?;
        self.file.seek(SeekFrom::Start(0))?;
        let mut content = String::new();
        for file in self.files.iter() {
            let (path, size) = (fs::canonicalize(file.0.clone())?, file.1);
            content.push_str(&format!("{:?} - {:?}\n", path, size));
        }
        self.file.write_all(content.as_bytes())?;
        self.file.flush()?;
        Ok(())
    }

    fn print_log_file(&mut self) -> Result<(), SizerError> {
        self.file.seek(SeekFrom::Start(0))?;
        let mut result = String::new();
        self.file.read_to_string(&mut result)?;
        Ok(())
    }

    fn delete_log_file(&self, index: u8) -> Result<(), SizerError> {
        if index == 0 || index > self.files.capacity() as u8 {
            return Ok(());
        }

        fs::remove_file(self.files[index as usize].0.clone())?;

        Ok(())
    }

    fn load_files_from_log_file(&mut self) -> Result<(), SizerError> {
        let mut result = String::new();
        self.file.read_to_string(&mut result)?;
        let mut files = vec![];
        result.truncate(result.len() - 1);

        for line in result.split('\n') {
            let data_el = line.split(" - ").collect::<Vec<&str>>();
            let size = data_el[1].parse::<u64>()?;
            let path = Path::new(&data_el[0].replace('\"', "")).to_path_buf();
            files.push((path, size));
        }

        self.files = files;

        Ok(())
    }
}

fn main() -> Result<(), SizerError> {
    let args = Args::parse();
    let mut sizer = Sizer::parse_sizer(&args)?;
    sizer.load_files_from_log_file()?;

    if args.list {
        sizer.print_log_file()?;
        return Ok(());
    }

    if let Some(index) = args.index_delete {
        sizer.delete_log_file(index)?;
        return Ok(());
    }

    sizer.get_largest_n_files()?;

    Ok(())
}
