use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

type Index = BTreeMap<String, String>;

pub fn build_index(dir_path: Vec<&str>) {
    fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    }

    let mut index = Index::new();
    for filepath in dir_path {
        let filepath = WalkDir::new(filepath);

        let mut entries = filepath.into_iter();

        loop {
            let entry = match entries.next() {
                None => break,
                Some(Err(err)) => {
                    println!("ERROR: {}", err);
                    continue;
                }
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
        }
    }
    let index_path = "index.json";
    let index_file = File::create(index_path).unwrap();
    serde_json::to_writer_pretty(index_file, &index).unwrap();
    println!("Indexing complete");
}

pub fn search_files(buffer: String) -> Result<HashMap<String, String>, Error> {
    let index_file = File::open("index.json")?;
    let index: Index = serde_json::from_reader(index_file).expect("Should be able to read content");
    let mut search_results = HashMap::<String, String>::new();
    for (path, filename) in index.into_iter().filter(|(_, v)| v.contains(&buffer)) {
        println!("Found: {:?} at {:?}", filename, path);
        search_results.insert(path, filename);
    }
    Ok(search_results)
}

pub fn open_file(path: String) -> Result<(), Error> {
    let path = Path::new(&path);

    match open::commands(path)[0].spawn() {
        Ok(_) => {
            println!("Opened {}", path.display());
        }
        Err(err) => return Err(Error::Io(err)),
    }

    Ok(())
}
