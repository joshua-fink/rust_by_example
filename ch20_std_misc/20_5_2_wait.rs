// Wait - RUNTIME ERR

// If need to wait for `process::Child` to finish, must call `Child::wait` which returns `process::ExitStatus`

use std::process::Command;

fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}