pub mod utils;
pub use utils::*;

#[derive(Debug)]
pub struct GitInfo {
    pub username: String,
    pub email: String,
    pub ssh_url: String,
    pub user_repo: String,
    pub current_branch: String,
}

pub fn get_git_info() -> GitInfo {
    let ssh_url = get_stdout(&run_git(vec!["config", "remote.origin.url"]));
    let username = get_stdout(&run_git(vec!["config", "user.name"]));
    let email = get_stdout(&run_git(vec!["config", "user.email"]));
    let current_branch = get_stdout(&run_git(vec!["rev-parse", "--abbrev-ref", "HEAD"]));

    GitInfo {
        user_repo: get_user_repo(&ssh_url),
        ssh_url,
        username,
        email,
        current_branch,
    }
}
