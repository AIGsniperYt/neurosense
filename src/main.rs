// time to use all these libraries to access file system
use std::fs;  // file system
use std::path::{Path, PathBuf};  // this one lets us borrow path strings (buffers - specifically "PathBuf"s), gotta import both the buf object and the reference type object
// thats useful for having a directory string we want to modify, "push()" to, with relative paths ("dir") to push and absolute ("/etc") to overwrite
use std::fs::DirEntry;

// define a recursive subprogram function so it can call itself 
// it needs an argument of what directory to start searching on, this will be a borrowed Path
fn find_mds(dir: &Path) {
    // ─── fs::read_dir(dir) ───
    // Opens the directory and returns a ReadDir iterator.
    // Under the hood: calls the OS `opendir()` / `readdir()` syscalls.
    // Returns Result<ReadDir, io::Error> because the dir might not exist
    // or you might not have permission.
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries, // if ok (success), the match's value is "entries"
        Err(_) => return, // if there is an error (permission denied/doesnt exist) silently ignore - do nothing
    };

/*/ Print each item the iterator produces
for entry in entries {
    println!("{:?}", entries);  // entry is io::Result<DirEntry>

    // wait im starting to understand, lesson:  "{}" displays the pretty human readable output with the Display trait, and "{:?}"" is debug trait showing raw internal names and stuff
}   
    println!("{}", dir.display());
} // */
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a new PathBuf from a string - one with what we want to inspect 
    let directory = PathBuf::from("/"); // dont forget your semicolons :)
    find_mds(&directory); // its gotta be a reference remember?
    let entries: Vec<DirEntry> = fs::read_dir(&directory)?
    .filter_map(|e| e.ok())  // drop the Err entries
    .collect();              // ← this forces the iterator to run to completion

println!("{} entries", entries.len());
for entry in entries.iter() {
println!("{:?}", entry.file_name()); 

}
Ok(())  
}
