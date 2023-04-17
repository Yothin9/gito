use crate::{constants::get_git_account_file, utils::*};
use ini::Ini;
pub fn run(alias: &str) {
    let git_accounts = Ini::load_from_file(get_git_account_file()).unwrap();
    if git_accounts.section(Some(alias)).is_some() {
        let account = git_accounts.section(Some(alias)).unwrap();
        let git_name = account.get("name").unwrap_or_default();
        let git_email = account.get("email").unwrap_or_default();

        run_git(vec![
            "commit --amend --author --no-edit",
            format!(r#"'{git_name} <{git_email}>'"#).as_str(),
        ]);
    } else {
        println!("Invalid alias");
    }
}
