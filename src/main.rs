use std::env;
use std::fs;
use std::path::Path;
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
        
        print!("{}", "  ".repeat(level));
        
        println!("{}", file_name);
        
        if entry.file_type().unwrap().is_dir() {
            let sub_path = path.join(entry_name);
            tree(&sub_path, level + 1);
        }
    }
}
