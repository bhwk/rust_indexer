use std::collections::BTreeMap;
use std::io;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

fn build_index(dir_path: WalkDir) -> Result<BTreeMap<String, PathBuf>, io::Error> {
    fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }

    let mut index: BTreeMap<String, PathBuf> = BTreeMap::new();
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

        let path = entry.path().to_owned();
        let file = entry
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        println!("indexing: {:?}", &path);
        index.insert(file, path);

        println!("{}", entry.path().display());
    }

    Ok(index)
}

fn open_file(buffer: &String, index: &BTreeMap<String, PathBuf>) {
    let buffer = String::from(buffer);
    let filepath = index.get(&buffer);

    match filepath {
        Some(path) => match open::commands(path)[0].spawn() {
            Ok(child) => {
                println!("Opened {:?}", path)
            }

            Err(err) => eprint!("Error: {:?}", err),
        },
        None => {
            for (filename, path) in index
                .range(buffer.to_owned()..)
                .take_while(|(k, _)| k.contains(&buffer))
            {
                println!("Found: {:?}: {:?}", filename, path)
            }
        }
    }
}

fn main() -> io::Result<()> {
    let dir_path = WalkDir::new("/root");
    let build = build_index(dir_path);
    let index = match build {
        Ok(build) => build,
        Err(error) => {
            println!("error: {:?}", error);
            panic!()
        }
    };

    for key in &index {
        println!("{:?}", key);
    }

    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            buffer.pop();
        }

        Err(error) => {
            println!("error: {:?}", error);
        }
    }

    open_file(&buffer, &index);

    Ok(())
}
