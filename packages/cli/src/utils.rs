use ini::Ini;

use crate::constants::get_git_account_file;

pub fn bug_report(command: &str) {
    println!(
        "{command} failed, please retry or report at https://github.com/HomyeeKing/gito/issues"
    )
}

pub fn safe_get_git_account() -> Ini {
    let config = match Ini::load_from_file(get_git_account_file()) {
        Ok(c) => c,
        Err(_) => Ini::new(),
    };
    return config;
}
