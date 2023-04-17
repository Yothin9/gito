// TODO: seprate file
pub mod add {
    use crate::constants::*;
    use ini::Ini;
    pub fn run(name: &str, email: &str, alias: &str) {
        let mut user = Ini::new();
        user.with_section(Some(alias))
            .set("name", name)
            .set("email", email);
        user.write_to_file(get_git_account_file()).unwrap();
        println!("Add {alias} successfully");
    }
}

pub mod list {
    use std::vec;

    use crate::constants::get_git_account_file;
    use ini::Ini;
    use prettytable::{row, Cell, Row, Table};
    pub fn run() {
        let mut git_account_table = Table::new();
        git_account_table.add_row(row!["alias", "name", "email"]);
        let i = Ini::load_from_file(get_git_account_file()).unwrap();
        for (sec, prop) in i.iter() {
            let mut group: Vec<Cell> = vec![];
            group.push(Cell::new(sec.unwrap_or_default()));
            for (_, v) in prop.iter() {
                group.push(Cell::new(v));
            }
            git_account_table.add_row(Row::new(group));
        }
        // TODO: mark current user
        git_account_table.printstd();
    }
}

pub mod use_user {
    use crate::constants::*;
    use crate::utils::*;
    use ini::Ini;
    pub fn run(alias: &str) {
        let git_accounts = Ini::load_from_file(get_git_account_file()).unwrap();
        if git_accounts.section(Some(alias)).is_some() {
            let account = git_accounts.section(Some(alias)).unwrap();
            let git_name = account.get("name").unwrap_or_default();
            let git_email = account.get("email").unwrap_or_default();

            run_git(vec!["config", "--local", "user.name", git_name]);
            run_git(vec!["config", "--local", "user.email", git_email]);
            println!("git account has been reset to {} {}", git_name, git_email);
        } else {
            println!("Invalid alias");
        }
    }
}
