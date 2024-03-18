// Path

// Path struct represents file paths in underlying filesystem. Two flavors of Path:
// 1. `posix::Path` for UNIX-like systems, 2. windows::Path for Windows, prelude exports appropriate variant

// Path can be created from `OsStr`, provides several methods to get information from the file/directory path points to

// Path is immutable, owned version of Path is `PathBuf` -> relationship between two like `str` and `String`
// `PathBuf` can be mutated in-place, and can be dereferenced to a `Path`

// `Path` not internally represented as UTF-8 string, stored as `OsString`
// Converting `Path` to `&str` thus may fail and not free (returns `Option`)

// Path CAN be freely converted to `OsString` or `&OsStr` using `into_os_string` and `as_os_str` respectively

use std::path::Path;

fn main() {
    // Create `Path` from `&'static str`
    let path = Path::new(".");

    // `display` method returns `Display`able structure
    let _display = path.display();

    // `join` merges path with byte container using OS specific separator, returns `PathBuf`
    let mut new_path = path.join("a").join("b");

    // `push` extends `PathBuf` with `&Path`
    new_path.push("c");
    new_path.push("myfile.tar.gz");

    // `set_file_name` updates file name of `PathBuf`
    new_path.set_file_name("package.tgz");

    // Convert `PathBuf` into string slice
    match new_path.to_str() {
        None => panic!("new path not valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}