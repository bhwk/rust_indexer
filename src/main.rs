use serde_json::Result;
use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use walkdir::{DirEntry, WalkDir};

type Index = BTreeMap<String, String>;

fn build_index(dir_path: WalkDir) -> Result<()> {
    fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }

    let mut index = Index::new();
    let mut entries = dir_path.into_iter();

    loop {
        let entry = match entries.next() {
            None => break,
            Some(Err(err)) => panic!("ERROR: {}", err),
            Some(Ok(entry)) => entry,
        };

        if entry.file_name() == "node_modules" {
            entries.skip_current_dir();
            continue;
        }

        if is_hidden(&entry) {
            if entry.file_type().is_dir() {
                entries.skip_current_dir();
            }
            continue;
        }

        let path = entry.path().display().to_string();
        let file = entry
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        println!("indexing: {:?}", &path);
        index.insert(path, file);

        let index_path = "index.json";
        let index_file = File::create(index_path).unwrap();
        serde_json::to_writer_pretty(index_file, &index)?;
    }

    Ok(())
}

fn open_file(buffer: String, index: &Index) {
    for (path, filename) in index.into_iter().filter(|(_, v)| v.contains(&buffer)) {
        println!("Found: {:?} at {:?}", filename, path)
    }
}

fn main() -> io::Result<()> {
    let index_path = "index.json";
    let index_file = File::open(index_path)?;
    let index: Index = serde_json::from_reader(index_file).expect("Should be able to read content");
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            buffer.pop();
        }

        Err(error) => {
            println!("error: {:?}", error);
        }
    }
    open_file(buffer, &index);

    Ok(())
}
