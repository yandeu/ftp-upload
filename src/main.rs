use clap::Parser;
use round::round_down;
use std::{
    collections::BTreeMap,
    fs::{read_dir, File},
    io::BufReader,
};
use suppaftp::FtpStream;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    dir: String,
    #[arg(short, long)]
    user: String,
    #[arg(short, long)]
    password: String,
    #[arg(short = 'H', long)]
    host: String,
    #[arg(short, long)]
    silent: Option<bool>,
}

fn main() {
    let args = Args::parse();
    let silent = args.silent.unwrap_or(false);

    // check if dir ands with /*
    let mut dir = args.dir.to_string();
    let mut include_dir = true;
    if args.dir.ends_with("/*") {
        dir.pop();
        dir.pop();
        include_dir = false;
    }

    let mut all_files: Vec<String> = Vec::new();
    read_dir_recursive(&dir, &mut all_files, silent);

    let mut paths_created: BTreeMap<String, &str> = BTreeMap::new();

    let binding = args.host.split(':').collect::<Vec<&str>>();
    let _domain = &binding.first().unwrap();

    // Create a connection to an FTP server and authenticate to it.
    let mut ftp_stream = FtpStream::connect(&args.host).unwrap();
    ftp_stream.login(&args.user, &args.password).unwrap();

    if !silent {
        println!("Uploading...");
        println!("0%");
    }

    let mut percent = 0f64;
    let len = all_files.len();

    for (i, mut file) in all_files.into_iter().enumerate() {
        if (i as f64 / len as f64) * 100f64 > percent + 10f64 {
            percent = i as f64 / len as f64;
            if !silent {
                println!("{}%", round_down(percent * 100f64, 0));
            }
        }

        let f = File::open(&file).unwrap();
        let mut reader = BufReader::new(f);

        if !include_dir {
            file = file.split_at(dir.len() + 1).1.to_string();
        }

        let mut paths = file.split('/').collect::<Vec<&str>>();
        let file = paths.pop().unwrap();
        let mut full_path = "/".to_owned();

        for path in paths {
            full_path += path;

            if paths_created.get(&full_path).is_none() {
                ftp_stream.mkdir(&full_path).unwrap_or_default();
                paths_created.insert(full_path.clone(), "");
            }

            full_path += "/";
        }

        full_path += file;
        ftp_stream.put_file(&full_path, &mut reader).unwrap();
    }

    if !silent {
        println!("100%");
    }

    let _ = ftp_stream.quit();
}

fn read_dir_recursive<T>(dir: T, all_files: &mut Vec<String>, silent: bool)
where
    T: Into<String>,
{
    let paths = read_dir(dir.into()).unwrap();
    for path in paths.flatten() {
        if let Ok(file_type) = path.file_type() {
            let is_dir = file_type.is_dir();
            let file_or_dir = path.path().display().to_string();
            if is_dir {
                read_dir_recursive(file_or_dir, all_files, silent);
            } else {
                if !silent {
                    println!("FILE: {}", file_or_dir.replace('\\', "/"));
                }
                all_files.push(file_or_dir.replace('\\', "/"))
            }
        }
    }
}
