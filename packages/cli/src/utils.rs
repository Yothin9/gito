use std::process::{Command, Output};

pub fn run_command(program: &str, args: Vec<&str>) -> Output {
    Command::new(program).args(args).output().unwrap()
}

pub fn run_git(args: Vec<&str>) -> Output {
    run_command("git", args)
}
