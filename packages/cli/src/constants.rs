use std::path::PathBuf;

pub fn get_home_dir() -> PathBuf {
    match home::home_dir() {
        Some(path) => path,
        None => panic!("Failed to get home directory"),
    }
}

pub fn get_git_account_file() -> PathBuf {
    const ACCOUNT_FILE: &str = ".gito-account";
    get_home_dir().join(ACCOUNT_FILE)
}
