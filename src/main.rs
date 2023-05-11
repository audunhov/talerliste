use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    innlegg: Option<Vec<String>>,

    #[arg(short, long)]
    replikk: Option<Vec<String>>,

    #[arg(short, long)]
    file: Option<PathBuf>,

    #[arg(short, long)]
    clear: bool,
}

fn main() {
    let args = Args::parse();

    let path: PathBuf = match args.file {
        Some(p) => p,
        None => ["talerliste.txt"].iter().collect(),
    };

    if args.clear {
        clear(&path);
    }

    match args.innlegg {
        Some(names) => {
            for name in names.iter() {
                write(&path, name.to_string());
            }
        }
        None => (),
    }

    match args.replikk {
        Some(names) => {
            for name in names.iter() {
                write(&path, "\t".to_owned() + name);
            }
        }
        None => (),
    }

    print(read(&path).as_str());
}

fn print(s: &str) {
    s.lines()
        .map(|line| {
            println!("{}", line.trim_end());
        })
        .collect()
}

fn read(path: &PathBuf) -> String {
    let data = match fs::read_to_string(path) {
        Ok(f) => f,
        Err(_) => {
            clear(path);
            return fs::read_to_string(path).unwrap();
        }
    };
    data
}

fn clear(path: &PathBuf) {
    fs::write(path, "---Talerlise---\n").expect("Unable to write file");
}

fn write(path: &PathBuf, line: String) {
    let file: String = read(path);
    fs::write(path, file + &line + "\n").expect("Unable to write file");
}
