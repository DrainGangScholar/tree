use std::fs::ReadDir;
use std::io;
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
    //let entries = read_entries(&path).into_iter();   
    let entries=read_entries(&path);
    let entries_iter=&entries.into_iter();
    let entry_count=&entries_iter.count();
    let mut entry_index=0;


    for(index,entry) in entries.enumerate() {
        let entry_name = entry.file_name();
        let display_name=entry_name.to_string_lossy();

        let is_last_entry= entry_index==entry_count-1;
        let branch_symbol= if is_last_entry { "└──" } else { "├──" };
        let indent = "│   ".repeat(level);

        println!("{}{}{}",indent,branch_symbol,display_name);   
        entry_index+=1;

        match entry.file_type() {
            Ok(file_type)=> {
                if file_type.is_dir(){
                    let sub_path=path.join(&entry_name);
                    tree(&sub_path,level+1);
                } else {
                    
                }
            },
            Err(err)=>{
                eprintln!("Permission denied: {}", path.display());
                Err(err);
            },
        }
    }
}
fn read_entries(path:&Path)->Result<ReadDir,io::Error>{
    match fs::read_dir(path) {
        Ok(entries) => Ok(entries),
        Err(err) => {
            eprintln!("Permission denied: {}", path.display());
            Err(err)
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
