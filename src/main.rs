use std::env;
use std::fs;
use std::path::Path;
fn main() {
    let curr_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            eprintln!("Error obtaining current directory: {}", err);
            return;
        }
    };
    tree(&curr_dir, 0);
}

fn tree(path: &Path, level: usize) {
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let entry_name = entry.file_name();
        let display_name = entry_name.to_string_lossy();

        print!("{}├──", " ".repeat(level));

        if entry.file_type().unwrap().is_dir() {
            let deep_path = path.join(&entry_name);
            tree(&deep_path, level + 1);
        } else {
            println!("{}",&display_name);
        }
    }
}
//TODO
//ignorisanje fajlova/foldera, vrv minimalna hesh tablica
//za ignorisanje fajlova radim .split('') I hvatam drugi element pa gledam da li postoji u hesh tablici
//ako postoji onda ignorisem
//ako ne postoji onda printam :D
//da bude malo vise "rust idiomatic"
//lepsi error handling
//isto formatiranje kao u pravom tree-u
//treba da se sortira entries
