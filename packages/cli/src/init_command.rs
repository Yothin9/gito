use crate::{user_command, utils::bug_report};

use gito_core::utils::run_git;

pub fn run(alias: &str) {
    let git_init = run_git(vec!["init"]);
    if git_init.status.success() {
        user_command::use_user::run(alias, false);
    } else {
        bug_report("git init")
    }
}
