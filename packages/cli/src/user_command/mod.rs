// TODO: seprate file
pub mod add {
    use crate::{constants::*, utils::safe_get_git_account};
    pub fn run(alias: &str, name: &str, email: &str) {
        let mut config = safe_get_git_account();
        config.set_to(Some(alias), "name".to_string(), name.to_string());
        config.set_to(Some(alias), "email".to_string(), email.to_string());
        config.write_to_file(get_git_account_file()).unwrap();
        println!("Add {alias} successfully");
    }
}
pub mod del {
    use crate::{constants::*, utils::safe_get_git_account};
    pub fn run(alias: &str) {
        let mut config = safe_get_git_account();
        config.delete(Some(alias));
        config.write_to_file(get_git_account_file()).unwrap();
        println!("Delete {alias} successfully");
    }
}

pub mod list {
    use crate::utils::safe_get_git_account;
    use gito_core::GitInfo;
    use prettytable::{color, row, Attr, Cell, Row, Table};
    use std::vec;
    pub fn run(git_info: &GitInfo) {
        let mut git_account_table = Table::new();
        git_account_table.add_row(row!["alias", "name", "email"]);
        let i = safe_get_git_account();
        for (sec, prop) in i.iter() {
            let mut group: Vec<Cell> = vec![];
            group.push(Cell::new(sec.unwrap_or_default()));
            for (_, v) in prop.iter() {
                group.push(Cell::new(v));
            }

            if group[1].get_content() == git_info.username {
                for cell in group.iter_mut() {
                    cell.style(Attr::ForegroundColor(color::GREEN))
                }
            }
            git_account_table.add_row(Row::new(group));
        }
        git_account_table.printstd();
    }
}

pub mod use_user {
    use crate::utils::*;
    use gito_core::utils::run_git;

    pub fn run(alias: &str, is_global: bool) {
        let git_accounts = safe_get_git_account();
        if git_accounts.section(Some(alias)).is_some() {
            let account = git_accounts.section(Some(alias)).unwrap();
            let git_name = account.get("name").unwrap_or_default();
            let git_email = account.get("email").unwrap_or_default();

            run_git(vec!["config", "--local", "user.name", git_name]);
            run_git(vec!["config", "--local", "user.email", git_email]);
            if is_global {
                run_git(vec!["config", "--global", "user.name", git_name]);
                run_git(vec!["config", "--global", "user.email", git_email]);
            }
            println!("git account now is {} {}", git_name, git_email);
        } else {
            println!("Invalid alias");
        }
    }
}
