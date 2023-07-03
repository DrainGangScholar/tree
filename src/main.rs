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
    tree(&curr_dir, "");
}

fn tree(path: &Path, prefix: &str) {
    let mut entries: Vec<_> = fs::read_dir(path).unwrap().collect();

    entries.sort_by_key(|a| a.as_ref().unwrap().file_name());

    let entry_count = entries.len();

    for (index, entry) in entries.into_iter().enumerate() {
        let entry = entry.unwrap();
        let entry_name = entry.file_name();
        let display_name = entry_name.to_string_lossy();

        let (symbol, spacing);

        if index == entry_count - 1 {
            symbol = "└── ";
            spacing = "    ";
        } else {
            symbol = "├── ";
            spacing = "│   ";
        }

        println!("{}{}{}", prefix, symbol, &display_name);

        if entry.file_type().unwrap().is_dir() {
            let go_next = path.join(&entry_name);
            let new_prefix = format!("{}{}", prefix, spacing);
            tree(&go_next, &new_prefix);
        }
    }
}
//TODO
//ignorisanje fajlova/foldera, vrv minimalna hesh tablica
//za ignorisanje fajlova radim .split('') I hvatam drugi element pa gledam da li postoji u hesh tablici
//ako postoji onda ignorisem
//ako ne postoji onda printam :D
//isto formatiranje kao u pravom tree-u
