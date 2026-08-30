// time to use all these libraries to access file system
use std::fs;  // file system
use std::path::{Path, PathBuf};  // this one lets us borrow path strings (buffers - specifically "PathBuf"s), gotta import both the buf object and the reference type object
// thats useful for having a directory string we want to modify, "push()" to, with relative paths ("dir") to push and absolute ("/etc") to overwrite


// define a recursive subprogram function so it can call itself 
// it needs an argument of what directory to start searching on, this will be a borrowed Path
fn find_mds(dir: &Path) {

    // a file descriptor is an integer given by OS acting as a ticket, to access a file without looking up a string path like a human would
    // a directory is like a list file actually, containing its contents and inode position on disk, like a book!
    // the cursor is like a bookmark recording which PAGE youre on in the book
    // this is nice because you can read books page by page, without needing to look at the entire book at once with all pages simultaneously
    // naive idea would be to load the entire "list file" into RAM, just to read it
    // this is better because you get to read it at constant speeds, regardless how big the directory ("list file of children") is, by streaming (lazy & better!)

    // 1. open the directory.
    // if it fails (Err), return immediately (stop this function call), because we can't use this malformed dir anyways
    // if it succeeds (Ok), bind the iterator to 'entries'.
    let Ok(entries) = fs::read_dir(dir) else { return }; // silently ignore errors, only filter the Ok() Results

    // this returns a ReadDir which is a file descriptor + cursor, we now have access to this directory

    // 2. iterate over the directory
    // "raw_entry" is a Result<DirEntry, Error> from the Os
    for raw_entry in entries {

        // 3. try to extract the raw DirEntry, if its Ok(), name the contents "entry"
        // if its an err, skip this iteration and silently ignore 
        if let Ok(entry) = raw_entry {

            // if it is a directory, recurse immediately, and move onto the next entry
            if path.is_dir() {
                find_mds(&path);
                // move on, dont check further
                continue;
            }

            // if the program survives this far, we know that we have a clean DirEntry struct!
            // we want its path for its filetype so lets get it, we can do anything to "path" and "entry" now!
            let path = entry.path(); // the raw path looks like a normal literal string "/home" etc, see in debug mode by printing with {:?}
            
            if path.is_file() {
                if let Some(ext) = path.extension() { // the ext is wrapped in a Some() because thats the shape the path.extension returns, Some(value) or None
                    if ext == "md" {
                        println!("{}", path.display()); // .display() strips the "" into clean human readable paths to look at! :)
                    }
                }
            }

        }

    }
}


fn main() {
    // create a new PathBuf from a string - one with what we want to inspect 
    let directory = PathBuf::from("/"); // dont forget your semicolons :)
    find_mds(&directory); // its gotta be a reference remember?

}
