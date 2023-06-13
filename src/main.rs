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
    let mut entries: Vec<_> = fs::read_dir(path).unwrap().collect();

    entries.sort_by_key(|a| a.as_ref().unwrap().file_name());

    let entry_count = entries.len();

    for (index, entry) in entries.into_iter().enumerate() {
        let entry = entry.unwrap();
        let entry_name = entry.file_name();
        let display_name = entry_name.to_string_lossy();

        let is_last = index == entry_count - 1;
        let symbol = if is_last { "└──" } else { "├──" };
        let spacing = "|   ".repeat(level);

        print!("{}{}", spacing, symbol);

        if entry.file_type().unwrap().is_dir() {
            println!("\x1B[1;32m{}\x1B[0m", display_name);
            let deep_path = path.join(&entry_name);
            tree(&deep_path, level + 1);
        } else {
            println!("{}", &display_name);
        }
    }
}
//TODO
//ignorisanje fajlova/foldera, vrv minimalna hesh tablica
//za ignorisanje fajlova radim .split('') I hvatam drugi element pa gledam da li postoji u hesh tablici
//ako postoji onda ignorisem
//ako ne postoji onda printam :D
//isto formatiranje kao u pravom tree-u
