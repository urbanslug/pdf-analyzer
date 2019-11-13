#![feature(pattern)]

use std::fs;
use std::path::Path;

mod cli;
mod search;

#[cfg(test)]
mod test {
    #[test]
    fn test_something() {
        assert_eq!(3, 3);
    }
}

fn main() {
    let cli::SearchObject {
        in_dir,
        pattern,
        out_dir,
    } = match cli::parse_cli_args() {
        Some(x) => x,
        None => return,
    };

    let dir = Path::new(&in_dir);

    let mut pdfs: Vec<fs::DirEntry> = vec![];

    if dir.is_dir() {
        for entry in fs::read_dir(in_dir).unwrap() {
            let entry = entry.unwrap();
            let b = entry.file_name().into_string().unwrap();

            if b.contains(".pdf") {
                pdfs.push(entry);
            }
        }
    }

    for pdf in pdfs {
        let p = pdf.path();
        let fp = p.as_path();
        if search::search_in_document(fp, &pattern) {
            let b = format!("{}{}", out_dir, pdf.file_name().to_str().unwrap());
            println!("{}", b);
            fs::copy(fp, b).unwrap();
        }
    }
}
