use std::env;
use std::fs;
use std::path::Path;
const COLOR_RESET: &str = "\x1B[0m";
const COLOR_FOLDER: &str = "\x1B[1;34m";//Zelena DDDclea
const COLOR_FILE: &str = "\x1B[0;32m";//plava :DDD
fn main() {
    let curr_dir=env::current_dir().unwrap();
    tree(&curr_dir,0);
}

fn tree(path:&Path,level:usize) {
    let entries=fs::read_dir(path).unwrap();

    for entry in entries {
        let entry=entry.unwrap();        
        let entry_name= entry.file_name();
        let file_name= entry_name.to_string_lossy();
        
        print!("|");
        print!("{}", "-".repeat(level));
        
        if entry.file_type().unwrap().is_dir() {
            
            println!("{}{}{}", COLOR_FOLDER, file_name, COLOR_RESET);
            
            let sub_path = path.join(entry_name);
            tree(&sub_path, level + 1);
        } else {
            println!("{}{}{}", COLOR_FILE, file_name, COLOR_RESET);
        }
    }
}
//TODO
//ignorisanje fajlova/foldera, vrv minimalna hesh tablica
//da bude malo vise "rust idiomatic"
//permission denied ako sam u 