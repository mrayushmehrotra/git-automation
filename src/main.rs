use names::Generator;
//use std::env::args;
use std::process::{exit, Command};

fn update_commit_push() {
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to add command");

    if !add_command.status.success() {
        eprintln!("Error: Failed to add files to git repo");
        exit(1);
    }

    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_generator())
        .output()
        .expect("Failed to execute git commit");

    if !commit_command.status.success() {
        eprintln!("Error: failed to commit changes.");
        exit(1);
    }

    // if let Some(remote) = args().nth(1) {
    //     if !remote.is_empty() {
    //         let _add_remote = Command::new("git")
    //             .arg("remote")
    //             .arg("add")
    //             .arg("origin")
    //             .arg(&remote)
    //             .spawn();
    //         match _add_remote {
    //             Ok(mut child) => {
    //                 if let Err(err) = child.wait() {
    //                     eprintln!("Error: Failed to execute command: {}", err);
    //                     exit(1);
    //                 }
    //             }
    //             Err(err) => {
    //                 eprintln!("Error: failed to spawn command: {}", err);
    //                 exit(1);
    //             }
    //         }
    //     } else {
    //         eprintln!("Error: Remote argument has errors or might be empty");
    //     }
    // } else {
    //     eprintln!("remote argument not provided, pushing code to github");
    // }
    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("Failed to do git push");
    if !push_command.status.success() {
        eprintln!("Error failed to push changes to remote, can be remote url error");
        exit(1);
    }

    println!("Successfully added, commit, and pushed changes");
}
fn name_generator() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap()
}

fn main() {
    update_commit_push();
}
