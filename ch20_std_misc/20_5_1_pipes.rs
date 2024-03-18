// Pipes

// std::Child struct represents running child process, exposes `stdin`, `stdout` and `stderr` handles for interaction with underlying process via pipes

use std::io::prelude::*;
use std::process::Stdio;
use std::process::Command;

static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";

fn main() {
    // Spawn `wc` command
    let mut cmd = if cfg!(target_family = "windows") {
        let mut cmd = Command::new("powershell");
        cmd.arg("-Command").arg("$input | Measure-Object -Line -Word -Character");
        cmd
    } else {
        Command::new("wc")
    };
    let process = match cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
            Err(why) => panic!("couldn't spawn wc"),
            Ok(process) => process,
    };

    // Write string to `stdin` of `wc`

    // `stdin` has type `Option<ChildStdin>`, but since we know this instance must have one, can directly `unwrap` it
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why),
        Ok(_) => println!("sent pangram to wc"),
    }

    // Because `stdin` does not live after the above calls, it is `drop`ed and pipe closed
    // Very important, otherwise `wc` wouldn't start processing input just sent

    // `stdout` field also has type `Option<ChildStdout>` so must be unwrapped
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}