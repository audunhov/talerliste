use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    innlegg: Option<String>,

    #[arg(short, long)]
    replikk: Option<String>,

    #[arg(short, long)]
    stryk: Option<String>,

    #[arg(short, long)]
    file: Option<PathBuf>,

    #[arg(short, long)]
    neste: bool,

    #[arg(short, long)]
    clear: bool,
}

fn main() {
    let args = Args::parse();

    // file
    let path = match args.file {
        Some(path) => path.into_os_string().into_string().unwrap(),
        None => "talerliste.txt".to_string(),
    };

    if args.clear {
        clear(path.to_owned());
        write(path.to_owned(), "---Talerliste---".to_string());
    }

    match args.innlegg {
        Some(i) => write(path.to_owned(), i),
        None => (),
    }
}

fn read(path: String) -> String {
    let data = fs::read_to_string(path).expect("Unable to read file");
    data
}

fn clear(path: String) {
    fs::write(path, "").expect("Unable to write file");
}
fn write(path: String, line: String) {
    let file = read(path.to_owned());
    fs::write(path, file + &line + "\n").expect("Unable to write file");
}
