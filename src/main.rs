// time to use all these libraries to access file system
use std::fs;  // file system
use std::path::{Path, PathBuf};  // this one lets us borrow path strings (buffers - specifically "PathBuf"s), gotta import both the buf object and the reference type object
// thats useful for having a directory string we want to modify, "push()" to, with relative paths ("dir") to push and absolute ("/etc") to overwrite


// define a recursive subprogram function so it can call itself 
// it needs an argument of what directory to start searching on, this will be a borrowed Path
fn find_mds(dir: &Path) {
    // ─── fs::read_dir(dir) ───
    // Opens the directory and returns a ReadDir iterator.
    // Under the hood: calls the OS `opendir()` / `readdir()` syscalls.
    // Returns Result<ReadDir, io::Error> because the dir might not exist
    // or you might not have permission.
    let entries = match fs::read_dir(dir) {  // this returns a ReadDir which is a file descriptor + curosr
        // a file descriptor is an integer given by OS acting as a ticket, to access a file without looking up a string path like a human would
        // a directory is like a list file actually, containing its contents and inode position on disk, like a book!
        // the cursor is like a bookmark recording which PAGE youre on in the book
        // this is nice because you can read books page by page, without needing to look at the entire book at once with all pages simultaneously
        // naive idea would be to load the entire "list file" into RAM, just to read it
        // this is better because you get to read it at constant speeds, regardless how big the directory ("list file of children") is, by streaming (lazy & better!)
        Ok(entries) => entries, // if ok (success), the match's value is "entries"
        Err(_) => return, // if there is an error (permission denied/doesnt exist) silently ignore - do nothing
    };

    // this is a cleaner way to write the above code
    // if the raw_entries looks like an Ok(), pull it out into an "entries" variable (if it lets me assign this)
    if let Ok(entries) = fs::read_dir(dir) else { return }; // silently ignore errors, only filter the Ok() Results

    println!("{:?}", entries);  // entry is io::Result<DirEntry>

    // Print each item the iterator produces
    for entry in entries {

    }   
}


fn main() {
    // create a new PathBuf from a string - one with what we want to inspect 
    let directory = PathBuf::from("/"); // dont forget your semicolons :)
    find_mds(&directory); // its gotta be a reference remember?

}
