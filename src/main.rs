use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf};

fn build_index(dir_path: &Path) -> Result<BTreeMap<String, PathBuf>, io::Error> {
    let mut index: BTreeMap<String, PathBuf> = BTreeMap::new();

    for entry in dir_path.read_dir().expect("read dir failed") {
        if let Ok(entry) = entry {
            let file = entry
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            let path = entry.path().to_owned();
            index.insert(file, path);
        }
    }

    Ok(index)
}

fn open_file(buffer: &String, index: &BTreeMap<String, PathBuf>) {
    let buffer = String::from(buffer);
    let filepath = index.get(&buffer);

    match filepath {
        Some(path) => {
            open::commands(path)[0]
                .spawn()
                .expect("should open application");
        }
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
    let dir_path = Path::new("test");
    let build = build_index(dir_path);
    let index = match build {
        Ok(map) => map,
        Err(error) => {
            println!("error: {:?}", error);
            panic!()
        }
    };

    println!("{:?}", index);

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
